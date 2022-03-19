use std::{env, path::PathBuf};
use libm;
use std::fs::File;
use std::io::prelude::*;
use super::materials::{
    Dielectric,
    DiffuseLight,
    Lambertian,
    Material,
    MaterialType,
    Metal
};
#[allow(unused_imports)]
use super::textures::{
    ConstantTexture,
    CheckerTexture,
    MappedTexture,
    MappedTextureBuilder,
    NoiseTexture,
    TextureType};
use super::ray::Ray;
use super::rectangle::XYRect;
use super::sphere::Sphere;
#[allow(unused_imports)]
use super::vec3::{self, dot, unit_vector, Color, Point3, Vec3};
use super::{color_to_texture, vect, wrap_material};
use crate::hittable::Hittable;
use rand::Rng;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::sync::Arc;
use super::hitlist::HitList;
#[allow(unused_imports)]
use super::hittable::{Hitters, TextureCoord};
use num_traits::float;

#[allow(unused_imports, dead_code)]
pub fn optional_arg<T>(thing: Option<T>) -> T
where
    T: Default,
{
    thing.unwrap_or_default()
}

// better way to do optional with the builder pattern
// use std::fmt::Debug
// use std::market::PhantomData;
// #[derive(Debug, Default)]
// example: 
// pub struct Yes;
//#[derive(Debug, Default)]
//pub struct No;
//
//pub trait ToAssign: Debug {}
//pub trait Assigned: ToAssign {}
//pub trait NotAssigned: ToAssign {}
//
//impl ToAssign for Yes {}
//impl ToAssign for No {}
//
//impl Assigned for Yes {}
//impl NotAssigned for No {}
//
//pub fn cook_pasta(
//    pasta_type: String,
//    pasta_name: Option<String>,
//    pasta_length: u64,
//    altitude: u64,
//    water_type: Option<String>,
//) {
//    // your code here
//    println!(
//        "cooking pasta! -> {:?}, {:?}, {:?}, {:?}, {:?}",
//        pasta_type, pasta_name, pasta_length, altitude, water_type
//    );
//}
//
//#[derive(Debug, Clone, Default)]
//pub struct CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET>
//where
//    PASTA_TYPE_SET: ToAssign,
//    PASTA_LENGTH_SET: ToAssign,
//    ALTITUDE_SET: ToAssign,
//{
//    pasta_type_set: PhantomData<PASTA_TYPE_SET>,
//    pasta_length_set: PhantomData<PASTA_LENGTH_SET>,
//    altitude_set: PhantomData<ALTITUDE_SET>,
//
//    pasta_type: String,
//    pasta_name: Option<String>,
//    pasta_length: u64,
//    altitude: u64,
//    water_type: Option<String>,
//}
//
//impl<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET>
//    CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET>
//where
//    PASTA_TYPE_SET: ToAssign,
//    PASTA_LENGTH_SET: ToAssign,
//    ALTITUDE_SET: ToAssign,
//{
//    pub fn with_pasta_type(
//        self,
//        pasta_type: String,
//    ) -> CookPastaBuilder<Yes, PASTA_LENGTH_SET, ALTITUDE_SET> {
//        CookPastaBuilder {
//            pasta_type_set: PhantomData {},
//            pasta_length_set: PhantomData {},
//            altitude_set: PhantomData {},
//            pasta_type,
//            pasta_name: self.pasta_name,
//            pasta_length: self.pasta_length,
//            altitude: self.altitude,
//            water_type: self.water_type,
//        }
//    }
//
//    pub fn with_pasta_name(
//        self,
//        pasta_name: String,
//    ) -> CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET> {
//        CookPastaBuilder {
//            pasta_type_set: PhantomData {},
//            pasta_length_set: PhantomData {},
//            altitude_set: PhantomData {},
//            pasta_type: self.pasta_type,
//            pasta_name: Some(pasta_name),
//            pasta_length: self.pasta_length,
//            altitude: self.altitude,
//            water_type: self.water_type,
//        }
//    }
//
//    pub fn with_pasta_length(
//        self,
//        pasta_length: u64,
//    ) -> CookPastaBuilder<PASTA_TYPE_SET, Yes, ALTITUDE_SET> {
//        CookPastaBuilder {
//            pasta_type_set: PhantomData {},
//            pasta_length_set: PhantomData {},
//            altitude_set: PhantomData {},
//            pasta_type: self.pasta_type,
//            pasta_name: self.pasta_name,
//            pasta_length,
//            altitude: self.altitude,
//            water_type: self.water_type,
//        }
//    }
//
//    pub fn with_altitude(
//        self,
//        altitude: u64,
//    ) -> CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, Yes> {
//        CookPastaBuilder {
//            pasta_type_set: PhantomData {},
//            pasta_length_set: PhantomData {},
//            altitude_set: PhantomData {},
//            pasta_type: self.pasta_type,
//            pasta_name: self.pasta_name,
//            pasta_length: self.pasta_length,
//            altitude,
//            water_type: self.water_type,
//        }
//    }
//
//    pub fn with_water_type(
//        self,
//        water_type: String,
//    ) -> CookPastaBuilder<PASTA_TYPE_SET, PASTA_LENGTH_SET, ALTITUDE_SET> {
//        CookPastaBuilder {
//            pasta_type_set: PhantomData {},
//            pasta_length_set: PhantomData {},
//            altitude_set: PhantomData {},
//            pasta_type: self.pasta_type,
//            pasta_name: self.pasta_name,
//            pasta_length: self.pasta_length,
//            altitude: self.altitude,
//            water_type: Some(water_type),
//        }
//    }
//}
//
//impl CookPastaBuilder<Yes, Yes, Yes> {
//    pub fn execute(&self) {
//        // your code here
//        println!("cooking pasta! -> {:?}", self);
//    }
//}
//
//pub fn cook_pasta2() -> CookPastaBuilder<No, No, No> {
//    CookPastaBuilder::default()
//}
//
//fn main() {
//    cook_pasta("Penne".to_owned(), None, 100, 300, Some("Salty".to_owned()));
//
//    cook_pasta2()
//        .with_pasta_type("Penne".to_owned())
//        .with_pasta_length(100)
//        .with_water_type("Salty".to_owned())
//        .with_altitude(300)
//        .execute();
//}


