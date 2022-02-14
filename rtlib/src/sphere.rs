use super::hittable::{Hittable, HitRecord};
use super::vec3::{Vec3, dot};
use super::materials::{MaterialType};
use super::ray::Ray;


#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    #[allow(dead_code)]
    material: MaterialType,
}

impl Sphere{
    #[allow(dead_code)]
    pub fn new(c: &Vec3, r: f64, material: MaterialType) -> Self {
        Sphere {
            center: *c,
            radius: r,
            material: material,
        }
    }
    
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray,
        t_min: f64,
        t_max: f64) -> Option<HitRecord> {
            let mut rec: Option<HitRecord> = None;
            let oc = r.origin() - self.center;
            let a = dot(&r.direction(), &r.direction());
            let b = dot(&oc, &r.direction());
            let c = dot(&oc, &oc) - self.radius*self.radius;
            let discriminant = b*b - a*c;
            let disc_sq = discriminant.sqrt();
            if discriminant > 0.0 {
                let mut temp = (-b - disc_sq)/a;
                if temp < t_max && temp > t_min {
let pat = r.point_at_parameter(temp);
                    rec.replace(HitRecord {
                        t: temp,
                        p: pat,
                        normal: (pat - self.center) / self.radius,
                        front_face: false,
                        material: self.material,
                    });
                    return rec;
                }
                temp = (-b + disc_sq)/a;
                if temp < t_max && temp > t_min {
                    let pat = r.point_at_parameter(temp);
                    rec.replace( HitRecord{
                        t: temp,
                        p: pat,
                        normal: (pat - self.center) / self.radius,
                        front_face: false,
                        material: self.material,
                    });
                    return rec;
                }
            }
            return rec;
        }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

#[allow(unused_imports, dead_code)]
#[derive(Default, Clone, Copy)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    material: MaterialType,
}

impl MovingSphere{
    #[allow(dead_code)]
    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        t0: f64,
        t1: f64,
        r: f64,
        material: MaterialType,
    ) -> Self {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            material: material,
        }
    }
     
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0))
            * (self.center1 -self.center0)
    }
}


impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut retrec: Option<HitRecord> = None;
        let oc = r.origin() - self.center(r.time());
        let a  = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let disc_sq = discriminant.sqrt();
            let mut temp = (-b - disc_sq)/a;
            if temp < t_max && temp > t_min {
                    let p = r.point_at_parameter(temp);
                retrec.replace(HitRecord{
                    t: temp,
                    p: p,
                    normal: (p - self.center(r.time()))/self.radius,
                    material: self.material,
                    front_face: false,
                });
                return retrec;
            }
            temp = (-b + disc_sq)/a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                retrec.replace(HitRecord{
                    t: temp,
                    p: p,
                    normal: (p - self.center(r.time()))/self.radius,
                    material: self.material,
                    front_face: false,
                });
                return retrec;
            }
        }
        retrec
    }
    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod test {
#[allow(unused_imports)]
use crate::ray::{Ray};
#[allow(unused_imports)]
use crate::vec3::{Point3,Color,Vec3};
#[allow(unused_imports)]
use crate::materials::{Lambertian, MaterialType};
#[allow(unused_imports)]
use crate::color_to_texture;
#[allow(unused_imports)]
use crate::textures::ConstantTexture;
use crate::sphere::Sphere;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;

    #[test]
    fn test_sphere_hit(){
        let pt1 = Point3::new(0.0,0.0,0.0);
        let pt2 = Point3::new(1.0,1.0,1.0);
        let l   = MaterialType::Lambertian(Lambertian::new( &color_to_texture!(&Color::new(0.1, 0.8, 0.1))));
        let r   = Ray::new(&pt1, &pt2, None);
        let center = Point3::new(2.0, 2.0, 2.0);
        let radius = 3.0;
        let s   = Sphere::new(
            &center, radius, l);
        let hitrec = HitRecord { t: 0.26794919243112264,
                                    p: Vec3 { x: 0.26794919243112264,
                                            y: 0.26794919243112264,
                                            z: 0.26794919243112264 },
                                    normal: Vec3 { x: -0.5773502691896258,
                                        y: -0.5773502691896258,
                                        z: -0.5773502691896258 },
                                    front_face: false,
                                    material: l,
                                    };

        // this should have 2 hits, but we'll return the closest one
        if let Some(result) = s.hit(&r, 0.0, 4.0){
            println!("the result front_face: {}", result.front_face );
            assert_eq!(result.t, hitrec.t);
            println!("the result front_face: {}", result);
        }
        //println!("the result front_face: {}", result.material);
    }
}
