mod sphere {

    //use crate::{Hitable, Ray, HitRecord, HitWrapper};
    use crate::{Hitable, Ray, HitRecord};
    use crate::raytracer::vec3::{Vec3, dot};

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Sphere {
        center: Vec3,
        radius: f64,
        //wrapper: crate::HitWrapper,
    }
    
    impl Sphere{
        #[allow(dead_code)]
        pub fn new(c: &Vec3, r: f64) -> Sphere {
            Sphere {
                center: *c,
                radius: r,
                //wrapper: Sphere: crate::raytracer::sphere::Sphere,
            }
        }
        
    }
    
    impl Hitable for Sphere {
        fn hit(&self, r: &Ray,
            t_min: f64,
            t_max: f64) -> Option<HitRecord> {
                let mut rec: Option<HitRecord> = None;
                let oc = r.origin() - self.center;
                let a = dot(&r.direction(), &r.direction());
                let b = dot(&oc, &r.direction());
                let c = dot(&oc, &oc) - self.radius*self.radius;
                let discriminant = b*b - a*c;
                if discriminant > 0.0 {
                    let mut temp = (-b - (b*b-a*c).sqrt())/a;
                    if temp < t_max && temp > t_min {
                        let pat = r.point_at_parameter(temp);
                        rec.replace(HitRecord {
                            t: temp,
                            p: pat,
                            normal: (pat - self.center) / self.radius,
                        });
                        return rec;
                    }
                    temp = (-b + (b*b-a*c).sqrt())/a;
                    if temp < t_max && temp > t_min {
                        let pat = r.point_at_parameter(temp);
                        rec.replace( HitRecord{
                            t: temp,
                            p: pat,
                            normal: (pat - self.center) / self.radius,
                        });
                        return rec;
                    }
                }
                return rec;
            }
    }
}

pub use sphere::Sphere;

#[cfg(test)]

#[allow(unused_imports)]
use crate::raytracer::ray::{Ray};
#[allow(unused_imports)]
use crate::raytracer::vec3::{Vec3, Point3};
#[allow(unused_imports)]
use crate::HitRecord;
#[allow(unused_imports)]
use crate::Hitable;

#[test]
fn test_sphere_hit(){
    let pt1 = Point3::new(0.0,0.0,0.0);
    let pt2 = Point3::new(1.0,1.0,1.0);
    let r   = Ray::new(&pt1, &pt2);
    let center = Point3::new(2.0, 2.0, 2.0);
    let radius = 3.0;
    let s   = sphere::Sphere::new(&center, radius);
    let hitrec = Some(HitRecord { t: 0.26794919243112264,
                                p: Vec3 { x: 0.26794919243112264,
                                        y: 0.26794919243112264,
                                        z: 0.26794919243112264 },
                                normal: Vec3 { x: -0.5773502691896258,
                                    y: -0.5773502691896258,
                                    z: -0.5773502691896258
                                }});

    // this should have 2 hits, but we'll return the closest one
    assert_eq!(s.hit(&r, 0.0, 4.0), hitrec);
    println!("the hitrec i: {:?}", &hitrec);
}