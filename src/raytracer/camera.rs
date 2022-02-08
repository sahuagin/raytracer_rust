mod camera {
    #[allow(unused_imports)]
    use crate::{ray, Ray};
    #[allow(unused_imports)]
    use crate::{vect, raytracer::vec3::{Vec3}};
    use crate::{unit_vector};
    use rand::Rng;
    
    // for including an aperture to create a depth of field, we'll
    // shoot the rays from a disc around the origin rather than from
    // a single point.
    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p: Vec3;
        loop {
            p = 2.0 * vect!(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - vect!(1.0, 1.0, 0.0);
            if p.dot(&p) < 1.0 {
                break;
            }
        }
        p
    }
    
    #[derive(Clone, Copy, Debug)]
    #[allow(unused_imports, dead_code)]
    pub struct Camera {
        pub origin: Vec3,
        pub lower_left_corner: Vec3,
        pub horizontal: Vec3,
        pub vertical: Vec3,
        pub u: Vec3,
        pub v: Vec3,
        pub w: Vec3,
        pub lens_radius: f64,
        // shutter open/close times
        time0: f64,
        time1: f64,
    }
    
    impl Camera {
        // vfov is top to bottom in degrees
        #[allow(unused_imports, dead_code)]
        pub fn new(lookfrom: Vec3,
                lookat: Vec3,
                vup: Vec3,
                vfov: f64,
                aspect: f64,
                aperture: f64,
                focus_dist: f64,
                shutter_open: f64,
                shutter_close: f64,) -> Camera {
            let lens_radius = aperture / 2.0;
            let theta: f64 = vfov*std::f64::consts::PI/180.0;
            let half_height: f64 = (theta/2.0).tan();
            let half_width = aspect * half_height;
            let orig = lookfrom;
            let w = unit_vector(&(lookfrom - lookat));
            let u = unit_vector(&vup.cross(&w));
            let v = w.cross(&u);
            let llc = orig - half_width*focus_dist*u - half_height * focus_dist * v - focus_dist * w;
            let horizon = 2.0*half_width * focus_dist * u;
            let vert = 2.0*half_height * focus_dist * v;
            Camera {
                lower_left_corner: llc,
                horizontal: horizon,
                vertical: vert,
                origin: orig,
                u: u,
                v: v,
                w: w,
                lens_radius: lens_radius,
                time0: shutter_open,
                time1: shutter_close,
            }
        }
        
        #[allow(unused_imports, dead_code)]
        pub fn get_ray(&self, s: f64, t: f64) -> Ray {
            let rd: Vec3 = self.lens_radius * random_in_unit_disk();
            let offset: Vec3 = self.u * rd.x + self.v * rd.y;
            let mut rng = rand::thread_rng();
            let time = self.time0 + rng.gen::<f64>() * (self.time1-self.time0);
            ray!( &(self.origin + offset),
                &(self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset), // changed the mult order so it didn't try to dereference self
                time)
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
    let look_from = vect!(1.0, 1.0, 1.0);
    let look_at   = vect!(1.0, 1.0, -1.0);
    let vup       = vect!(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (look_from-look_at).length();
    let aperture = 2.0;
    let c = Camera::new(look_from,
                        look_at,
                        vup,
                        vfov, aspect, aperture, dist_to_focus, 0.0, 0.0);
    let llc = vect!(-2.9999999999999996, -0.9999999999999998, -1.0);
    let horizon = vect!(7.999999999999999, -0.0, 0.0);
    let vert = vect!(0.0, 3.9999999999999996, -0.0);
    
    assert_eq!(c.lower_left_corner, llc);
    assert_eq!(c.horizontal, horizon);
    assert_eq!(c.vertical, vert);
    assert_eq!(c.origin, raytracer::vec3::Vec3::new(1.0, 1.0, 1.0));
    
}

#[allow(unused_imports)]
use crate::{ray};
#[allow(unused_imports)]
use crate::raytracer::ray::Ray;
#[allow(unused_imports)]
use crate::{vect, raytracer::vec3::Vec3};
#[test]
fn test_get_ray() {
    let vfov: f64 = 90.0;
    let nx = 200;
    let ny = 100;
    let aspect: f64 = nx as f64/ny as f64;
    let look_from = vect!(1.0, 1.0, 1.0);
    let look_at   = vect!(1.0, 1.0, -1.0);
    let vup       = vect!(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (look_from-look_at).length();
    let aperture = 2.0;
    let c = Camera::new(look_from,
                        look_at,
                        vup,
                        vfov, aspect, aperture, dist_to_focus, 0.0, 0.0);
 
    let r = c.get_ray(4.0, 2.0);
    // this is now random, so it'll change every time!
    //let ans: raytracer::ray::Ray = raytracer::ray::Ray::new(&raytracer::vec3::Vec3::new(0.0, 0.0, 0.0), &raytracer::vec3::Vec3::new(14.0, 3.0, -1.0));
    let ans = ray!( &vect!(1.6712465080623153, 1.3039624788267186, 1.0 ),
                     &vect!(7.328753491937682, 5.696037521173281, -2.0 ));

    //Ray { a: Vec3 { x: 1.1474845657837764, y: 0.5490526987309197, z: 1.0 }, b: Vec3 { x: 27.85251543421622, y: 6.450947301269079, z: -2.0 } }
    assert_ne!(r, ans);
}