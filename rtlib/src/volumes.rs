use crate::{
    aabb::BoundingBox,
    hittable::{Hittable, HitRecord},
    materials::MaterialType,
    ray::Ray,
    util,
    vect
};
use std::marker::PhantomData;
use rand::Rng;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    density: f64,
    phase_function: MaterialType,
}

impl ConstantMedium {
    pub fn new(hitit: Box<dyn Hittable>)
        -> ConstantMediumBuilder<util::Yes, util::No, util::No> {
        ConstantMediumBuilder {
            boundary_type_set: PhantomData{},
            density_type_set: PhantomData{},
            material_type_set: PhantomData{},
            boundary: hitit,
            density: f64::default(),
            phase_function: MaterialType::default(),
        }
    }

    pub fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConstantMedium: boundary: {} density: {}, material: {}",
            &self.boundary,
            &self.density,
            &self.phase_function)
    }
}

#[allow(non_camel_case_types)]
pub struct ConstantMediumBuilder<CMB_BOUNDARY_SET, CMB_DENSITY_SET, CMB_MATERIAL_SET>
where
    CMB_BOUNDARY_SET: util::ToAssign,
    CMB_DENSITY_SET: util::ToAssign,
    CMB_MATERIAL_SET: util::ToAssign,
{
    
    boundary_type_set: PhantomData<CMB_BOUNDARY_SET>,
    density_type_set: PhantomData<CMB_DENSITY_SET>,
    material_type_set: PhantomData<CMB_MATERIAL_SET>,
    boundary: Box<dyn Hittable>,
    density: f64,
    phase_function: MaterialType,
}

#[allow(non_camel_case_types)]
impl<CMB_BOUNDARY_SET, CMB_DENSITY_SET, CMB_MATERIAL_SET>
    ConstantMediumBuilder<CMB_BOUNDARY_SET, CMB_DENSITY_SET, CMB_MATERIAL_SET>
where CMB_BOUNDARY_SET: util::ToAssign,
      CMB_DENSITY_SET: util::ToAssign,
      CMB_MATERIAL_SET: util::ToAssign,
{
    pub fn with_hittable(self, hitit: Box::<dyn Hittable>)
        -> ConstantMediumBuilder<util::Yes, CMB_DENSITY_SET, CMB_MATERIAL_SET> {
            ConstantMediumBuilder {
                boundary_type_set: PhantomData{},
                density_type_set: PhantomData{},
                material_type_set: PhantomData{},
                boundary: hitit,
                density: self.density,
                phase_function: self.phase_function,
                
            }
        }
    pub fn with_density(self, density: f64)
        -> ConstantMediumBuilder<CMB_BOUNDARY_SET, util::Yes, CMB_MATERIAL_SET> {
            ConstantMediumBuilder {
                boundary_type_set: PhantomData{},
                density_type_set: PhantomData{},
                material_type_set: PhantomData{},
                boundary: self.boundary,
                density: density,
                phase_function: self.phase_function,
            }
        }

    pub fn with_phase_function(self, mat: MaterialType)
        -> ConstantMediumBuilder<CMB_BOUNDARY_SET, CMB_DENSITY_SET, util::Yes> {
            ConstantMediumBuilder {
                boundary_type_set: PhantomData{},
                density_type_set: PhantomData{},
                material_type_set: PhantomData{},
                boundary: self.boundary,
                density: self.density,
                phase_function: mat,
            }

        }
}

impl ConstantMediumBuilder<util::Yes, util::Yes, util::Yes>
{
    pub fn build(self) -> ConstantMedium {
        ConstantMedium {
            boundary: self.boundary,
            density: self.density,
            phase_function: self.phase_function,
        }
    }
}

impl Hittable for ConstantMedium {

    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        //let db: bool = rng.gen::<f64>() < 0.00001;
        let db = false;
        if let Some(mut hitrec0) = self.boundary.hit(r, f64::MIN, f64::MAX) {
            if let Some(mut hitrec1) = self.boundary.hit(r, hitrec0.t+0.0001, f64::MAX){
                if db == true {
                    eprintln!("\nt0 t1  {}  {}", &hitrec0.t, &hitrec1.t);
                }
                if hitrec0.t < tmin { hitrec0.t = tmin; }
                if hitrec1.t > tmax { hitrec1.t = tmax; }
                if hitrec0.t >= hitrec1.t { return None; }
                if hitrec0.t < 0. { hitrec0.t = 0.; }
                let distance_inside_boundary: f64 =
                    (hitrec1.t - hitrec0.t) * r.direction().length();
                let hit_distance: f64 = -1.*(1./self.density) * rng.gen::<f64>().log(10.);
                if hit_distance < distance_inside_boundary {
                    if db == true {
                        eprintln!("hit_distance = {}", &hit_distance);
                    }
                    let t = hitrec0.t + hit_distance / r.direction().length();
                    if db == true {
                        eprintln!("hitrec.t = {}", &t);
                    }
                    let p = r.point_at_parameter(t);
                    if db == true {
                        eprintln!("hitrec.p = {}", &p);
                    }
                    let normal = vect!(1, 0, 0); // arbitrary
                    let mat = self.phase_function.clone();
                    let mut retval = HitRecord::new( p, t, mat);
                    retval.normal = normal;
                    return Some(retval);
                } else {return None;}
            } else {return None;}
        } else {return None;};
    }

    fn box_clone<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<BoundingBox> {
        self.boundary.bounding_box(t0, t1)
    }

    fn hitter_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::{ConstantMedium};
    use crate::hittable::{Hittable};
    use crate::sphere::Sphere;
    use crate::vect;
    use crate::materials::{Lambertian, MaterialType, Dielectric};
    use crate::textures::{NoiseTexture, TextureType};


    #[test]
    fn test_constant_medium_construction() {
        let noise = TextureType::NoiseTexture(NoiseTexture::new());
        let text = MaterialType::Lambertian(Lambertian::new(&noise));
        let cm = ConstantMedium::new(Sphere::new(&vect!(1,2,3), 4.0, text).box_clone());
        let cm = cm.with_phase_function(MaterialType::Dielectric(Dielectric::new(&vect!(1.5, 0.2, 0.3), 0.3)));
        let cm = cm.with_density(30.0).build();

        assert_eq!(cm.density, 30.);

    }
}