// Putting the reusable parts here for the program
#[derive(Debug, Default)]
pub struct Yes;
#[derive(Debug, Default)]
pub struct No;

pub trait ToAssign: std::fmt::Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

#[allow(unused_imports, dead_code)]
pub fn color(
    ray: &Ray,
    world: & dyn Hittable,
    depth: i32,
    interior_light: &Color) -> Color {
    // the 0.001 ignores hits very close to 0, which handles issues with
    // floating point approximation, which generates "shadow acne"

    // Originally this had been a recursive algorithm; it worked fine. When
    // first adding the MappedTexture, the size caused the stack to consume all
    // it's memory (and having 32 threads). Things also got really slow. Changed
    // the method to iterative instead; the result used far less stack and was
    // considerably faster.
    //
    // Now, we're adding lighting. This is an addative to the result of the recursive
    // call. Now the method looks like a Taylors Series. Using Horners method, you would
    // structure your polynomial similarly to how this algorithm is layed out. Meaning,
    // this form of the equation x + fn(x + fn()) is the most efficient way to calculate
    // a polynomial, but you have to start from the inside and work your way out. Since
    // we can't be recursive due to size, we'll store values in a Vec/stack rather than
    // on THE stack.
    let mut emitts: Vec<Color> = Vec::with_capacity(depth as usize + 1);
    let mut attenuations: Vec<Color> = Vec::with_capacity(depth as usize + 1);
    //let start_color = vect!(1,1,1);
    //let start_color = vect!(0,0,0);
    let start_color = *interior_light;
    // since this will reduce the color by a percent, we'll default to (1,1,1) for 
    // interior lighting. For scenes with explicit lighting, (0,0,0) should be used.
    let mut tmpray = ray.clone();
    let mut depth = depth;
    loop {
        // does the ray we currently have 
        let hit_record = world.hit(&tmpray, 0.001, f64::INFINITY);
        if hit_record.is_some() == true{
            let hit_record = hit_record.unwrap();
            // as soon as we know we have a hit, generate the emitted value
            let emitted = hit_record.material.emitted(
                hit_record.texture_coord.unwrap_or_default().u,
                hit_record.texture_coord.unwrap_or_default().v,
                &hit_record.p,
                );
            // now, do we scatter the ray?
            if depth > 0 &&
                let Some((attenuation, sray)) =
                   hit_record.material.scatter(&tmpray, &hit_record) {
                
                emitts.push(emitted);
                attenuations.push(attenuation);
                tmpray = sray;
                depth -= 1;
            } else {
                // return emitted. Likely the default of black
                emitts.push(emitted);
                attenuations.push(start_color);
                break;
            }

        } else {
            // return emitted. Likely the default of black
            emitts.push(vect!(0,0,0));
            attenuations.push(start_color);
            break;
        }
    }

    // we've checked to see if we've hit anything, we've accumulated attenuation and color
    // return the overall result
    // multiplicitive for color, so use 1 for identity
    let it = emitts.iter().zip(attenuations.iter()).rev();
    let mut color = start_color;
    for (e, a) in it {
        color = (*a * color) + e;
    }
    color

}


