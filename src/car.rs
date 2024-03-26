use ggez::Context;
use ggez::graphics::{Canvas,Mesh, DrawMode, Color, Image, DrawParam};
use ggez::winit::event::VirtualKeyCode;
use ggez::glam::Vec2;

use crate::Checkerboard;

// angle in radians
fn rotated_by(v: Vec2, angle: f32) -> Vec2 {
	let cos = f32::cos(angle);
    let sin = f32::sin(angle);

    Vec2::new(cos * v.x - sin * v.y, sin * v.x + cos * v.y)
}

pub struct Car {
	pub position: Vec2,
	pub orientation: f32,
	light_offsets: [Vec2; 24],

	texture: Image,
}
const MOVE_SPEED: f32 = 150.0;
const WHEEL_DISTANCE: f32 = 120.0;

impl Car {
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			position: Vec2::new(0.0, 0.0),
			orientation: 0.0,
			light_offsets: [
				// vorne
				Vec2::new(60.0, -60.0),
				Vec2::new(36.6, -60.0),
				Vec2::new(13.3, -60.0),
				Vec2::new(-13.3, -60.0),
				Vec2::new(-36.6, -60.0),
				Vec2::new(-60.0, -60.0),
		
				// hinten
				Vec2::new(60.0, 60.0),
				Vec2::new(36.6, 60.0),
				Vec2::new(13.3, 60.0),
				Vec2::new(-13.3, 60.0),
				Vec2::new(-36.6, 60.0),
				Vec2::new(-60.0, 60.0),
		
				// links
				Vec2::new(-60.0, 60.0),
				Vec2::new(-60.0, 36.6),
				Vec2::new(-60.0, 13.3),
				Vec2::new(-60.0, -13.3),
				Vec2::new(-60.0, -36.6),
				Vec2::new(-60.0, -60.0),
		
				// rechts
				Vec2::new(60.0, 60.0),
				Vec2::new(60.0, 36.6),
				Vec2::new(60.0, 13.3),
				Vec2::new(60.0, -13.3),
				Vec2::new(60.0, -36.6),
				Vec2::new(60.0, -60.0),
			],

			texture: Image::from_bytes(ctx, include_bytes!("../images/car.png")).unwrap(),
		}
	}

	pub fn update(&mut self, ctx: &mut Context, delta_time: f32) {
		let mut left_motor = 0.0;
		let mut right_motor = 0.0;

		if ctx.keyboard.is_key_pressed(VirtualKeyCode::W) {
			left_motor = MOVE_SPEED;
			right_motor = MOVE_SPEED;
		}
		if ctx.keyboard.is_key_pressed(VirtualKeyCode::S) {
			left_motor = -MOVE_SPEED;
			right_motor = -MOVE_SPEED;
		}
		if ctx.keyboard.is_key_pressed(VirtualKeyCode::A) {
			left_motor = 0.0;
		}
		if ctx.keyboard.is_key_pressed(VirtualKeyCode::D) {
			right_motor = 0.0;
		}

		let delta_orientation = ((left_motor - right_motor) / WHEEL_DISTANCE) * delta_time;
		// sprite rotate radians(delta_orientation)
		self.orientation += delta_orientation;
		let velocity = (left_motor + right_motor) / 2.0;
		self.position += Vec2::new(f32::cos(self.orientation), f32::sin(self.orientation)) * velocity * delta_time;
	}

	pub fn draw(&mut self, checkerboard: &Checkerboard, ctx: &mut Context, canvas: &mut Canvas) {
		canvas.draw(&self.texture,
			DrawParam::new()
			.dest(self.position)
			.rotation(self.orientation)
			.offset([0.5, 0.5])
			.scale([WHEEL_DISTANCE / self.texture.width() as f32, WHEEL_DISTANCE / self.texture.height() as f32])
		);


		let light_circle = Mesh::new_circle(ctx, DrawMode::fill(), [0.0, 0.0], 7.5, 1.0, Color::WHITE).unwrap();
		for light_offset in &self.light_offsets {
			let position = self.position + rotated_by(*light_offset, self.orientation);

			canvas.draw(&light_circle, DrawParam::new()
				.dest(position)
				.color(checkerboard.get_color(position, false))
			);
		}
	}
}