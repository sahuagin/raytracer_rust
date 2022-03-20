use crate::{
    aabb::{AABB, AabbF, BoundingBox},
    hittable::{Hittable, HitRecord},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct TranslateHitable {
    instance: Box<dyn Hittable>,
    offset: Vec3,
}

impl TranslateHitable {
    pub fn new(instance: &Box<dyn Hittable>, offset: &Vec3) -> TranslateHitable {
        TranslateHitable{
        instance: instance.box_clone(),
        offset: offset.clone(),
        }
    }
}

impl Hittable for TranslateHitable {
   fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
       let moved_ray = Ray::new(&(r.origin() - self.offset), &r.direction(), Some(r.time()));
       match self.instance.hit(&moved_ray, t_min, t_max) {
            Some(mut gothit) => {gothit.p += self.offset; Some(gothit)},
            None => None,
       }
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        match self.instance.bounding_box(t0, t1) {
            Some(bb) => {Some(BoundingBox::AabbF(AabbF::new(
                            bb.min() + self.offset,
                            bb.max() + self.offset,
                            )))},
            None => None,
        }
    }
 
}

#[cfg(test)]
mod test {
    use super::TranslateHitable;
    use crate::{
        aabb::{AabbF, BoundingBox},
        cube::Cube,
        hittable::{Hittable, HitRecord, TextureCoord},
        materials::{DiffuseLight, MaterialType},
        ray::Ray,
        textures::{TextureType, ConstantTexture},
        vect,
    };

    #[test]
    fn test_translate_cube() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let cube_initial = Cube::new(
            &p0,
            &p1,
            &MaterialType::default());

        let t_offset = vect!(2, 3, 4);
        let translated_cube = TranslateHitable::new(
            &cube_initial.box_clone(),
            &t_offset,
            );

        let t_bb = translated_cube.bounding_box(0.0, 1.0);
        let ans_bb = Some(BoundingBox::AabbF(AabbF::new(p0 + t_offset, p1 + t_offset)));

        assert_eq!(t_bb.unwrap(), ans_bb.unwrap());


    }

    #[test]
    fn test_translated_cube_hit() {
        let p0 = vect!(-1,-1,-1);
        let p1 = vect!(1,1,1);
        let c0 = Cube::new(
            &p0,
            &p1,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let t_offset = vect!(2,3,4);
        let t_c0 = TranslateHitable::new(
            &c0.box_clone(),
            &t_offset,
            );

        // from the front
        let r = Ray::new(
            //origin gets translated
            &(vect!(0, 0, 2) + t_offset),
            // direction stays the same
            &vect!(0, 0, -1),
            // give it enough length/time? that it hits
            Some(10.0)
            );

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 1) + t_offset,
            1.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        hr_ans.normal = vect!(0,0,1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 0.5});

        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));

        // from the back
        let r = Ray::new(
            &(vect!(0, 0, -2) + t_offset),
            &vect!(0, 0, 1),
            Some(10.0)
            );

        hr_ans.normal = vect!(0, 0, -1);
        hr_ans.p = vect!(0, 0, -1) + t_offset;

        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 

        // from the right side
        let r = Ray::new(
            &(vect!(2, 0, 0) + t_offset),
            &vect!(-1, 0, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(1, 0, 0);
        hr_ans.p = vect!(1, 0, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the left side
        let r = Ray::new(
            &(vect!(-2, 0, 0) + t_offset),
            &vect!(1, 0, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(-1, 0, 0);
        hr_ans.p = vect!(-1, 0, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the top
        let r = Ray::new(
            &(vect!(0, 2, 0) + t_offset),
            &vect!(0, -1, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(0, 1, 0);
        hr_ans.p = vect!(0, 1, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 
        // from the bottom
        let r = Ray::new(
            &(vect!(0, -2, 0) + t_offset),
            &vect!(0, 1, 0),
            Some(10.0)
            );

        hr_ans.normal = vect!(0, -1, 0);
        hr_ans.p = vect!(0, -1, 0) + t_offset;
        let hr = t_c0.hit(&r, 0.0, 1.0);
        assert_eq!(hr, Some(hr_ans.clone()));
 

    }
}