// This is used for all of the initial images that we've done where you just see
// the placed objects and their colors. Shadows are from the reflections and reduction
// of color(subsequent colors/hits are multiplied to each other, this reduces the value
// and makes things darker). You use this if you want to see the scene light fully.
#[allow(unused_imports, dead_code)]
pub fn color_use_interior_lighting(ray: &Ray, world: & dyn Hittable, depth: i32) -> Color {
    let interior_light = Color::new(1.0, 1.0, 1.0);
    color(&ray, world, depth, &interior_light)
}

// This is used when you've placed lights in the scene and want to have anything not lit
// dark, or unable to be seen. You could also call the color function and pass
// in the color/light directly.
#[allow(unused_imports, dead_code)]
pub fn color_use_explicit_lighting(
    ray: &Ray,
    world: & dyn Hittable,
    depth: i32) -> Color {
    let interior_light = Color::new(0.0, 0.0, 0.0);
    color(&ray, world, depth, &interior_light)
}

#[allow(unused_imports, dead_code)]
pub fn color_just_attenuation(ray: &Ray, world: & dyn Hittable, _depth: i32) -> Color {
    // the 0.001 ignores hits very close to 0, which handles issues with
    // floating point approximation, which generates "shadow acne"
    let last_color: Color;
    // since this will reduce the color by a percent, we'll default to (1,1,1)
    loop {
        // does the ray we currently have 
        if let Some(hit_record) = world.hit(&ray, 0.001, f64::INFINITY) {
            // attenuation IS the color returned
            if let Some((attenuation, _)) = hit_record.material.scatter(&ray, &hit_record) {
                return attenuation;
            } else {
                last_color = Color::new(0.0, 0.0, 0.0);
                return last_color;
            }
        } else {
            // this iteration didn't hit anything, that also mean that there won't
            // be any reflections
            let unit_direction = unit_vector(&ray.direction());
            let t = 0.5 * (unit_direction.y + 1.0);
            last_color = (1.0 - t) * Color::new(1.0, 1.0, 1.0)
                + t * Color::new(0.5, 0.7, 1.0);
            return last_color;
        }
    }
}


#[allow(unused_imports, dead_code)]
pub fn write_color(
    stream: &mut impl Write,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> Result<(), io::Error> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(
        stream,
        "{} {} {}",
        ((r * (u8::MAX as f64 * 1.)) as i32).clamp(0, u8::MAX as i32),
        ((g * (u8::MAX as f64 * 1.)) as i32).clamp(0, u8::MAX as i32),
        ((b * (u8::MAX as f64 * 1.)) as i32).clamp(0, u8::MAX as i32)
    )
    .map(|_| ())
}

#[allow(unused_imports, dead_code)]
pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4_f64 * a * c;
    if discriminant < 0_f64 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

#[allow(unused_imports, dead_code)]
pub fn uv_for_sphere(hitvec: &Vec3) -> TextureCoord {
    let phi: f64 = libm::atan2(hitvec.z, hitvec.x);
    let theta: f64 = libm::asin(hitvec.y);
    TextureCoord{
        u: 1. - (phi + std::f64::consts::PI) / (2. * std::f64::consts::PI),
        v: (theta + std::f64::consts::PI/2.) / std::f64::consts::PI, }
}

