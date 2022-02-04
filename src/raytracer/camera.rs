mod camera {
    use crate::{Ray};
    use crate::raytracer::vec3::{Vec3};
    
    #[allow(unused_imports, dead_code)]
    pub struct Camera {
        pub origin: Vec3,
        pub lower_left_corner: Vec3,
        pub horizontal: Vec3,
        pub vertical: Vec3,
    }
    
    impl Camera {
        #[allow(unused_imports, dead_code)]
        pub fn new() -> Camera {
            Camera {
                lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
                horizontal: Vec3::new(4.0, 0.0, 0.0),
                vertical: Vec3::new(0.0, 2.0, 0.0),
                origin: Vec3::new(0.0, 0.0, 0.0),
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
    let c = Camera::new();
    
    assert_eq!(c.lower_left_corner, raytracer::vec3::Vec3::new(-2.0, -1.0, -1.0));
    assert_eq!(c.horizontal, raytracer::vec3::Vec3::new(4.0, 0.0, 0.0));
    assert_eq!(c.vertical, raytracer::vec3::Vec3::new(0.0, 2.0, 0.0));
    assert_eq!(c.origin, raytracer::vec3::Vec3::new(0.0, 0.0, 0.0));
    
}

#[test]
fn test_get_ray() {
    let c = Camera::new();

    let r = c.get_ray(4.0, 2.0);
    let ans: raytracer::ray::Ray = raytracer::ray::Ray::new(&raytracer::vec3::Vec3::new(0.0, 0.0, 0.0), &raytracer::vec3::Vec3::new(14.0, 3.0, -1.0));
    assert_eq!(r, ans);
}