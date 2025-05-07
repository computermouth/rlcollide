use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn get_world_pos(v: Vector3) -> Vector3 {
    v + Vector3::new(500., 500., 500.)
}

fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("rlcollide")
        .build();

    rl.set_target_fps(60);
    rl.disable_cursor();

    let mut camera = Camera3D::perspective(
        get_world_pos(Vector3::new(-7.0, 15.0, 7.0)),
        Vector3::zero(),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
    );

    let mut map_model = rl.load_model(&thread, "res/map.glb").unwrap();
    map_model.set_transform(&(Matrix::identity() * Matrix::rotate_x(deg_to_rad(90.))* Matrix::rotate_z(deg_to_rad(180.))));

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);
        // update_player();
        update_draw_frame(&mut rl, &thread, &mut camera, &map_model);
    }
}

fn update_draw_frame(rl: &mut RaylibHandle, thread: &RaylibThread, camera: &mut Camera3D, map: &Model) {

    let mut d = rl.begin_drawing(&thread);
    {
        d.clear_background(Color::new(16, 16, 32, 255));
        d.draw_mode3D(*camera, |mut d3d, _camera| {
            d3d.draw_model(map, get_world_pos(Vector3::new(0.,0.,0.)), 1., Color::WHITE);
        });
        d.draw_rectangle(10, 10, 250, 70, Color::SKYBLUE);
        d.draw_rectangle_lines(10, 10, 250, 70, Color::BLUE);
        d.draw_text("First person camera default controls:", 20, 20, 10, Color::BLACK);
        d.draw_text(&format!("position: {{ {:.1} {:.1} {:.1} }}", camera.position.x, camera.position.y, camera.position.z), 40, 60, 10, Color::BLACK);
        d.draw_text(&format!("look at : {{ {:.1} {:.1} {:.1} }}", camera.target.x, camera.target.y, camera.target.z), 40, 40, 10, Color::BLACK);
    };
}
