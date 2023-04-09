use raylib::prelude::*;

struct Entity {
    x: i32,
    y: i32,
    speed: i32,
}

impl Entity {
    fn new(x: i32, y: i32, speed: i32) -> Self {
        Self { x, y, speed }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Hello, World")
        .build();
    rl.set_target_fps(60);

    let player = Entity::new(100, 200, 50);

    while !rl.window_should_close() {
        let deltatime: f32 = rl.get_frame_time() * 100.0;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // stuff here
        d.draw_rectangle(player.x, player.y, 12, 12, Color::RED);

        //player.update(30.0);
    }
}

