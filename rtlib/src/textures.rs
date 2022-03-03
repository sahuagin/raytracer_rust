use super::vec3::{Color, Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn albedo(&self) -> TextureType;
    fn box_clone(&self) -> Box<TextureType>;
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
    fn box_clone(&self) -> Box<TextureType> {
        Box::new(TextureType::Nothing(self::NoneTexture))
    }
}

impl NoneTexture {
    #[allow(dead_code)]
    fn new() -> Self {
        NoneTexture {}
    }
}

#[derive(Copy, Clone)]
pub enum TextureType {
    ConstantTexture(ConstantTexture),
    Nothing(NoneTexture),
}

impl Texture for TextureType {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        match self {
            TextureType::ConstantTexture(x) => x.color,
            TextureType::Nothing(_x) => Color::new(0.0, 0.0, 0.0),
        }
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureType::ConstantTexture(x) => x.inner_fmt(f),
            TextureType::Nothing(x) => x.inner_fmt(f),
        }
    }
    fn albedo(&self) -> TextureType {
        match self {
            TextureType::ConstantTexture(x) => x.albedo(),
            TextureType::Nothing(x) => x.albedo(),
        }
    }
    fn box_clone(&self) -> Box<TextureType> {
        Box::new(self.clone())
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
    pub fn new(_u: f64, _v: f64, p: &Color) -> Self {
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

    fn box_clone(&self) -> Box<TextureType> {
        Box::new(TextureType::ConstantTexture(*self))
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
