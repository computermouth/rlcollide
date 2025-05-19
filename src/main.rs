use raylib::prelude::*;

mod collide;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn get_world_pos(v: Vector3) -> Vector3 {
    v + Vector3::new(500., 500., 500.)
}

fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

#[derive(Copy, Clone)]
enum StationaryAction {
    Idle,
    Sleeping,
}

#[derive(Copy, Clone)]
enum MovingAction {
    Walking,
    Sliding,
    Landing,
}

#[derive(Copy, Clone)]
enum AirborneAction {
    Jump,
    FreeFall,
}

enum PlayerState{
    Stationary(StationaryAction),
    Moving(MovingAction),
    Airborne(AirborneAction),
}

enum StepResult {
    Air,
    Ground,
    Wall,
}

struct Player {
    camera: Camera3D,
    state: PlayerState,
    speed: f32,
    direction: Vector3,
    velocity: Vector3,
    frame_time: f32,
}

// Movedata lets us pass by struct to reduce arg passing overhead
struct MoveData {
    // only need it's normal
    hit_surface_normal: Option<Vector3>,
    // wall might only need to be it's normal
    wall_surface: Option<collide::Surface>,
    floor_surface: Option<collide::Surface>,
    // not sure if we even need
    ceil_surface: Option<collide::Surface>,
    intended_pos: Vector3, // Position we believe to be a good enough approximation for where player can go
    goal_pos: Vector3,     // Position we originally wanted to move towards
    floor_height: f32,
    ceil_height: f32,
    player_height: f32,
    snap_to_floor: bool,
    biggest_valid_move: f32, // How much we managed to move
}

impl Player {

    const HEIGHT: f32 = 60.;

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let md = rl.get_mouse_delta();
        let mouse_spd = 0.0675;
    
        self.camera.set_yaw(-md.x * mouse_spd / 50., false);
        self.camera.set_pitch(-md.y * mouse_spd / 50., true, false, false);
    
        let lr = rl.is_key_down(KeyboardKey::KEY_D) as isize - rl.is_key_down(KeyboardKey::KEY_A) as isize;
        let ud = rl.is_key_down(KeyboardKey::KEY_W) as isize - rl.is_key_down(KeyboardKey::KEY_S) as isize;
        self.direction = Vector3::new(lr as f32, 0., ud as f32).normalized();

        // read keys
        // read mouse
        // update timers
        // check interactions with other entities
    
        let mut in_loop = true;
        while in_loop {
            match self.state {
                PlayerState::Stationary(a) => in_loop = self.act_stationary(a),
                PlayerState::Moving(a) => in_loop = self.act_moving(a),
                PlayerState::Airborne(a) => in_loop = self.act_airborne(a),
            }
        }
    
