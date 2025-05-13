use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn get_world_pos(v: Vector3) -> Vector3 {
    v + Vector3::new(500., 500., 500.)
}

fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

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

enum AirborneAction {
    Jump,
    FreeFall,
}

enum PlayerState{
    Stationary(StationaryAction),
    Moving(MovingAction),
    Airborne(AirborneAction),
}

struct Player {
    camera: Camera3D,
    state: PlayerState,
    speed: f32,
    direction: Vector3,
    velocity: Vector3,
    frame_time: f32,
}

impl Player {
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
                PlayerState::Stationary(_) => in_loop = self.act_stationary(),
                PlayerState::Moving(a) => in_loop = self.act_moving(a),
                PlayerState::Airborne(_) => in_loop = self.act_airborne(),
            }
        }
    
        // self.camera.move_forward(vel.z, true);
        // self.camera.move_right(vel.x, true);
        // self.camera.move_up(vel.y);
    }

    fn act_stationary(&mut self) -> bool {false}
    fn act_moving(&mut self, action: MovingAction) -> bool {
        // early cancels

        match action {
            MovingAction::Walking => self.act_walking(),
            MovingAction::Sliding => {false}
            MovingAction::Landing => {false}
        }
    }
    fn act_airborne(&mut self) -> bool {false}

    fn act_walking(&mut self) -> bool {

        // if change action
        // return true;

        let start_pos = self.camera.position;

        self.update_walking_speed();

        match self.perform_ground_step() {
            _ => false
        }

    }

    fn perform_ground_step(&self) -> bool { false }

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
        state: PlayerState::Stationary(StationaryAction::Idle),
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
