use super::aabb::{AabbF, AABB};
use super::hittable::{HitRecord, Hittable};
use super::materials::MaterialType;
use super::ray::Ray;
use super::util::{ffmax, ffmin, uv_for_sphere};
use super::vec3::{dot, Vec3};
use super::vect;
use crate::prelude::BoundingBox;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    #[allow(dead_code)]
    material: MaterialType,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(c: &Vec3, r: f64, material: MaterialType) -> Self {
        Sphere {
            center: *c,
            radius: r,
            material,
        }
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sphere: center: {} radius: {}, material: {}",
            &self.center, &self.radius, &self.material
        )
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let oc = r.origin() - self.center;
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        let disc_sq = discriminant.sqrt();
        if discriminant > 0.0 {
            let mut temp = (-b - disc_sq) / a;
            if temp < t_max && temp > t_min {
                let pat = r.point_at_parameter(temp);
                rec.replace(HitRecord {
                    t: temp,
                    p: pat,
                    normal: (pat - self.center) / self.radius,
                    front_face: false,
                    material: self.material.clone(),
                    // the u,v coord computation assumes that it has
                    // a unit sphere centered on the origin. So, we'll
                    // do that. Note, it's the same as the "normal"
                    // above.
                    texture_coord: Some(uv_for_sphere(&((pat - self.center) / self.radius))),
                });
                return rec;
            }
            temp = (-b + disc_sq) / a;
            if temp < t_max && temp > t_min {
                let pat = r.point_at_parameter(temp);
                rec.replace(HitRecord {
                    t: temp,
                    p: pat,
                    normal: (pat - self.center) / self.radius,
                    front_face: false,
                    material: self.material.clone(),
                    // the u,v coord computation assumes that it has
                    // a unit sphere centered on the origin. So, we'll
                    // do that.
                    texture_coord: Some(uv_for_sphere(&((pat - self.center) / self.radius))),
                });
                return rec;
            }
        }
        rec
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::AabbF(AabbF::new(
            self.center - vect!(self.radius, self.radius, self.radius),
            self.center + vect!(self.radius, self.radius, self.radius),
        )))
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[allow(unused_imports, dead_code)]
#[derive(Default, Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    material: MaterialType,
}

impl MovingSphere {
    #[allow(dead_code)]
    pub fn new(cen0: Vec3, cen1: Vec3, t0: f64, t1: f64, r: f64, material: MaterialType) -> Self {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            material,
        }
    }

    pub fn center_at_time(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MovingSphere: center0: {} center1: {} time0: {} time1: {} radius: {}, material: {}",
            &self.center0, &self.center1, &self.time0, &self.time1, &self.radius, &self.material
        )
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut retrec: Option<HitRecord> = None;
        let oc = r.origin() - self.center_at_time(r.time());
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let disc_sq = discriminant.sqrt();
            let mut temp = (-b - disc_sq) / a;
            if temp < t_max && temp > t_min {
                let pat = r.point_at_parameter(temp);
                retrec.replace(HitRecord {
                    t: temp,
                    p: pat,
                    normal: (pat - self.center_at_time(r.time())) / self.radius,
                    material: self.material.clone(),
                    texture_coord: Some(uv_for_sphere(&pat)),
                    front_face: false,
                });
                return retrec;
            }
            temp = (-b + disc_sq) / a;
            if temp < t_max && temp > t_min {
                let pat = r.point_at_parameter(temp);
                retrec.replace(HitRecord {
                    t: temp,
                    p: pat,
                    normal: (pat - self.center_at_time(r.time())) / self.radius,
                    material: self.material.clone(),
                    texture_coord: Some(uv_for_sphere(&pat)),
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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        // if the object isn't active during the interval requested, just return none
        let min_time = f64::max(t0, self.time0);
        let max_time = f64::min(t1, self.time1);
        if max_time - min_time < 0. {
            return None;
        }

        let center_t0 = self.center_at_time(min_time);
        let center_t1 = self.center_at_time(max_time);

        let box0 = BoundingBox::AabbF(AabbF::new(
            center_t0 - vect!(self.radius, self.radius, self.radius),
            center_t0 + vect!(self.radius, self.radius, self.radius),
        ));
        let box1 = BoundingBox::AabbF(AabbF::new(
            center_t1 - vect!(self.radius, self.radius, self.radius),
            center_t1 + vect!(self.radius, self.radius, self.radius),
        ));
        let small = Vec3::new(
            ffmin(box0.min().x, box1.min().x),
            ffmin(box0.min().y, box1.min().y),
            ffmin(box0.min().z, box1.min().z),
        );
        let big = Vec3::new(
            ffmin(box0.max().x, box1.max().x),
            ffmax(box0.max().y, box1.max().y),
            ffmax(box0.max().z, box1.max().z),
        );

        Some(BoundingBox::AabbF(AabbF::new(small, big)))
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use crate::color_to_texture;
    use crate::hittable::HitRecord;
    use crate::hittable::Hittable;
    #[allow(unused_imports)]
    use crate::materials::{Lambertian, MaterialType};
    #[allow(unused_imports)]
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    #[allow(unused_imports)]
    use crate::textures::ConstantTexture;
    use crate::util;
    #[allow(unused_imports)]
    use crate::vec3::{Color, Point3, Vec3};

    #[test]
    fn test_sphere_hit() {
        let pt1 = Point3::new(0.0, 0.0, 0.0);
        let pt2 = Point3::new(1.0, 1.0, 1.0);
        let l = MaterialType::Lambertian(Lambertian::new(&color_to_texture!(&Color::new(
            0.1, 0.8, 0.1
        ))));
        let r = Ray::new(&pt1, &pt2, None);
        let center = Point3::new(2.0, 2.0, 2.0);
        let radius = 3.0;
        let s = Sphere::new(&center, radius, l.clone());
        let pat = Vec3 {
            x: 0.26794919243112264,
            y: 0.26794919243112264,
            z: 0.26794919243112264,
        };
        let hitrec = HitRecord {
            t: 0.26794919243112264,
            p: pat,
            normal: Vec3 {
                x: -0.5773502691896258,
                y: -0.5773502691896258,
                z: -0.5773502691896258,
            },
            front_face: false,
            texture_coord: Some(util::uv_for_sphere(&pat)),
            material: l,
        };

        // this should have 2 hits, but we'll return the closest one
        if let Some(result) = s.hit(&r, 0.0, 4.0) {
            println!("the result front_face: {}", result.front_face);
            assert_eq!(result.t, hitrec.t);
            println!("the result : {}", result);
        }
        //println!("the result front_face: {}", result.material);
    }
}