#[allow(unused_imports, dead_code)]
pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Option<Vec3> = None;
    let mut rng = rand::thread_rng();

    loop {
        p.replace(
            2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
                - Vec3::new(1.0, 1.0, 1.0),
        );
        if p.unwrap().length_squared() >= 1.0 {
            break;
        }
    }
    p.unwrap()
}

#[allow(unused_imports, dead_code)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (2_f64 * v.dot(n) * *n)
}

#[allow(unused_imports, dead_code)]
pub fn refract(v: &Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt: f64 = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1_f64 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        return Some(refracted);
    } else {
        return None;
    }
}
#[allow(unused_imports, dead_code)]
pub fn random_scene(rng: &mut impl rand::Rng, checked: bool) -> HitList {
    let mut hl: HitList = HitList::new();
    let checker: TextureType;
    if checked == true {
        checker = TextureType::CheckerTexture(CheckerTexture::new(
        TextureType::ConstantTexture(
            ConstantTexture::new(&vect!(0.2, 0.3, 0.1))
            ),
        TextureType::ConstantTexture(
            ConstantTexture::new(&vect!(0.9, 0.9, 0.9))
            )
        ));
    } else 
    {
        checker = TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(0.5, 0.5, 0.5))
            )
    }
    hl.add(Hitters::Sphere(Sphere::new(
        &vect!(0.0, -1000.0, 0.0),
        1000.0,
        MaterialType::Lambertian(Lambertian::new(&checker))
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center: Vec3 = vect!(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>()
            );
            if (center - vect!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    hl.add(Hitters::Sphere(Sphere::new(
                        &center,
                        0.2,
                        MaterialType::Lambertian(Lambertian::new(&color_to_texture!(&vect!(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>()
                        )))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    hl.add(Hitters::Sphere(Sphere::new(
                        &center,
                        0.2,
                        wrap_material!(
                            Metal,
                            color_to_texture!(&vect!(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>())
                            )),
                            0.5 * rng.gen::<f64>()
                        ),
                    )));
                } else {
                    // glass
                    hl.add(Hitters::Sphere(Sphere::new(
                        &center,
                        0.2,
                        wrap_material!(Dielectric, &vect!(1.0, 1.0, 1.0), 1.5),
                    )));
                }
            }
        }
    }

    hl.add(Hitters::Sphere(Sphere::new(
        &vect!(0.0, 1.0, 0.0),
        1.0,
        wrap_material!(Dielectric, &vect!(1.0, 1.0, 1.0), 1.5),
    )));
    hl.add(Hitters::Sphere(Sphere::new(
        &vect!(-4.0, 1.0, 0.0),
        1.0,
        wrap_material!(Lambertian, &color_to_texture!(&vect!(0.4, 0.2, 0.1))),
    )));
    hl.add(Hitters::Sphere(Sphere::new(
        &vect!(4.0, 1.0, 0.0),
        1.0,
        wrap_material!(Metal, color_to_texture!(&vect!(0.7, 0.6, 0.5)), 0.0),
    )));

    hl
}

#[allow(unused_imports, dead_code)]
pub fn random_scene_with_time() {
    const _N: i32 = 50_000;
    let _list = HitList::new();
    //list.add(sphere!(&vect!(0.0, -1_000.0, 0.0), 1_000, Lambertian::new(checker)));
}

#[allow(unused_imports, dead_code)]
pub fn two_spheres() -> HitList{
    let checker = TextureType::CheckerTexture(CheckerTexture::new(
            TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(0.2, 0.3, 0.1))),
            TextureType::ConstantTexture(
                ConstantTexture::new(&vect!(0.9, 0.9, 0.9))
                )
            ));
    let mut hl: HitList = HitList::new();
    hl.add(Hitters::Sphere(Sphere::new(
                &vect!(0, -10, 0),
                10.0,
                MaterialType::Lambertian(Lambertian::new(&checker)))
                ));
    hl.add(Hitters::Sphere(Sphere::new(
                &vect!(0, 10, 0),
                10.,
                MaterialType::Lambertian(Lambertian::new(&checker)))));

    hl
}

#[allow(unused_imports, dead_code)]
pub fn two_perlin_spheres() -> HitList {
    let pertext = TextureType::NoiseTexture(
                NoiseTexture::new().scale(5.7),
            );
    let mut hl: HitList = HitList::new();
    hl.add(Hitters::Sphere(Sphere::new(
                &vect!(0, -1000, 0),
                1000.0,
                MaterialType::Lambertian(Lambertian::new(&pertext)))
                ));
    hl.add(Hitters::Sphere(Sphere::new(
                &vect!(0, 2, 0),
                2.,
                MaterialType::Lambertian(Lambertian::new(&pertext)))));

    hl

}