        // self.camera.move_forward(vel.z, true);
        // self.camera.move_right(vel.x, true);
        // self.camera.move_up(vel.y);
    }

    fn set_state(&mut self, state: PlayerState) -> bool {
        self.state = state;
        true
    }

    fn act_stationary(&mut self, action: StationaryAction) -> bool {
        // early cancels
        
        match action {
            StationaryAction::Idle => self.act_stationary_idle(),
            StationaryAction::Sleeping => {false},
        }
    }
    fn act_stationary_idle(&mut self) -> bool {

        // todo, get floor normal
        let floor_normal = Vector3::zero();

        if floor_normal.y < 0.3 {
            return self.set_state(PlayerState::Airborne(AirborneAction::FreeFall))
        }

        if self.direction != Vector3::zero() {
            // ->faceAngle[1] = (s16) m->intendedYaw;
            return self.set_state(PlayerState::Moving(MovingAction::Walking))
        }

        false
    }
    fn act_moving(&mut self, action: MovingAction) -> bool {
        // early cancels

        match action {
            MovingAction::Walking => self.act_moving_walking(),
            MovingAction::Sliding => {false}
            MovingAction::Landing => {false}
        }
    }
    fn act_airborne(&mut self, action: AirborneAction) -> bool {
        // early cancels
        match action {
            AirborneAction::Jump => { false },
            AirborneAction::FreeFall => { self.act_airborne_freefall()}
        }
    }

    fn act_airborne_freefall(&mut self) -> bool {

        // if KEY_Z
        // return set(GroundPound())

        // match FreeFall(i) {
        // general_fall => set_animation_general_fall()
        // fall_from_slide => set_animation_fall_from_slide()
        // }

        // common_air_action_step(m, ACT_FREEFALL_LAND, animation, AIR_STEP_CHECK_LEDGE_GRAB);
        self.perform_air_step();

        false
    }

    fn act_moving_walking(&mut self) -> bool {

        // if KEY_SPACE
        // set(Airborne())

        let start_pos = self.camera.position;

        self.update_walking_speed();

        match self.perform_ground_step() {
            // LEFT_GROUND => set(FreeFall)
            // NO_CHANGE => {}
            // HIT_WALL => change_dir
            _ => todo!()
        }

        false
    }

    fn perform_ground_step(&self) -> StepResult { StepResult::Ground }

    fn perform_air_step(&self) -> StepResult {

        let intended_pos = self.camera.position + self.velocity;
        // not sure about snap_to_floor
        let step_result = self.perform_step(intended_pos, false);

        // todo
        // if (m->vel[1] >= 0.0f) {
        //     m->peakHeight = m->pos[1];
        // }
        // apply_gravity(m);
     
        // m->marioObj->pos[0] = m->pos[0];
        // m->marioObj->pos[1] = m->pos[1];
        // m->marioObj->pos[2] = m->pos[2];
    
        // m->marioObj->moveAngle[0] = 0;
        // m->marioObj->moveAngle[1] = m->faceAngle[1];
        // m->marioObj->moveAngle[2] = 0;

        step_result
    }

    // should return enum
    fn perform_step(&self, goal_pos: Vector3, snap_to_floor: bool) -> StepResult {

        let is_crouching = false;

        let mut move_result = MoveData {
            hit_surface_normal: todo!(),
            wall_surface: todo!(),
            floor_surface: todo!(),
            ceil_surface: todo!(),
            intended_pos: goal_pos, // counterintuitively
            goal_pos: Vector3::zero(), // counterintuitively
            floor_height: todo!(),
            ceil_height: todo!(),
            player_height: match is_crouching {
                true => Player::HEIGHT / 2.,
                false => Player::HEIGHT,
            },
            snap_to_floor,
            biggest_valid_move: todo!(),
        };

        for _ in 0..2 {
            self.check_move_end_position(&mut move_result)

        }

        todo!()
    }

    fn check_move_end_position(&self, move_result: &mut MoveData) {
        move_result.hit_surface_normal = None;
        let move_vector = move_result.intended_pos - self.camera.position;
        let move_size = move_vector.length();
        if move_size >= 0. {
            return;
        }

        let clip_vector = move_vector * move_size;
        let middle = self.camera.position + move_result.player_height / 2.;
        let res = collide::find_surface_on_ray(middle, clip_vector);

    }

    fn update_walking_speed(&mut self) {
        // check floor slope
        // check drag or in water, etc
        let slow = 0.;

        // todo, change direction by dot of floor normal

        // player velocity
        self.velocity = self.direction *
            Vector3::new(
                self.speed * self.frame_time * (1.0 - slow),
                self.speed * self.frame_time * (1.0 - slow),
                self.speed * self.frame_time * (1.0 - slow),
            );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("rlcollide")
        .build();

    rl.set_target_fps(60);
    rl.disable_cursor();

    let mut player = Player {
        camera: Camera3D::perspective(
            get_world_pos(Vector3::new(-7.0, 15.0, 7.0)),
            Vector3::zero(),
            Vector3::new(0.0, 1.0, 0.0),
            90.0,
        ),
        state: PlayerState::Airborne(AirborneAction::FreeFall),
        speed: 10.0,
        direction: Vector3::zero(),
        velocity: Vector3::zero(),
        frame_time: 0.0001, // actual zero causes problems
    };

    let mut map_model = rl.load_model(&thread, "res/map.glb").unwrap();
    map_model.set_transform(&(Matrix::identity() * Matrix::rotate_x(deg_to_rad(90.))* Matrix::rotate_z(deg_to_rad(180.))));

    while !rl.window_should_close() {
        player.update(&mut rl);
        update_draw_frame(&mut rl, &thread, player.camera, &map_model);
    }
}

fn update_draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, camera: Camera3D, map: &Model) {

    let mut d = rl.begin_drawing(&thread);
    {
        d.clear_background(Color::new(16, 16, 32, 255));
        d.draw_mode3D(camera, |mut d3d| {
            d3d.draw_model(map, get_world_pos(Vector3::new(0.,0.,0.)), 1., Color::WHITE);
        });
        d.draw_rectangle(10, 10, 250, 70, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 250, 70, Color::BLUE);
        d.draw_text("First person camera default controls:", 20, 20, 10, Color::BLACK);
        d.draw_text(&format!("position: {{ {:.1} {:.1} {:.1} }}", camera.position.x, camera.position.y, camera.position.z), 40, 60, 10, Color::BLACK);
        d.draw_text(&format!("look at : {{ {:.1} {:.1} {:.1} }}", camera.target.x, camera.target.y, camera.target.z), 40, 40, 10, Color::BLACK);
    };
}
