use super::util::{reflect, refract, random_in_unit_sphere};
use super::vec3::{Color, dot, unit_vector};
use super::hittable::HitRecord;
use super::ray::Ray;
use super::textures::{ConstantTexture, Texture, TextureType, NoneTexture};
use rand::Rng;

pub trait Material: {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn albedo(&self) -> TextureType;
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn box_clone(&self) -> Box<MaterialType>;
}

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
    Nothing(NoneMaterial),
}

impl Default for MaterialType {
    fn default() -> Self {MaterialType::Nothing(NoneMaterial)}
}

#[derive(Copy, Clone)]
pub struct NoneMaterial;

impl Material for NoneMaterial {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)>{
        return None;
    }
    fn albedo(&self) -> TextureType {
        TextureType::Nothing(NoneTexture)
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "NoneMaterial is empty.")
    }
    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Nothing(*self))
    }
}


impl Material for MaterialType {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        match self {
            MaterialType::Lambertian(innertype) => {
                return innertype.scatter(ray_in, rec);} ,
            MaterialType::Dielectric(innertype) => {
                return innertype.scatter(ray_in, rec);},
            MaterialType::Metal(innertype) => {
                return innertype.scatter(ray_in, rec);},
            MaterialType::Nothing(_innertype) => {
                return None;
            }
        }
    }
    fn albedo(&self) -> TextureType{
        match self {
            MaterialType::Lambertian(innertype) => 
                return innertype.albedo(),
            MaterialType::Dielectric(innertype) =>
                return innertype.albedo(),
            MaterialType::Metal(innertype) =>
                return innertype.albedo(),
            MaterialType::Nothing(_innertype) => {
                TextureType::Nothing(NoneTexture)
            }
        }
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self {
            MaterialType::Lambertian(innertype) =>
                return innertype.inner_fmt(f),
            MaterialType::Dielectric(innertype) =>
                return innertype.inner_fmt(f),
            MaterialType::Metal(innertype) =>
                return innertype.inner_fmt(f),
            MaterialType::Nothing(innertype) => {
                innertype.inner_fmt(f)
            }
        }
    }
    fn box_clone(&self) -> Box<MaterialType>{
        match self {
            MaterialType::Lambertian(innertype) => {
                return Box::new(MaterialType::Lambertian(*innertype));
            },
            MaterialType::Dielectric(innertype) => {
                return Box::new(MaterialType::Dielectric(*innertype));
            },
            MaterialType::Metal(innertype) => {
                return Box::new(MaterialType::Metal(*innertype));
            },
            MaterialType::Nothing(_innertype) => {
                Box::new(MaterialType::Nothing(NoneMaterial))
            }
        }
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}
// we want the material trait to be supported for references AND non-references
//impl<'a, T> Material for &'a T where T: Material { }
//impl<'a, T> Material for &'a mut T where T: Material {}

macro_rules! mat_display{
    ($klass:ty) => {
        #[allow(dead_code)]
        impl std::fmt::Display for $klass {
            fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner_fmt(f)
            }
        }
    };
}

mat_display!(dyn Material);
// because the material can be None, we'll also add a specialization for an option
// can't do this because of the orphan rule
//impl std::fmt::Display for std::option::Option<&dyn Material> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        match self.is_some() {
//            true => return self.as_ref().inner_fmt(f),
//            false => return write!(f, "None"),
//        }
//    }
//}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: TextureType,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(texture: &TextureType) -> Self {
        Lambertian {
            albedo: *texture,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(&rec.p, &(target-&rec.p), None);
        let attenuation = self.albedo().value(0.0, 0.0, &target);
        return Some((attenuation, scattered));
        
    }
    
    fn albedo(&self) -> TextureType {
        self.albedo
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambertian::albedo: ")?;
        let alb = self.albedo();
            return alb.inner_fmt(f);
    }
    
    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Lambertian(*self))
    }

}

mat_display!(Lambertian);

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: TextureType,
    fuzz: f64,
}

impl Metal {
    // NOTE: Default fuzz is 1
    pub fn new(a: TextureType, mut fuzz: f64) -> Self {
        if fuzz > 1.0 {fuzz=1.0};
        Metal {
            albedo: a,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(&rec.p, &(*reflected + self.fuzz*random_in_unit_sphere()), None);
        let attenuation = self.albedo().value(0.0,0.0, reflected);
        
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        return None;
    }

    fn albedo(&self) -> TextureType {
        self.albedo
    }
    
    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Metal::fuzz: {} albedo: ", self.fuzz)?;
        return self.albedo().inner_fmt(f);
    }
    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Metal(*self))
    }
}
mat_display!(Metal);

#[derive(Copy, Clone)]
pub struct Dielectric {
    #[allow(dead_code)]
    albedo: TextureType,
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(albedo: &Color, refractive_index: f64) -> Self {
            Dielectric {
                albedo: TextureType::ConstantTexture(
                    ConstantTexture::new(
                    0.0,0.0,
                    albedo )),
                ref_idx: refractive_index,
            }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut outward_normal = rec.normal;
        let reflected = reflect(&ray_in.direction(), &rec.normal);
        let mut ni_over_nt:f64 = self.ref_idx;
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let cosine:f64;
        let reflect_prob: f64;
        let mut refracted = Color::new(0.0,0.0,0.0);
        let scattered: Ray;

        if dot(&ray_in.direction(), &outward_normal) > 0.0 {
            // otherwise it goes into the object
            outward_normal *= -1.0;
            cosine = self.ref_idx * dot(&ray_in.direction(), &rec.normal) / ray_in.direction().length();
        } else {
            // otherwise, it's the inverse refractive index
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(&ray_in.direction(), &rec.normal) / ray_in.direction().length()
        }
        if let Some(tmp_refracted) = refract(&ray_in.direction(), outward_normal, ni_over_nt) {
            refracted = tmp_refracted;
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            scattered = Ray::new(&rec.p, &reflected, None);
        } else {
            scattered = Ray::new(&rec.p, &refracted, None);
        }
        return Some((attenuation, scattered));
    }
    
    fn albedo(&self) -> TextureType {
        self.albedo
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dielectric::refraction_index: {} albedo: ", self.ref_idx)?;
        return self.albedo().inner_fmt(f);
    }

    fn box_clone(&self) -> Box<MaterialType> {
        Box::new(MaterialType::Dielectric(*self))
    }
}
//mat_display!(Metal);


pub fn schlick(cosine: f64, refractive_index: f64) -> f64{
    let mut r0: f64 = (1.0-refractive_index) / (1.0+refractive_index);
    r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0-cosine).powf(5.0);
}
