#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
extern crate num;
extern crate eventual;

pub mod mandelbrot;

pub struct Generator {
    center_x: f64,
    center_y: f64,
    zoom: f64,
    res: u32,
    threads: u32,
    iterations: u32
}

impl Generator {
    pub fn new(res: u32, threads: u32, iterations: u32) -> Generator {
        Generator {
            center_x : 0.0,
            center_y : 0.0,
            zoom : 1.0,
            res: res,
            threads : threads,
            iterations: iterations
        }
    }

    fn print_parameters(&mut self) {
        println!("");
        println!("x: {}, y: {}, zoom: {}", self.center_x, self.center_y, self.zoom);
    }

    pub fn set_location(&mut self, center_x: f64, center_y: f64, zoom: f64) {
        self.center_x = center_x;
        self.center_y = center_y;
        self.zoom = zoom;
    }

    pub fn compute(&mut self) -> Vec<u8>{
        self.print_parameters();
        mandelbrot::mandelbrot_scene(self.iterations, self.res, self.threads, self.center_x, self.center_y, self.zoom)
    }

    pub fn get_resolution(&mut self) -> u32 {
        self.res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_is_generated_and_is_correct() {
        let iterations: u32 = 100;
        let res: u32 = 100; // Meaning a square resolution of RESxRES pixels
        let threads: u32 = 1;
        let zoom: f64 = 1.0;
        let center_x = 0.0;
        let center_y = 0.0;

        let mut generator = Generator::new(res, threads, iterations);
        generator.set_location(center_x, center_y, zoom);
        let scene = generator.compute();
        
        assert_eq!(scene[0], 0);
        assert_eq!(scene[9999], 0);
        assert!(scene[3150] > 0);
    }

    #[test]
    fn scene_is_generated_and_is_correct_with_multithreading() {
        let iterations: u32 = 100;
        let res: u32 = 100; // Meaning a square resolution of RESxRES pixels
        let threads: u32 = 8;
        let zoom: f64 = 1.0;
        let center_x = 0.0;
        let center_y = 0.0;

        let mut generator = Generator::new(res, threads, iterations);
        generator.set_location(center_x, center_y, zoom);
        let scene = generator.compute();
        
        assert_eq!(scene[0], 0);
        assert_eq!(scene[9999], 0);
        assert!(scene[3150] > 0);
    }
}