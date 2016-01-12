#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
use num::complex::Complex;
use std::ops::{Mul};
use eventual::*;

pub fn mandelbrot_point(x: f64, y: f64, iterations: u32) -> u8 {        
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    
    for i in 0..iterations {
        z = z.mul(z) + c;
        if z.norm_sqr() >= 4.0 {
            return (i % 255) as u8;
        }
    }
    0
}

pub fn simple_mandelbrot_scene(iterations: u32, res: u32) -> Vec<u8> {
    let mut fractal: Vec<u8> = Vec::with_capacity((res * res) as usize);
    
    // Adapt our points in a scene resolution (pixels) to points in the domain (complex numbers as f64)
    for i in 0..res {
        for j in 0..res {
            let x = (j as f64 - (res as f64 / 2.0)) / (res as f64 / 4.0);
            let y = (i as f64 - (res as f64 / 2.0)) / (res as f64 / 4.0);
            fractal.push(mandelbrot_point(x, y, iterations));
        }
    }
    fractal
}

pub fn mandelbrot_scene(iterations: u32, res: u32, threads: u32, center_x: f64, center_y: f64, zoom: f64) -> Vec<u8> {

    // This is the scene that will be returned
    // Partial computations results will be added orderly to this vector
    let mut scene: Vec<u8> = Vec::with_capacity((res * res) as usize);
    let mut thread_pool = Vec::new();
    
    for process in 0..threads {
        let future = Future::spawn(move || {
            let mut fractal: Vec<u8> = match (process % threads) {
                0 => Vec::with_capacity((res + res % threads) as usize),
                _ => Vec::with_capacity(res as usize),
            };
            
            // Partition of rows in thread_pool
            // init_range and end_range define a portion to be processed
            // Pattern matching used for last process should it need to accomodate extra rows
            let init_range = process * res / threads;
            let end_range = match (process % threads) {
                0 => init_range + res / threads + res % threads,
                _ => init_range + res / threads,
            };

            for i in init_range..end_range {
                for j in 0..res {
                    let x = center_x + ((j as f64 - (res as f64 / 2.0)) / (res as f64 / 4.0) / zoom);
                    let y = center_y - ((i as f64 - (res as f64 / 2.0)) / (res as f64 / 4.0) / zoom);
                    fractal.push(mandelbrot_point(x, y, iterations));
                }
            }
            println!("Thread {} returning", process);
            fractal
        });
        thread_pool.push(future);
    }

    // Recover the results orderly and append them to the scene array
    // 'thread_pool' has moved into the iterator 'it', so 'thread_pool' cannot be used again (into_iter iterates over T).
    // The iterator elements are consumed orderly in this loop, appending portions to the final scene.
    let mut it = thread_pool.into_iter();
    loop {
        match it.next() {
            Some(future) => { 
                let mut res: Vec<u8> = future.and_then(|v| {
                    Ok(v)
                }).await().unwrap();
                scene.append(&mut res);
                //println!("Process result appended");
            },
            None => break,
        }
    }
    scene    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_figure() {
        assert_eq!(mandelbrot_point(0.0, 0.0, 100), 0);
    }

    #[test]
    fn value_in_domain() {
        assert!(mandelbrot_point(1.5, 0.0, 100) > 0);
    }

    #[test]
    fn out_of_domain() {
        assert_eq!(mandelbrot_point(2.4, 2.4, 100), 0);
    }

    #[test]
    fn simple_scene_results() {
        let scene = simple_mandelbrot_scene(100, 100);
        assert_eq!(scene[0], 0);
        assert_eq!(scene[9999], 0);
        assert!(scene[3150] > 0);
    }

    #[test]
    fn scene_results() {
        let scene = mandelbrot_scene(100, 100, 1, 0.0, 0.0, 1.0);
        assert_eq!(scene[0], 0);
        assert_eq!(scene[9999], 0);
        assert!(scene[3150] > 0);
    }


}