mod tiles;
mod characters;

mod prelude{
	pub use macroquad::prelude::*;
	pub use geo::{Point, LineString, Polygon};
	pub use geo::algorithm::contains::Contains;
	pub use std::{thread, time};
	pub use crate::tiles::*;
	pub use crate::characters::*;
}
use prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Shotcaller".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
	
	// Borrowed fish from fish-game
	let mut whale = Whale::new();
	
	// creating the tiles-containing map
	let mut map = Map::new();
    //

 	// the loop where macroquad deploys the ticks
    loop {
    	clear_background(WHITE);
    	
    	// hexagon rendering
    	map.render();
    		
		// <writing mouse position on screen 
    	let (x, y) = mouse_position();
    	let mouse_x_and_y = format!("({}, {})", x, y);
    	draw_text(&mouse_x_and_y, 100.0, 100.0, 20.0, BLACK);
    	//		
		  
		//move sprite
		if is_mouse_button_pressed(MouseButton::Left) {
			match map.check_mouse() {
        	Some((x,y)) => whale.update_pos(x, y),
       	 	None => (),
    	}
		}
    	
		whale.render();
		    	
        next_frame().await
    }
}
