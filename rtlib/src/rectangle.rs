use super::{
    aabb::{AabbF, BoundingBox},
    hittable::{HitRecord, Hittable, TextureCoord},
    materials::{MaterialType, NoneMaterial},
    ray::Ray,
    vect,
};

#[derive(Clone)]
pub struct XYRect {
    material: MaterialType,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}


impl Default for XYRect {
    fn default() -> XYRect {
        XYRect {
            material: MaterialType::Nothing(NoneMaterial),
            x0: f64::default(),
            x1: f64::default(),
            y0: f64::default(),
            y1: f64::default(),
            k:  f64::default(),
        }
    }
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: &MaterialType) -> Self {
        XYRect {
            material: mat.clone(),
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.origin().z) / r.direction().z;
        if t < tmin || t > tmax {
            return None;
        }
        let x: f64 = r.origin().x + t * r.direction().x;
        let y: f64 = r.origin().y + t * r.direction().y;
        if (x < self.x0) || (x > self.x1) ||
            (y < self.y0) || (y > self.y1) {
            return None;
        }

        let mut hitrec = HitRecord::new(
            r.point_at_parameter(t),
            t,
            self.material.clone()
            );

        hitrec.texture_coord = 
            Some(TextureCoord{
                u: (x - self.x0) / (self.x1 - self.x0),
                v: (y - self.y0) / (self.y1 - self.y0),
            });
        // axis aligned rectangle.
        hitrec.normal = vect!(0, 0, 1);
        Some(hitrec)
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<BoundingBox> {
        Some(BoundingBox::AabbF(
            AabbF{
                minimum: vect!(self.x0, self.y0, self.k - 0.0001),
                maximum: vect!(self.x1, self.y1, self.k + 0.0001)}
        ))
    }
}

#[cfg(test)]
mod test {
use crate::{
    aabb::{AabbF, BoundingBox},
    hittable::{Hittable, HitRecord, TextureCoord},
    materials::{DiffuseLight, MaterialType},
    ray::Ray,
    textures::{ConstantTexture, TextureType},
    vect,
};
use super::XYRect;

    #[test]
    fn test_xyrect_creation() {
        let xyrect = XYRect::new(
            -1.0, 1.0, -1.0, 1.0, 0.0,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let bb_ans = BoundingBox::AabbF(AabbF::new(
            vect!(-1, -1, 0.0-0.0001),
            vect!(1, 1, 0.0+0.0001)));

        assert_eq!(xyrect.bounding_box(0.0, 0.0), Some(bb_ans));
    } 

    #[test]
    fn test_xyrect_hit() {
        let xyrect = XYRect::new(
            -1.0, 1.0, -1.0, 1.0, 0.0,
            &MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));

        let r = Ray::new(
            &vect!(0, 0, 1),
            &vect!(0, 0, -1),
            None
            );

        let mut hr_ans = HitRecord::new(
            vect!(0, 0, 0),
            1.0,
            MaterialType::DiffuseLight(
                DiffuseLight::new(
                    TextureType::ConstantTexture(
                        ConstantTexture::new(&vect!(4,4,4))))));
        hr_ans.normal = vect!(0,0,1);
        hr_ans.texture_coord = Some(TextureCoord{u: 0.5, v: 0.5});


        let hr = xyrect.hit(&r, 0.0, 1.0);
        
        assert_eq!(hr, Some(hr_ans));

    }

}