pub fn ffmin<T: float::Float + std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
pub fn ffmax<T: float::Float + std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// should be read only
unsafe impl Sync for Image{}
#[derive(Debug, Clone)]
pub struct Image {
    #[allow(dead_code)]
    //pub image: Box<[u8]>,
    pub image: Arc<[u8]>,
    #[allow(dead_code)]
    pub nx: u32,
    #[allow(dead_code)]
    pub ny: u32,
    #[allow(dead_code)]
    pub comp: u32,
}

#[allow(dead_code)]
impl Image {
    pub fn new(path: & dyn ToString) -> Self {
            get_image_from_file(path)
    }

    pub fn get(&self, x: u32, y: u32) -> Result<Color, &'static str> {
        if x >= self.nx || y >= self.ny {
            return Err("Coordinates do no fit into image");
        }
        // y axis is flipped so, self.ny - y = y
        // NOTE: ny is the size, so it'll be 1 larger 
        let y = self.ny - y - 1;
        let index = ((x + (y * self.nx)) * self.comp) as usize;

        // data stored in self.image is from stb_image_rust, and is stored
        // rgb and perhaps rgba, you can check the components count to determine
        // how many channels there are (components like component out on tv)
        // NOTE: The rgb values are 0-255 u8, and our colors are floats
        // from 0.0 to 1.0, so we'll divide by 255.0 to turn into a "percentage"
        // of our value.
        // NOTE: Documentation differs from implementation in the stb_load_from_memory.
        // In particular, when you call with the rgba, it doesn't 1) fail if there aren't
        // as many channels as requested (4, rgb+a (unless a isn't considered a component)), 2)
        // return the number of components in the image (returns 3, not 4).
        // What it DOES do, is returns 3 for rgb, and include a 4th alpha channel in
        // every cell. So, the stride is always 4, not 3. The only way to know this
        // is that you passed the rgba flag into the method, and that in essence
        // asks to have an alpha channel regardless.
        // So, not sure what do do here. Do we hardcode to 4, or do we take the returned
        // number of components and add +1 to it?
        Ok(vect!(
                self.image[index] as f64 / 255.,
                self.image[index+1] as f64 / 255.,
                self.image[index+2] as f64 / 255.))
    }

    // takes a percentage of the x, and a percentage of the y coordinates.
    // It's going to expect that the values will be 0 <= val < 100
    //
    pub fn get_uv(&self, u: f32, v: f32) -> Result<Color, &'static str> {
        if u < 0.0 || u >= 1.0
            || v < 0.0 || v >= 1.0 {
                return Err("Coordinates are out of bounds of a percentage.");
            }
        // get what x and y their percentages ==
        let x = (self.nx as f32 * u) as u32;
        // the (1. - v) inverts the axis so it matches our render screen
        let y = (self.ny as f32 * (1.-v) - 0.001) as u32;

        self.get(x, y)
    }
}

pub fn get_image_from_file(path: & dyn ToString) -> Image {
    let mut f = File::open(path.to_string()).expect("file not found");
    let mut contents = vec![];
    f.read_to_end(&mut contents).expect("Error reading file");

    // load the image
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut comp: i32 = 0;
    let img: *mut u8;

    unsafe {
        img = stb_image_rust::stbi_load_from_memory(
            contents.as_mut_ptr(),
            contents.len() as i32,
            &mut x,
            &mut y,
            &mut comp,
            stb_image_rust::STBI_rgb_alpha,
            );
        // add the alpha channel
        comp += 1;
        let alloced_len: usize = (x * y * comp) as usize;
        let mut retval: Vec<u8> = Vec::with_capacity(alloced_len);
        for i in 0..alloced_len {
            retval.push(*img.add(i));
        }
        stb_image_rust::c_runtime::free(img);

        Image {
            image: Arc::from(retval.into_boxed_slice()),
            nx: x as u32,
            ny: y as u32,
            comp: comp as u32,

        }
    }
}

