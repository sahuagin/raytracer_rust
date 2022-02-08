#[allow(unused_imports)]
use crate::raytracer::hittable::{Hittable, HitRecord, Hitters};
use crate::raytracer::ray::Ray;

#[allow(unused_imports, dead_code)]
#[derive(Default, Clone)]
pub struct HitList {
    // we could also call this 'objects' but where's the fun in that?
    pub list: Vec<Hitters>,
}

impl HitList {
    //pub fn push(&mut self, obj: dyn Hittable) {
    //    self.hitlist.push(Box::new(obj));
    //}
    pub fn new() -> HitList {
        HitList {
            list: Vec::new(),
        }
    }
    
    pub fn from_hittable(object: Hitters) -> HitList {
        let mut list = HitList {
            list: Vec::new(),
        };
        list.add(object);
        list
    }
    
    pub fn clear(&mut self) {
        self.list.clear()
    }
    
    pub fn add(&mut self, object: Hitters){
        self.list.push(object)
    }
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;
        for obj in &self.list {
            if let Some(rec) = obj.hit(&r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec.replace(rec);
            }
        }
        temp_rec
    }
    
    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
    
}

//#[allow(unused_macros)]
//#[macro_export]
//macro_rules! box_clone_for_type{
//    ($klass:ty) => {
//        fn box_clone(&self) -> Box<dyn $klass> {
//            Box::new((*self).clone())
//        }
//    };
//}

