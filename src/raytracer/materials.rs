use crate::{HitRecord, Color, reflect, refract, unit_vector, dot, random_in_unit_sphere};
use crate::raytracer::ray::Ray;
use rand::Rng;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn albedo(&self) -> Color;
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

//impl std::fmt::Display for dyn Material {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        self.inner_fmt(&self, f)
//    }
//}

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
    albedo: Color,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(a: &Color) -> Self {
        Lambertian {
            albedo: *a,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(&rec.p, &(target-&rec.p));
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
        
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambertian::albedo: {}", self.albedo)
    }

}

mat_display!(Lambertian);

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    // NOTE: Default fuzz is 1
    pub fn new(a: &Color, mut fuzz: f64) -> Self {
        if fuzz > 1.0 {fuzz=1.0};
        Metal {
            albedo: *a,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(&rec.p, &(*reflected + self.fuzz*random_in_unit_sphere()));
        let attenuation = self.albedo;
        
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        return None;
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
    
    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Metal::albedo: {}, fuzz: {}", self.albedo, self.fuzz)
    }
}
mat_display!(Metal);

#[derive(Copy, Clone)]
pub struct Dielectric {
    albedo: Color,
    ref_idx: f64,
}

impl Dielectric {
    //pub fn new(albedo: &Color, refractive_index: f64) -> Self {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric {
            albedo: Color::new(0.0,0.0,0.0),
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
        let mut refracted: Option<Color> = None;
        let mut scattered: Option<Ray> = None;

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
            refracted.replace(tmp_refracted);
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else {
            scattered.replace(Ray::new(&rec.p, &reflected)); 
            reflect_prob = 1.0;
        }
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            scattered.replace(Ray::new(&rec.p, &reflected));
        } else {
            scattered.replace(Ray::new(&rec.p, refracted.as_ref().unwrap()));
        }
        return Some((attenuation, *scattered.as_ref().unwrap()));
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }

    #[allow(dead_code)]
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dielectric::albedo: {}, refraction_index: {}", self.albedo, self.ref_idx)
    }
}
//mat_display!(Metal);


pub fn schlick(cosine: f64, refractive_index: f64) -> f64{
    let mut r0: f64 = (1.0-refractive_index) / (1.0+refractive_index);
    r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0-cosine).powf(5.0);
}