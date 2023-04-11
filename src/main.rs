use raylib::prelude::*;

struct Bird {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
    speed_min: f32,
    speed_max: f32,
}

impl Bird {
    fn new(x: f32, y: f32, speed: f32, width: f32, height: f32, speed_min: f32, speed_max: f32) -> Self {
        Self { x, y, speed, width, height, speed_min, speed_max }
    }

    fn update(&mut self, delta_time: f32) {
        if self.y + 32.0 < 350.0 {
            self.y += self.speed * delta_time;
        }

        self.speed = self.speed.clamp(self.speed_min, self.speed_max);
        self.speed += 10.0;
    }

    fn center(&mut self) -> Vector2 {
        Vector2::new(self.width / 2.0, self.height / 2.0)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Hello, World")
        .build();
    rl.set_target_fps(60);

    let mut bird = Bird::new(300.0, 100.0, 50.0, 50.0, 32.0, -250.0, 350.0);

    while !rl.window_should_close() {
        let deltatime: f32 = rl.get_frame_time() * 2.0;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            println!("speed was: {}", bird.speed);
            bird.speed = bird.speed_min;
            println!("speed is now: {}", bird.speed);
        }

        bird.update(deltatime);

        let rect = Rectangle::new(bird.x, bird.y, bird.width, bird.height);
        d.draw_rectangle_pro(rect, bird.center(), bird.speed / 10.0, Color::RED);

        d.draw_rectangle(0, 350, 5000, 3, Color::RED);
    }
}

