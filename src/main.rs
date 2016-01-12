#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
extern crate sdl2;
extern crate fractal_generation;

use fractal_generation::*;

use sdl2::event::{Event};
use sdl2::rect::{Point};
use sdl2::pixels::{Color};
use sdl2::mouse::{Mouse};
use sdl2::render::{Renderer};

fn main() {

    ////////////////////////////////////////////////
    // Initial values and generator instantiation //
    ////////////////////////////////////////////////

    const ITERATIONS: u32 = 500;
    const RES: u32 = 400; // Meaning a square resolution of RESxRES pixels
    const THREADS: u32 = 4;

    // We start rendering at 0 + 0i with no zooming
    let mut zoom: f64 = 1.0;
    let mut center_x = 0.0;
    let mut center_y = 0.0;

    let mut generator = fractal_generation::Generator::new(RES, THREADS, ITERATIONS);

    ///////////////////////////////
    // SDL Setup scene           //
    ///////////////////////////////

    // Init SDL
    let sdl_context = sdl2::init().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    // Create SDL window
    let window = match sdl_context.video().unwrap().window("Fractal Render", RES, RES).position_centered().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    ///////////////////////////////
    // Fractals and rendering    //
    ///////////////////////////////
    
    update_renderer(&mut renderer, &mut generator, center_x, center_y, zoom);
    renderer.present();

    // Event loop on the 'event label provided by the library, keeps program running until window close (Event::Quit)
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::MouseButtonDown{timestamp, window_id, which, mouse_btn, x, y} => {
                    center_x = center_x + (x as f64 - (RES as f64 / 2.0)) / (RES as f64 / 4.0) / zoom;
                    center_y = center_y - (y as f64 - (RES as f64 / 2.0)) / (RES as f64 / 4.0) / zoom;

                    match mouse_btn {
                        Mouse::Left => zoom = zoom * 2.0,
                        Mouse::Right => zoom = zoom / 2.0,
                        _ => zoom = zoom,
                    }
                    
                    renderer.clear();
                    update_renderer(&mut renderer, &mut generator, center_x, center_y, zoom);
                    renderer.present();
                },
                Event::Quit{..}     => break 'event,
                _                   => continue
            }
        }
    }

}

fn update_renderer(renderer: &mut Renderer, generator: &mut fractal_generation::Generator, center_x: f64, center_y: f64, zoom: f64) {
   generator.set_location(center_x, center_y, zoom);
   let scene = generator.compute();
   let res = generator.get_resolution();
   for x in 0..res {
        for y in 0..res {
            let point = scene[(y*res + x) as usize];
            renderer.set_draw_color(make_color(point));            
            renderer.draw_point(Point::new(x as i32, y as i32));
        }
    }
}

fn make_color(value: u8) -> Color {
    match value {
        0 => Color::RGB(0, 0, 0),
        x => {
            // Any operation for creating colors, this is only an example
            let color = ((x % 128) * 2) as u8; 
            Color::RGB(color, color, color)
        },
    }
}