mod camera {
    #[allow(unused_imports)]
    use crate::{ray2, Ray};
    use crate::{vect, raytracer::vec3::{Vec3}};
    
    #[allow(unused_imports, dead_code)]
    pub struct Camera {
        pub origin: Vec3,
        pub lower_left_corner: Vec3,
        pub horizontal: Vec3,
        pub vertical: Vec3,
    }
    
    impl Camera {
        // vfov is top to bottom in degrees
        #[allow(unused_imports, dead_code)]
        pub fn new(vfov: f64, aspect: f64) -> Camera {
            let theta: f64 = vfov*std::f64::consts::PI/180.0;
            let half_height: f64 = (theta/2.0).tan();
            let half_width = aspect * half_height;
            let llc = Vec3::new(-half_width, -half_height, -1.0);
            let horizon = Vec3::new(2.0*half_width, 0.0, 0.0);
            let vert = Vec3::new(0.0, 2.0*half_height, 0.0);
            let orig = vect!(0.0, 0.0, 0.0);
            Camera {
                lower_left_corner: llc,
                horizontal: horizon,
                vertical: vert,
                origin: orig,
            }
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn get_ray(&self, u: f64, v: f64) -> Ray {
            Ray {
                a: self.origin,
                b: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin, // changed the mult order so it didn't try to dereference self
            }
        }
    }
}

pub use camera::Camera;

#[cfg(test)]
pub use crate::raytracer;

#[test]
fn test_camera_new() {
    let vfov: f64 = 90.0;
    let nx = 200;
    let ny = 100;
    let aspect: f64 = nx as f64/ny as f64;
    let c = Camera::new(vfov, aspect);
    let llc = Vec3::new(-1.9999999999999998, -0.9999999999999999, -1.0 );
    let horizon = Vec3::new(3.9999999999999996, 0.0, 0.0 );
    let vert = Vec3::new(0.0, 1.9999999999999998, 0.0 );
    
    assert_eq!(c.lower_left_corner, llc);
    assert_eq!(c.horizontal, horizon);
    assert_eq!(c.vertical, vert);
    assert_eq!(c.origin, raytracer::vec3::Vec3::new(0.0, 0.0, 0.0));
    
}

#[allow(unused_imports)]
use crate::{ray2};
#[allow(unused_imports)]
use crate::raytracer::ray::Ray;
#[allow(unused_imports)]
use crate::raytracer::vec3::Vec3;
#[test]
fn test_get_ray() {
    let vfov: f64 = 90.0;
    let nx = 200;
    let ny = 100;
    let aspect: f64 = nx as f64/ny as f64;
    let c = Camera::new(vfov, aspect);
 
    let r = c.get_ray(4.0, 2.0);
    //let ans: raytracer::ray::Ray = raytracer::ray::Ray::new(&raytracer::vec3::Vec3::new(0.0, 0.0, 0.0), &raytracer::vec3::Vec3::new(14.0, 3.0, -1.0));
    let ans = ray2!(&Vec3::new(0.0, 0.0, 0.0 ),
                    &Vec3::new(13.999999999999998, 2.9999999999999996, -1.0));
    assert_eq!(r, ans);
}