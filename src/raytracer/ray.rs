mod ray
{
    use crate::raytracer::vec3::Vec3;
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Ray {
        pub a: Vec3,
        pub b: Vec3,
    }
    
    impl Ray {
        pub fn new(a: &Vec3, b: &Vec3) -> Self {
            Self{
                a: *a,
                b: *b,
            }
        }
        
        pub fn origin(&self) -> Vec3 {
            self.a
        }
        
        pub fn direction(&self) -> Vec3 {
            self.b
        }
        
        pub fn point_at_parameter(&self, t: f64) -> Vec3 {
            self.a + t*self.b
        }
    }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ray2{
    ($pt1: expr, $pt2: expr) => {
        Ray::new($pt1, $pt2)
    }
}

#[allow(unused_imports)]
//pub(crate) use ray2;

#[cfg(test)]
use crate::raytracer::vec3::Vec3;
pub use ray::Ray;

#[test]
fn test_constructor() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(7.3, 9.8, 10.2);
    let r = Ray::new(&a,&b);
    
    assert_eq!(r.a, a);
    assert_eq!(r.b, b);
}

#[test]
fn test_origin() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(77.7, 88.8, 12.34);
    let r = ray::Ray::new(&v, &v2);
 
    assert_eq!(r.origin(), v);
}

#[test]
fn test_direction() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(77.7, 88.8, 12.34);
    let r = ray::Ray::new(&v, &v2);
 
    assert_eq!(r.direction(), v2);

}

#[test]
fn test_point_at_parameter() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(77.7, 88.8, 12.34);
    let r = ray::Ray::new(&v, &v2);
    let p = 30.0;
 
    assert_eq!(r.point_at_parameter(p), r.a + r.b*p );
    assert_eq!(r.point_at_parameter(p), r.a + p*r.b );
    
}