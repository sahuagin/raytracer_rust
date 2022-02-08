extern crate rt_rs;
use rand::Rng;
use std:: {
    io::{stderr, Write},
    sync::Arc,
};

use rayon::prelude::*;

use crate::rt_rs::{vect, raytracer::vec3::{Vec3, Color}};
#[allow(unused_imports)]
use crate::rt_rs::{color, write_color, random_scene};
#[allow(unused_imports)]
use crate::rt_rs::raytracer::sphere::Sphere;
use crate::rt_rs::raytracer::camera::Camera;
#[allow(unused_imports)]
use crate::rt_rs::raytracer::materials::{Lambertian, Metal, Dielectric};

fn main() {
    // For error handling
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let mut rng = rand::thread_rng();
    
    
    // if you make it 2000x1000 that's 100x, and then 100 more samples of each,
    // and then test against all the objects again for differaction. And then
    // do a depth of 50.
    // NOTE: 2000x1000 takes about 40 minutes at this point
    // only 92s as of this commit. Run in release instead of debug.
    const SAMPLES_PER_PIXEL: i32 = 500;  // number of anti-aliasing samples
    const MAX_DEPTH: i32 = 500;
    //let radian: f64 = (std::f64::consts::PI/4.0).cos();
    let vfov: f64 = 8.0;
    //let aspect: f64 = nx as f64 / ny as f64;
    const ASPECT_RATIO: f64 = 2.0/1.0; // eg 2000x1000, 800x400, 200x100
    // instead of setting the values and generating the aspect ratio,
    // we'll say how wide we want the overall image, and let the
    // ratio determine the height instead.
    const IMAGE_WIDTH: i32 = 2000; // image width
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const NUM_PIXELS: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;

    let look_from = vect!(24.0, 2.5, 5.0);
    let look_at   = vect!(3.0, 0.75, 0.75);
    let vup       = vect!(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (look_from-look_at).length();
    const APERTURE: f64 = 0.2;
    
    //let mut world = HitList::new();
    // Chapter 12, what next?
    let world = Arc::new(random_scene(&mut rng));
    // chapter 10, camera
    //world.list.push(Box::new(Sphere::new(&vect!(-radian, 0.0, -1.0), radian,
    //                Lambertian::new(&vect!(0.0, 0.0, 1.0)))));
    //world.list.push(Box::new(Sphere::new(&vect!(radian, 0.0, -1.0), radian,
    //                Lambertian::new(&vect!(1.0, 0.0, 0.0)))));

    
    // From end of chapter 9
    //world.list.push(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5,
    //                                    Lambertian::new(&Color::new(0.1, 0.2, 0.5)))));
    //world.list.push(Box::new(Sphere::new(&Vec3::new(0.0,-100.5, -1.0), 100.0,
    //                                    Lambertian::new(&Color::new(0.8, 0.8, 0.0)))));
    //world.list.push(Box::new(Sphere::new(&Vec3::new(1.0,0.0,-1.0),0.5,
    //                                    Metal::new(&Color::new(0.8, 0.6, 0.2), 0.2))));
    //world.list.push(Box::new(Sphere::new(&Vec3::new(-1.0,0.0,-1.0), 0.5,
    //                                    Dielectric::new(1.5))));
    // an interesting and easy trick with dielectric spheres is to note that if you use a 
    // negative radius, the geometry is unaffected but the surface normal
    // points inward, so it can be used as a bubble to make a hollow glass sphere.
    //world.list.push(Box::new(Sphere::new(&Vec3::new(-1.0,0.0,-1.0), -0.45, Dielectric::new(1.5))));
    let camera = Camera::new(look_from,
                        look_at,
                        vup,
                        vfov,
                        ASPECT_RATIO,
                        APERTURE,
                        dist_to_focus,
                        0.0,
                        0.0);

    
    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let n_finished = Arc::new(std::sync::atomic::AtomicI32::new(0));

    // we use the riter because origin is at the lower left 
    // to maintain a right handed coordinate system
    let pixels = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(move |j| {
            let world = world.clone();
            let n_finished = n_finished.clone();
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let mut rng = rand::thread_rng();
                let mut pixel_color = Color::default();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rng.gen::<f64>())/ (IMAGE_WIDTH-1) as f64;
                    let v = (j as f64 + rng.gen::<f64>())/ (IMAGE_HEIGHT-1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += color(&r, world.as_ref(), MAX_DEPTH);
            }
            
            let n = n_finished.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            if n % (NUM_PIXELS / 1000) == 0 || n == NUM_PIXELS - 1 {
                eprint!(
                    "\rCalculated {}/{} pixels ({:.1?}%)",
                    n + 1,
                    NUM_PIXELS,
                    (n + 1) as f64 / NUM_PIXELS as f64 * 100.0,
                );
                stderr().flush().unwrap();
            }
            
            pixel_color
        })
    })
    .flatten();
    
    let mut pixel_vec = Vec::with_capacity(NUM_PIXELS as usize);
    pixel_vec.par_extend(pixels);
    
    eprintln!();
    
    for (i, pixel_color) in pixel_vec.into_iter().enumerate() {
        if i as i32 % (NUM_PIXELS / 1000) == 0 || i as i32 == NUM_PIXELS -1 {
            let n = i * 1;
            eprint!(
                "\rWriting pixel {}/{} ({:.1?}%)",
                n,
                NUM_PIXELS,
                n as f64 / NUM_PIXELS as f64 * 100.0,
            );
            stderr().flush().unwrap();
        }
        
        write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL).unwrap_or_else(|err| {
            panic!(
                "Oops, error {} saving color {} for pixel {}/{}",
                err,
                pixel_color,
                i + 1,
                NUM_PIXELS
            )
        })
    }
    
    eprintln!();
    eprintln!("Done");
}
