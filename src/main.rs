use raylib::prelude::*;

struct Bird {
    x: i32,
    y: i32,
    speed: i32,
    width: i32,
    height: i32,
    speed_min: i32,
    speed_max: i32,
}

impl Bird {
    fn new(x: i32, y: i32, speed: i32, width: i32, height: i32, speed_min: i32, speed_max: i32) -> Self {
        Self { x, y, speed, width, height, speed_min, speed_max }
    }

    fn update(&mut self, delta_time: f32) {
        if self.y + 32 < 350 {
            self.y += (self.speed as f32 * delta_time) as i32;
        }

        self.speed = self.speed.clamp(self.speed_min, self.speed_max);

        self.speed += 10;
    }

    fn center(&mut self) -> Vector2 {
        Vector2::new((self.width / 2) as f32, (self.height / 2) as f32)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(800, 450).title("Hello, World")
        .build();
    rl.set_target_fps(60);

    let mut bird = Bird::new(300, 100, 50, 50, 32, -250, 350);

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

        //d.draw_rectangle(bird.x, bird.y, bird.width, bird.height, Color::RED);
        let rect = Rectangle::new(bird.x as f32, bird.y as f32, bird.width as f32, bird.height as f32);
        d.draw_rectangle_pro(rect, bird.center(), bird.speed as f32 / 10.0, Color::RED);

        d.draw_rectangle(0, 350, 5000, 3, Color::RED);
    }
}

