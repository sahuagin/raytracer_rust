use super::vec3::{Color, Vec3};
use super::perlin::Perlin;
use super::vect;

pub trait Texture { fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
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
    NoiseTexture(NoiseTexture),
    Nothing(NoneTexture),
}

unsafe impl Sync for TextureType{}
unsafe impl Send for TextureType{}

impl Texture for TextureType {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match self {
            TextureType::ConstantTexture(x) => x.color,
            TextureType::CheckerTexture(x) => x.value(u, v, p),
            TextureType::NoiseTexture(x) => x.value(u, v, p),
            TextureType::Nothing(_x) => Color::new(0.0, 0.0, 0.0),
        }
    }
    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureType::ConstantTexture(x) => x.inner_fmt(f),
            TextureType::CheckerTexture(x) => x.inner_fmt(f),
            TextureType::NoiseTexture(x) => x.inner_fmt(f),
            TextureType::Nothing(x) => x.inner_fmt(f),
        }
    }
    fn albedo(&self) -> TextureType {
        match self {
            TextureType::ConstantTexture(x) => x.albedo(),
            TextureType::CheckerTexture(x) => x.albedo(),
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
        //eprintln!("CheckerTexture.value({}, {}, {})", u, v, p);
        let mult = 10.0;
        let sines = (mult * p.x).sin() * (mult*p.y).sin() * (mult*p.z).sin();
        //eprintln!("    CheckerTexture.sines = {}", &sines);
        if sines < 0. {
            //eprintln!("      less than 0. returning {})", self.odd.value(u, v, p));
            return self.odd.value(u, v, p);
        } else {
            //eprintln!("      greater than or equal to 0. returning {} )", self.even.value(u, v, p));
            return self.even.value(u, v, p);
        }
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Checker: odd: {} even: {}", self.odd, self.even )
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

    pub fn scale(mut self, sc: f64 ) -> Self {
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
        vect!(1,1,1) * 0.5 * (1.0 + (self.scale.unwrap() + 10.0 * self.inner_noise.turbulance(&noise_vec, 7)).sin())
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
    use crate::textures::{TextureType, CheckerTexture, ConstantTexture, Texture};
    use crate::vect;

    #[test]
    fn test_checker_texture() {
       let checker = TextureType::CheckerTexture(CheckerTexture::new(
        TextureType::ConstantTexture(
            ConstantTexture::new(&vect!(0.2, 0.3, 0.1))
            ),
        TextureType::ConstantTexture(
            ConstantTexture::new(&vect!(0.9, 0.9, 0.9))
            )
        ));

       for x in 0..100 {
            for y in 0..100 {
                for z in 0..100 {
                    let tmp_vec = vect!(x, y, z);
                    println!("{} at point {}", checker.value(0., 0., &tmp_vec), &tmp_vec);
                }
            }
       }
 
    }
}
