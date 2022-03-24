use super::perlin::Perlin;
use super::util::{self, Image};
use super::vec3::{Color, Vec3};
use super::vect;
use std::marker::PhantomData;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn albedo(&self) -> TextureType;
    fn box_clone(&self) -> Box<dyn Texture>;
}

impl std::fmt::Display for dyn Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

#[derive(Clone, Copy, Default)]
pub struct NoneTexture;

impl Texture for NoneTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        Color::default()
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoneTexture is empty.")
    }
    fn albedo(&self) -> TextureType {
        TextureType::Nothing(NoneTexture)
    }
    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self::NoneTexture)
    }
}

impl NoneTexture {
    #[allow(dead_code)]
    fn new() -> Self {
        NoneTexture {}
    }
}

#[derive(Clone)]
pub enum TextureType {
    ConstantTexture(ConstantTexture),
    CheckerTexture(CheckerTexture),
    MappedTexture(MappedTexture),
    NoiseTexture(NoiseTexture),
    Nothing(NoneTexture),
}

unsafe impl Sync for TextureType {}
unsafe impl Send for TextureType {}

impl Texture for TextureType {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match self {
            TextureType::ConstantTexture(x) => x.color,
            TextureType::CheckerTexture(x) => x.value(u, v, p),
            TextureType::MappedTexture(x) => x.value(u, v, p),
            TextureType::NoiseTexture(x) => x.value(u, v, p),
            TextureType::Nothing(_x) => Color::new(0.0, 0.0, 0.0),
        }
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureType::ConstantTexture(x) => x.inner_fmt(f),
            TextureType::CheckerTexture(x) => x.inner_fmt(f),
            TextureType::MappedTexture(x) => x.inner_fmt(f),
            TextureType::NoiseTexture(x) => x.inner_fmt(f),
            TextureType::Nothing(x) => x.inner_fmt(f),
        }
    }
    fn albedo(&self) -> TextureType {
        match self {
            TextureType::ConstantTexture(x) => x.albedo(),
            TextureType::CheckerTexture(x) => x.albedo(),
            TextureType::MappedTexture(x) => x.albedo(),
            TextureType::NoiseTexture(x) => x.albedo(),
            TextureType::Nothing(x) => x.albedo(),
        }
    }
    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl std::fmt::Display for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner_fmt(f)
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.box_clone()
    }
}

#[derive(Copy, Clone)]
pub struct ConstantTexture {
    color: Color,
}

impl ConstantTexture {
    pub fn new(p: &Color) -> Self {
        ConstantTexture { color: *p }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.color
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstantTexture: color: {}", self.color)
    }

    fn albedo(&self) -> TextureType {
        TextureType::ConstantTexture(*self)
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(*self)
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(t0: TextureType, t1: TextureType) -> Self {
        CheckerTexture {
            odd: Box::new(t0),
            even: Box::new(t1),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let mult = 10.0;
        let sines = (mult * p.x).sin() * (mult * p.y).sin() * (mult * p.z).sin();
        if sines < 0. {
            return self.odd.value(u, v, p);
        } else {
            //eprintln!("      greater than or equal to 0. returning {} )", self.even.value(u, v, p));
            return self.even.value(u, v, p);
        }
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Checker: odd: {} even: {}", self.odd, self.even)
    }

    fn albedo(&self) -> TextureType {
        TextureType::CheckerTexture(self.clone())
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Default)]
pub struct NoiseTexture {
    inner_noise: Perlin,
    scale: Option<f64>,
}

impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            inner_noise: Perlin::new(),
            scale: Some(1.0),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        self.inner_noise.noise(&p)
    }

    pub fn scale(mut self, sc: f64) -> Self {
        self.scale = Some(sc);
        self
    }
}

