use raylib::prelude::*;

struct Bird {
    x: i32,
    y: i32,
    speed: i32,
}

impl Bird {
    fn new(x: i32, y: i32, speed: i32) -> Self {
        Self { x, y, speed }
    }

    fn update(&mut self, delta_time: f32) {
        if self.y + 32 < 350 {
            self.y += (self.speed as f32 * delta_time) as i32;
        }
    }
}

struct Ground {
    x: i32,
    y: i32,
}

impl Ground {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Pipe {
    x: i32,
    y: i32,
}

impl Pipe {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Hello, World")
        .build();
    rl.set_target_fps(60);

    let mut bird = Bird::new(100, 200, 50);
    let mut ground = Ground::new(0, 400);

    while !rl.window_should_close() {
        let deltatime: f32 = rl.get_frame_time() * 2.0;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        bird.update(deltatime);


        d.draw_rectangle(bird.x, bird.y, 50, 32, Color::RED);

        d.draw_rectangle(0, 350, 5000, 3, Color::RED);

        //player.update(30.0);
    }
}

