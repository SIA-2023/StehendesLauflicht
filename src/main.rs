use ggez::{Context, ContextBuilder};
use ggez::graphics::{self, Color};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};

use std::time::Instant;

mod checkerboard;
use checkerboard::Checkerboard;

mod car;
use car::Car;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Stehendes Lauflicht Visualizierung", "Peanutt42")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0).resizable(true))
        .window_setup(WindowSetup::default().title("Stehendes Lauflicht Visualizierung"))
        .build()
        .expect("could not create ggez context!");

    let visualizierung = Visualization::new(&mut ctx);
    event::run(ctx, event_loop, visualizierung);
}

struct Visualization {
    last_update_time: Instant,
    car: Car,
    checkerboard: Checkerboard,
}

impl Visualization {
    fn new(ctx: &mut Context) -> Self {
        Self {
            last_update_time: Instant::now(),
            car: Car::new(ctx),
            checkerboard: Checkerboard::new(
                Color::BLACK,
                Color::WHITE,
                Color::new(0.35, 0.35, 0.35, 1.0),
                Color::new(0.85, 0.85, 0.85, 1.0),
                120.0
            ),
        }
    }
}

impl EventHandler for Visualization {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let now = Instant::now();
        let delta_time = (now - self.last_update_time).as_secs_f32();
        self.last_update_time = now;

        self.car.update(ctx, delta_time);
        
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        self.checkerboard.draw(ctx, &mut canvas);
        self.car.draw(&self.checkerboard, ctx, &mut canvas);

        canvas.finish(ctx)
    }
}