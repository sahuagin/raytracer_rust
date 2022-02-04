extern crate rt_rs;

use crate::rt_rs::raytracer::vec3::Vec3;
use crate::rt_rs::raytracer::ray::Ray;
use crate::rt_rs::{color, HitList};
use crate::rt_rs::raytracer::sphere::Sphere;

fn main() {
    let nx = 200; // image width
    let ny = 100; // image height
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut world = HitList::new();
    world.list.push(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.list.push(Box::new(Sphere::new(&Vec3::new(0.0,-100.5, -1.0), 100.0)));
    
    println!("P3\n{} {}\n255", nx, ny);

    // we use the riter because origin is at the lower left 
    // to maintain a right handed coordinate system
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(&origin, &(lower_left_corner + u*horizontal + v*vertical));
            
            let _p: Vec3 = r.point_at_parameter(2.0);
            let col = color(&r, &world);

            let ir = (255.999 * col.x) as i32;
            let ig = (255.999 * col.y) as i32;
            let ib = (255.999 * col.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
