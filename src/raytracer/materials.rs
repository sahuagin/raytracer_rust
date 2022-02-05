use crate::{HitRecord, Color, reflect, refract, unit_vector, dot, random_in_unit_sphere};
use crate::raytracer::ray::Ray;
pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn albedo(&self) -> Color;
}

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
        //let reflected = &reflect(&unit_vector(&ray_in.direction()), &rec.normal);
        //let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;
        //match dot(&scattered.direction(), &rec.normal) > 0.0 {
        //    true =>  {return Some((attenuation, scattered));},
        //false => {return None;}
        //};
        return Some((attenuation, scattered));
        
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
}

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
}

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
        //let reflected = reflect(&ray_in.direction(), &rec.normal);
        let mut ni_over_nt:f64 = self.ref_idx;
        let attenuation = Color::new(1.0, 1.0, 1.0);

        if dot(&ray_in.direction(), &outward_normal) > 0.0 {
            // otherwise it goes into the object
            outward_normal *= -1.0;
        } else {
            // otherwise, it's the inverse refractive index
            ni_over_nt = 1.0 / self.ref_idx;
        }
        if let Some(refracted) = refract(&ray_in.direction(), outward_normal, ni_over_nt) {
            // should also return true
            return Some((attenuation, Ray::new(&rec.p, &refracted)));
        }
        else {
            // should also return false
            // note: in the tutorial, these values were set and then 
            // false was returned. In color, if false is returned, the values
            // aren't used and a default color(black) is used.
            // NOTE: In the tutorial this is listed as a
            // "major bug that still leaves a reasonably plausible image"
            //return Some((attenuation, Ray3::new(rec.p, reflected)));
            return None;
        }
    }
    
    fn albedo(&self) -> Color {
        self.albedo
    }
}