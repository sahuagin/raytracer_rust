#[allow(unused_imports)]
use super::hittable::{HitRecord, Hittable, Hitters, TextureCoord};
use super::ray::Ray;
use crate::prelude;
use prelude::BoundingBox;

#[allow(unused_imports, dead_code)]
#[derive(Default, Clone)]
pub struct HitList {
    // we could also call this 'objects' but where's the fun in that?
    pub list: Vec<Hitters>,
}

impl<'a> HitList {
    //pub fn push(&mut self, obj: dyn Hittable) {
    //    self.hitlist.push(Box::new(obj));
    //}
    pub fn new() -> HitList {
        HitList { list: Vec::new() }
    }

    pub fn from_hittable(object: Hitters) -> HitList {
        let mut list = HitList { list: Vec::new() };
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }

    pub fn add(&mut self, object: Hitters) {
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

    // bounding_box for a HitList returns a bounding box that would contain
    // all of the objects it contains.
    fn bounding_box(&self, t_min: f64, t_max: f64) -> Option<BoundingBox> {
        // figure out the most extreme bounds of everything
        // under this one and return the extremities
        let mut temp_rec: Option<BoundingBox> = None;
        for obj in &self.list {
            let rec = obj.bounding_box(t_min, t_max);
            let bigger = BoundingBox::expand_to_contain(temp_rec, rec);
            temp_rec.replace(bigger.unwrap_or_default());
        }
        temp_rec
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