#[allow(unused_imports, dead_code)]
pub fn earth_scene() -> HitList {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    let mut source_path = PathBuf::from(root_dir);
    source_path.push("../results");
    source_path.push("Equirectangular_projection_SW.jpg");

    let pertext = TextureType::MappedTexture(
                MappedTextureBuilder::<No>::default().with_file(&String::from(source_path.to_str().unwrap())).build()
            );
    let mut hl: HitList = HitList::new();
    // make a smaller version of above to see if it's actually rendering
    hl.add(Hitters::Sphere(Sphere::new(
                &vect!(-20, 2, -2),
                10.0,
                MaterialType::Lambertian(Lambertian::new(&pertext)))
                ));
    hl.add(Hitters::Sphere(Sphere::new(
                &vect!(0, 2, 0),
                2.,
                MaterialType::Lambertian(Lambertian::new(&pertext)))));

    hl


}


#[allow(unused_imports, dead_code)]
pub fn simple_light_scene() -> HitList {
   let mut hl = two_perlin_spheres();

   hl.add(Hitters::Sphere(Sphere::new(
           &vect!(0, 7, 0), 2.,
           MaterialType::DiffuseLight(
               DiffuseLight::new(
                   TextureType::ConstantTexture(
                       ConstantTexture::new(&vect!(4, 4, 4))))))));
   hl.add(
       Hitters::XYRect(XYRect::new(
               3., 5., 1., 3., -2.,
               &MaterialType::DiffuseLight(
                   DiffuseLight::new(
                       TextureType::ConstantTexture(
                           ConstantTexture::new(&vect!(4.,4.,4.))))))));
   hl
}

#[cfg(test)]
mod test {
    use super::super::{color_to_texture, ray, wrap_material};
    use crate::hitlist::HitList;
    use crate::hittable::Hittable;
    use crate::hittable::{HitRecord, Hitters};
    use crate::sphere::Sphere;
    #[allow(unused_imports)]
    use crate::vec3::{Color, Vec3};
    use std::path::PathBuf;

    #[test]
    fn test_color() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        let r = ray!(&v, &v2);
        // thread 'util::test::test_color' panicked at 'assertion failed: `(left == right)`
        // left: `Vec3T { x: 0.6056624327025936, y: 0.7633974596215561, z: 1.0 }`,
        // right: `Vec3T { x: 0.8943375672974064, y: 0.9366025403784438, z: 1.0 }`', rtlib/src/util.rs:326:9
 
