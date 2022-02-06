extern crate rt_rs;
use rand::Rng;

use crate::rt_rs::{vect, raytracer::vec3::{Vec3, Color}};
use crate::rt_rs::{color, HitList};
use crate::rt_rs::raytracer::sphere::Sphere;
use crate::rt_rs::raytracer::camera::Camera;
#[allow(unused_imports)]
use crate::rt_rs::raytracer::materials::{Lambertian, Metal, Dielectric};

fn main() {
    let mut rng = rand::thread_rng();
    // if you make it 2000x1000 that's 100x, and then 100 more samples of each,
    // and then test against all the objects again for differaction. And then
    // do a depth of 50.
    // NOTE: 2000x1000 takes about 40 minutes at this point
    // only 92s as of this commit. Run in release instead of debug.
    let nx = 2000; // image width
    let ny = 1000; // image height
    let ns = 100;  // number of anti-aliasing samples
    const MAX_DEPTH: i32 = 50;
    //let radian: f64 = (std::f64::consts::PI/4.0).cos();
    let vfov: f64 = 20.0;
    let aspect: f64 = nx as f64 / ny as f64;
    let look_from = vect!(-2.0, 2.0, 1.0);
    let look_at   = vect!(0.0, 0.0, -1.0);
    let vup       = vect!(0.0, 1.0, 0.0);
    
    let mut world = HitList::new();
    // chapter 10, camera
    //world.list.push(Box::new(Sphere::new(&vect!(-radian, 0.0, -1.0), radian,
    //                Lambertian::new(&vect!(0.0, 0.0, 1.0)))));
    //world.list.push(Box::new(Sphere::new(&vect!(radian, 0.0, -1.0), radian,
    //                Lambertian::new(&vect!(1.0, 0.0, 0.0)))));

    
    // From end of chapter 9
    world.list.push(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5,
                                        Lambertian::new(&Color::new(0.1, 0.2, 0.5)))));
    world.list.push(Box::new(Sphere::new(&Vec3::new(0.0,-100.5, -1.0), 100.0,
                                        Lambertian::new(&Color::new(0.8, 0.8, 0.0)))));
    world.list.push(Box::new(Sphere::new(&Vec3::new(1.0,0.0,-1.0),0.5,
                                        Metal::new(&Color::new(0.8, 0.6, 0.2), 0.2))));
    world.list.push(Box::new(Sphere::new(&Vec3::new(-1.0,0.0,-1.0), 0.5,
                                        Dielectric::new(1.5))));
    // an interesting and easy trick with dielectric spheres is to note that if you use a 
    // negative radius, the geometry is unaffected but the surface normal
    // points inward, so it can be used as a bubble to make a hollow glass sphere.
    //world.list.push(Box::new(Sphere::new(&Vec3::new(-1.0,0.0,-1.0), -0.45, Dielectric::new(1.5))));
    let camera = Camera::new(look_from,
                        look_at,
                        vup,
                        vfov, aspect);

    
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
