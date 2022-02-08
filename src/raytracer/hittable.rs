#[allow(unused_imports)]
use crate::raytracer::materials::{MaterialType,Material};
use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::{Point3, Vec3};
use crate::raytracer::sphere::{Sphere, MovingSphere};
use crate::raytracer::hitlist::HitList;

#[allow(unused_imports, dead_code)]
pub trait Hittable {
    fn hit(&self,
            r: &Ray,
            t_min: f64,
            t_max: f64) -> Option<HitRecord>;
    
    fn box_clone(&self) -> Box<dyn Hittable>;
}

impl Hittable for Hitters {
    fn hit(&self,
            r: &Ray,
            t_min: f64,
            t_max: f64) -> Option<HitRecord>{
        match self {
            Hitters::HitList(x) => {
                x.hit(r, t_min, t_max)
            },
            Hitters::Sphere(x) => {
                x.hit(r, t_min, t_max)
            },
            Hitters::MovingSphere(x) => {
                x.hit(r, t_min, t_min)
            },
            Hitters::Nothing(_x) => {
                None
            }
        }
    }

    fn box_clone(&self) -> Box<dyn Hittable>{
        Box::new(self.clone())
    }
}

#[macro_export]
macro_rules! wrap_hitter {
    ($klass:item, $($p:expr),*) => {
        $crate::raytracer::hittable::Hitters::$klass(
            $klass::new(
                $( $p, )*))
    }
}
    
#[derive(Clone, Copy, Default)]
pub struct NoBatter;
impl Hittable for NoBatter {
    fn hit(&self, _r: &Ray, _f_tmin:f64, _f_tmax:f64) -> Option<HitRecord>
    {
        None
    }
    
    fn box_clone(&self) -> Box<dyn Hittable>{
        Box::new(self.clone())
    }

}

// all types that implement hittable will be represented in this wrapper
#[derive(Clone)]
pub enum Hitters {
    HitList(HitList),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    Nothing(NoBatter),
}

    
#[allow(unused_imports, dead_code)]
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: MaterialType,
    pub front_face: bool,
}

#[allow(unused_imports, dead_code)]
impl HitRecord {
    pub fn new(p: Point3, t: f64, material: MaterialType) -> Self {
        HitRecord {
            p,
            normal: p,
            material,
            t,
            front_face: false,
        }
    }
    
    pub fn set_face_normal(&mut self, ray: &Ray, mut outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal *= -1.0;
            outward_normal
        };
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.box_clone()
    }
}

impl std::fmt::Display for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HitRecord: p: {}, normal: {}, material: ", self.p, self.normal)?;
        self.material.inner_fmt(f)?;
        write!(f, ", t: {}, front_face: {}",
            self.t, self.front_face)
    }
}

