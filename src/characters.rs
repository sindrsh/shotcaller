use crate::prelude::*;


pub struct Whale {
	x: f32,
	y: f32,
	img: Texture2D,
}

impl Whale {
	pub fn new() -> Self {
		Self {
			x: 10.,
			y: 10.,
			img: Texture2D::from_file_with_format(
    			include_bytes!("../assets/whale.png"), Some(ImageFormat::Png)),
		}			
	}
	
	pub fn render(&self) {
		draw_texture(self.img, self.x, self.y, WHITE);
	}	
	
	pub fn update_pos(&mut self, x: f32, y: f32) {
		self.x = x-25.;
		self.y = y-25.;	
	}
}