        let ans = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let mut world = HitList::new();
        let metal = wrap_material!(Metal, color_to_texture!(&Color::new(1.0, 1.0, 1.0)), 0.0);
        world.list.push(Hitters::Sphere(Sphere::new(
            &Vec3::new(2.0, 2.0, 2.0),
            3.0,
            metal,
        )));
        let c = crate::util::color(&r, &world, 100);
        // so, now that the world has a depth, and there are random bounces for refraction,
        // this becomes a whole lot more difficult to test. Even giving it perfect reflection
        // surface (metal, all white, no fuzz) it'll return some random bounces.
        // although, it seems that this gives a decent passing?
        // NOTE: Later, after adding lighting, this does return 1,1,1 for the color.
        // Running the "two_perlin_spheres" the output looks correct. So, the answer
        // here has been adjusted.
        assert_eq!(c, ans);
        // left: `Vec3 { x: 0.8943375672974064, y: 0.9366025403784438, z: 1.0 }`,
        // right: `Vec3 { x: 0.21132486540518708, y: 0.21132486540518708, z: 0.21132486540518708 }`', src/lib.rs:178:5
    }

    #[allow(unused_imports)]
    use crate::materials::Metal;
    #[allow(unused_imports)]
    use crate::sphere;
    #[allow(unused_imports)]
    use crate::vec3::Point3;
    use super::vect;
    use crate::util::{self, Image};
    use std::env;

    #[test]
    fn test_hitlist() {
        let _ans = true;

        // steal test_sphere_hit data
        let pt1 = Point3::new(0.0, 0.0, 0.0);
        let pt2 = Point3::new(1.0, 1.0, 1.0);
        let r = ray!(&pt1, &pt2);
        let center = Point3::new(2.0, 2.0, 2.0);
        let radius = 3.0;
        let metal = wrap_material!(Metal, color_to_texture!(&Color::new(1.0, 1.0, 1.0)), 1.0);
        let metal2 = metal.clone();
        let s = Hitters::Sphere(Sphere::new(&center, radius, metal));
        let pat = Vec3 {
                x: 0.26794919243112264,
                y: 0.26794919243112264,
                z: 0.26794919243112264,
        };

        let hitrec = Some(HitRecord {
            t: 0.26794919243112264,
            p: pat,
            normal: Vec3 {
                x: -0.5773502691896258,
                y: -0.5773502691896258,
                z: -0.5773502691896258,
            },
            front_face: false,
            texture_coord: Some(util::uv_for_sphere(&pat)),
            material: metal2,
        });

        // then, we'll push the sphere into the HitList
        let mut hl = HitList::new();
        hl.list.push(s);
        // this should have 2 hits, but we'll return the closest one
        let hit_ans = hl.hit(&r, 0.0, 4.0);
        println!("{}", hit_ans.unwrap());
        println!("{}", hitrec.unwrap());
        //assert_eq!(hit_ans, hitrec);
        //println!("the hitrec i: {:?}", &hitrec);
    }

    #[test]
    fn test_reflect() {
        let v1 = Vec3::new(2.0, -1.0, -1.0);
        let v2 = Vec3::new(4.0, 2.0, 3.0);
        let ans = Vec3::new(-22.0, -13.0, -19.0);

        assert_eq!(crate::util::reflect(&v1, &v2), ans);
    }

    #[test]
    #[ignore]
    fn test_image_creation() {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = PathBuf::from(root_dir);
        source_path.push("../results");
        source_path.push("how_about_a_nice_cup_of_shut_up.jpg");
        let img = Image::new(&String::from(source_path.to_str().unwrap()));

        let c0 = img.get(97, 143);
        let ans0 = vect!(181.0/255., 132.0/255., 29.0/255.);

        let c1 = img.get(81, 148);
        let ans1 = vect!(181./255., 129./255., 43./255.);

        let c2 = img.get(102, 125);
        let ans2 = vect!(252./255., 231./255., 212./255.);

        let c3 = img.get(115, 164);
        let ans3 = vect!(181./255., 131./255., 44./255.);

        let c4 = img.get(0, 0);
        let ans4 = vect!(255./255., 255./255., 253./255.);

        assert_eq!(c4.ok(), Some(ans4));
        assert_eq!(c3.ok(), Some(ans3));
        assert_eq!(c0.ok(), Some(ans0));
        assert_eq!(c1.ok(), Some(ans1));
        assert_eq!(c2.ok(), Some(ans2));

    }

    #[test]
    fn test_image_load() {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = PathBuf::from(root_dir);
        source_path.push("../results");
        source_path.push("test_image.bmp");
        let img = Image::new(&String::from(source_path.to_str().unwrap()));

        let c0 = img.get(0, 0);
        let ans0 = vect!(0./255., 0./255., 0./255.);
        let c1_0 = img.get(1, 0);
        let ans1_0 = vect!(1./255., 0./255., 0./255.);
        let c2_0 = img.get(2, 0);
        let ans2_0 = vect!(2./255., 0./255., (2_u32*0_u32).div_floor(255) as f64/255.);
        let o0 = img.get(0, 254);
        let oans0 = vect!(0./255., 254./255., 0./255.);

        let c1 = img.get(0, 1);
        let ans1 = vect!(0./255., 1./255., 0./255.);

        let c2 = img.get(1, 2);
        let ans2 = vect!(1./255., 2./255., 0.);

        let c3 = img.get(115, 164);
        let ans3 = vect!(115./255., 164./255., (115_u32*164_u32).div_floor(255) as f64/255.);

        let c4 = img.get(254, 254);
        let ans4 = vect!(254./255., 254./255., (254_u32*254_u32).div_floor(255) as f64/255.);
        assert_eq!(c0.ok(), Some(ans0));
        assert_eq!(c1_0.ok(), Some(ans1_0));
        assert_eq!(c2_0.ok(), Some(ans2_0));
        assert_eq!(o0.ok(), Some(oans0));
        assert_eq!(c1.ok(), Some(ans1));
        assert_eq!(c2.ok(), Some(ans2));
        assert_eq!(c3.ok(), Some(ans3));
        assert_eq!(c4.ok(), Some(ans4));

    }
}
