use crate::prelude::*;

const NUM_TILES: usize = 6;
const TILESIZE: f32 = 70.0;

pub const WINDOW_WIDTH: usize = 1000;
pub const WINDOW_HEIGHT: usize = 800;


pub struct Tile {
	x: f32,	// x-position on map
	y: f32,	// y-position on map
	n: i32,	// tile number	
	hex: Polygon<f32>,
	interior_color: Color,
}

impl Tile {
	pub fn render(&self) {
		draw_hexagon(
 			self.x,
 			self.y,
 			TILESIZE,
 			2.,
 			true,
 			BLACK,
 			self.interior_color, 
 		);
 		draw_text(&self.n.to_string(),self.x,self.y,20.,WHITE);
	}
	pub fn check_mouse(&self) -> bool {
		let mouse = Point::new(mouse_position().0 as f32, mouse_position().1 as f32);
		self.hex.contains(&mouse)
	}
	
	pub fn change_color(&mut self, switch: bool) {
		if switch{
			self.interior_color = GREEN;
		}
		else {
			self.interior_color = BLUE;
		}
				
	}	

}

pub struct Map{
	tiles: Vec<Tile>,	
}

impl Map {
	pub fn new()-> Self {
		let mut t_list = Vec::new();
		
		// initializing the hex polygon
		
		let s = TILESIZE/2. as f32; 
		let h = 3_f32.powf(1./2.)*s;
		let hex_a = Point::new(0.,TILESIZE);
		let hex_b = Point::new(h,s);
		let hex_c = Point::new(h,-s);
		let hex_d = Point::new(0.,-TILESIZE);
		let hex_e = Point::new(-h,-s);
		let hex_f = Point::new(-h,s);
		
		let x = (WINDOW_WIDTH/2_usize) as f32;
		let y = TILESIZE*1.5;
		let c = Point::new(x,y);
		let hex_line_string = LineString(
			vec![hex_a+c, hex_b+c, hex_c+c, hex_d+c, hex_e+c,hex_f+c, hex_a+c]
			);
		let t = Tile {
				x: x,
				y: y, 
				n: 1_i32,
				hex: Polygon(
					hex_line_string,
					[].to_vec(),
					),
				interior_color: BLUE,
			};	
		t_list.push(t);	
		
		let mut j = 1.;
		let mut m = 0.;
		for i in 1..NUM_TILES {
			if j == 1. && m > 1. {
				j = 2.;
				m = 0.;
			}
			let x = (WINDOW_WIDTH/2_usize) as f32+3_f32.powf(1./2.)*m*TILESIZE-j*3_f32.powf(1./2.)/2.*TILESIZE;
			let y = TILESIZE*(j+1.)*1.5;
			let c = Point::new(x,y);
			let hex_line_string = LineString(
			vec![hex_a+c, hex_b+c, hex_c+c, hex_d+c, hex_e+c,hex_f+c, hex_a+c]
			);
			let t = Tile {
				x: x,
				y: y, 
				n: (i+1) as i32,
				hex: Polygon(
					hex_line_string,
					[].to_vec(),
					),
				interior_color: BLUE,
			};	
			t_list.push(t);
		
			m += 1.;
		};
		Self {
			tiles: t_list,
		}		
	}
	
	pub fn render(&self){
		for i in 0..self.tiles.len() {
			self.tiles[i].render();			
		}
	}
	
	pub fn check_mouse(&mut self) -> Option<(f32,f32)> {
		let mut pointer_in_tile: Option<(f32, f32)> = None;
		for i in 0..self.tiles.len() {
			if self.tiles[i].check_mouse() {
				self.tiles[i].change_color(true);
				pointer_in_tile = Some((self.tiles[i].x, self.tiles[i].y));
			}
			else {
				self.tiles[i].change_color(false);
			} 			
		}
		pointer_in_tile
	}
}

