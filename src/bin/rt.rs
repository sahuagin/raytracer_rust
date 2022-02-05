extern crate rt_rs;
use rand::Rng;

use crate::rt_rs::raytracer::vec3::{Vec3, Color};
use crate::rt_rs::{color, HitList};
use crate::rt_rs::raytracer::sphere::Sphere;
use crate::rt_rs::raytracer::camera::Camera;
use crate::rt_rs::raytracer::materials::{Lambertian, Metal};

fn main() {
    let mut rng = rand::thread_rng();
    // if you make it 2000x1000 that's 100x, and then 100 more samples of each,
    // and then test against all the objects again for differaction. And then
    // do a depth of 50.
    let nx = 2000; // image width
    let ny = 1000; // image height
    let ns = 100;  // number of anti-aliasing samples
    const MAX_DEPTH: i32 = 50;
    let mut world = HitList::new();
    world.list.push(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5,
                                        Lambertian::new(&Color::new(0.8, 0.3, 0.3)))));
    world.list.push(Box::new(Sphere::new(&Vec3::new(0.0,-100.5, -1.0), 100.0,
                                        Lambertian::new(&Color::new(0.8, 0.8, 0.0)))));
    world.list.push(Box::new(Sphere::new(&Vec3::new(1.0,0.0,-1.0),0.5,
    // change 0.0 to 0.3 for fuzzy
                                        Metal::new(&Color::new(0.8, 0.6, 0.2), 0.3))));
    world.list.push(Box::new(Sphere::new(&Vec3::new(-1.0,0.0,-1.0), 0.5,
    // change 0.0 to 1.0 for most fuzzy
                                        Metal::new(&Color::new(0.8, 0.8, 0.8), 1.0))));
    let camera = Camera::new();
    
    println!("P3\n{} {}\n255", nx, ny);

    // we use the riter because origin is at the lower left 
    // to maintain a right handed coordinate system
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Color::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>())/ nx as f64;
                let v = (j as f64 + rng.gen::<f64>())/ ny as f64;
                let r = camera.get_ray(u, v);
                
                let _p: Vec3 = r.point_at_parameter(2.0);
                col += color(&r, &world, MAX_DEPTH);
            }
            col /= ns as f64;
            // apply some gamma correction
            col = Vec3::new( col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.999 * col.x) as i32;
            let ig = (255.999 * col.y) as i32;
            let ib = (255.999 * col.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