impl Texture for NoiseTexture {
    // if we create an actual texture that takes these floats
    // between 0 and 1 it will create grey colors
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        //let noise_vec = self.scale.unwrap() * *p;
        let noise_vec = p;
        //vect!(1,1,1) * 0.5 * (1.0 +self.inner_noise.turbulance(&noise_vec, 7)).sin()
        // as in book
        //vect!(1,1,1) * 0.5 * (1.0 + (self.scale.unwrap() * p.z + 10.0 * self.inner_noise.turbulance(&noise_vec, 7)).sin())
        // two perlin spheres
        //vect!(1,1,1) * 0.5 * (1.0 + (self.scale.unwrap() + 10.0 * self.inner_noise.turbulance(&noise_vec, 7)).sin())
        // final scene
        vect!(1.0, 1.0, 1.0)
            * 2.0
            * (self.scale.unwrap()
                + self
                    .inner_noise
                    .turbulance(&(self.scale.unwrap() * *noise_vec), 7))
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Perlin noise: output would just be random.")
    }

    fn albedo(&self) -> TextureType {
        TextureType::NoiseTexture(self.clone())
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct MappedTexture {
    pub image: Image,
    pub filename: String,
}

#[derive(Clone, Debug, Default)]
pub struct MappedTextureBuilder<ImageFilenameSet>
where
    ImageFilenameSet: util::ToAssign,
{
    // mandatory field
    image_filename_set: PhantomData<ImageFilenameSet>,
    filename: String,
}

impl<ImageFilenameSet> MappedTextureBuilder<ImageFilenameSet>
where
    ImageFilenameSet: util::ToAssign,
{
    pub fn with_file(self, filename: &dyn ToString) -> MappedTextureBuilder<util::Yes> {
        MappedTextureBuilder {
            image_filename_set: PhantomData {},
            filename: filename.to_string(),
        }
    }
}

impl MappedTextureBuilder<util::Yes> {
    pub fn build(self) -> MappedTexture {
        MappedTexture {
            image: Image::new(&self.filename),
            filename: self.filename,
        }
    }
}

impl Texture for MappedTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Color {
        let r = self.image.get_uv(u as f32, v as f32);
        match r {
            Err(_) => {
                vect!(0, 0, 0)
            }
            Ok(x) => x,
        }
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MappedTexture: nx: {} ny: {} comp: {} using: {}",
            self.image.nx, self.image.ny, self.image.comp, self.filename
        )
    }

    fn albedo(&self) -> TextureType {
        TextureType::MappedTexture(self.clone())
    }

    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

//#[allow(unused_macros, unused_imports)]
//#[macro_export]
//macro_rules! color_to_texture{
//    ($col:expr) => {
//        TextureType::ConstantTexture(
//            ConstantTexture::new(0.0, 0.0, $col))
//    }
//}
//
//macro_rules! texture_display{
//    ($klass:ty) => {
//        #[allow(dead_code)]
//        impl std::fmt::Display for $klass {
//            fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                self.inner_fmt(f)
//            }
//        }
//    }
//}
//
//texture_display!(dyn Texture);
//texture_display!(ConstantTexture);
//
#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use crate::textures::{
        CheckerTexture, ConstantTexture, MappedTexture, MappedTextureBuilder, Texture, TextureType,
    };
    use crate::util;
    use crate::vect;
    use std::{env, path::PathBuf};

    #[test]
    fn test_checker_texture() {
        let _checker = TextureType::CheckerTexture(CheckerTexture::new(
            TextureType::ConstantTexture(ConstantTexture::new(&vect!(0.2, 0.3, 0.1))),
            TextureType::ConstantTexture(ConstantTexture::new(&vect!(0.9, 0.9, 0.9))),
        ));

        for x in 0..100 {
            for y in 0..100 {
                for z in 0..100 {
                    #[allow(unused_variables)]
                    let tmp_vec = vect!(x, y, z);
                    //poluting the output
                    //println!("{} at point {}", checker.value(0., 0., &tmp_vec), &tmp_vec);
                }
            }
        }
    }

    #[test]
    fn test_mapped_texture_builder() {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = PathBuf::from(root_dir);
        source_path.push("../results");
        source_path.push("test_image.bmp");

        let image_texture = MappedTextureBuilder::<util::No>::default()
            .with_file(&String::from(source_path.as_os_str().to_str().unwrap()))
            .build();

        let img = image_texture.image;
        let c3 = img.get(115, 164);
        let ans3 = vect!(
            115. / 255.,
            164. / 255.,
            (115_u32 * 164_u32).div_floor(255) as f64 / 255.
        );
        assert_eq!(c3.ok(), Some(ans3));
    }
}
