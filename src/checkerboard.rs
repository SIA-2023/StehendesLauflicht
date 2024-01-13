use ggez::Context;
use ggez::graphics::{Canvas, Color, DrawParam, Quad, Rect};
use ggez::glam::Vec2;

pub struct Checkerboard {
	primary: Color,
	secondary: Color,
	scale: f32,

	debug_primary: Color,
	debug_secondary: Color,
}

impl Checkerboard {
	pub fn new(primary: Color, secondary: Color, debug_primary: Color, debug_secondary: Color, scale: f32) -> Self {
		Self {
            primary,
            secondary,
            scale,
			debug_primary,
            debug_secondary,
        }
    }

	pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
		let (width, height) = ctx.gfx.size();

		for y in 0..(height / self.scale) as i32 + 1 {
			for x in 0..(width / self.scale) as i32 + 1 {
				canvas.draw(&Quad, DrawParam::new().dest_rect(Rect::new(x as f32 * self.scale, y as f32 * self.scale, self.scale, self.scale)).color(self.get_color(Vec2::new(x as f32, y as f32) * self.scale, true)))
			}
		}
	}

	pub fn get_color(&self, position: Vec2, debug: bool) -> Color {
		let scaled_pos = position / self.scale;
		if debug {
			if (scaled_pos.x as i32 + scaled_pos.y as i32) % 2 == 0 { self.debug_primary } else { self.debug_secondary }
		}
		else {
			if (scaled_pos.x as i32 + scaled_pos.y as i32) % 2 == 0 { self.primary } else {self.secondary}
		}
	}
}