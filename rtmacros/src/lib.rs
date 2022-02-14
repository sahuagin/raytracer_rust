#[allow(unused_imports)]
use ::rtlib::prelude::Hitters;
#[allow(unused_imports)]
use ::rtlib::textures::{Texture, ConstantTexture, TextureType};
#[allow(unused_macros, unused_imports)]
use ::rtlib::materials::{MaterialType };
#[allow(unused_imports)]
use ::rtlib::prelude::Ray;
#[allow(unused_imports)]
use ::rtlib::vec3::Vec3;
#[allow(unused_macros, unused_imports)]
#[macro_export]
macro_rules! color_to_texture{
    ($col:expr) => {
        $crate::textures::TextureType::ConstantTexture(
            $crate::textures::ConstantTexture::new(0.0, 0.0, $col))
    }
}


#[allow(unused_macros, unused_imports)]
use std::fmt;
#[allow(unused_macros, unused_imports)]
use std::ops;
#[allow(unused_imports)]

#[allow(unused_macros)]
#[macro_export]
macro_rules! vect{
($x: expr, $y: expr, $z: expr) => {
    rtlib::vec3::Vec3::new($x, $y, $z)
}
}
#[macro_export]
#[allow(unused_macros)]
macro_rules! color{
($x: expr, $y: expr, $z: expr) => {
    vect!($x, $y, $z)
}
}



#[allow(unused_macros)]
#[macro_export]
macro_rules! sphere{
    ($c:expr, $r:expr, $mat:expr) => {
        rtlib::sphere::Sphere::new($c, $r, Some($mat) )
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! ray{
    ($pt1: expr, $pt2: expr, $pt3: expr) => {
        rtlib::ray::Ray::new($pt1, $pt2, Some($pt3))
    };
    ($pt1: expr, $pt2: expr) => {
        rtlib::ray::Ray::new($pt1, $pt2, None)
    };
}

#[macro_export]
macro_rules! wrap_material {
    ($klass:ident, $($p0:expr),*) => {
        $crate::materials::MaterialType::$klass(
            $klass::new(
                $( $p0, )*))
    }
}

#[macro_export]
macro_rules! wrap_texture {
    ($klass:ty, $p0:expr) => {
        rtlib::textures::TextureType::$klass($p0)
    }
}

#[macro_export]
macro_rules! wrap_hitter {
    ($klass:item, $($p:expr),*) => {
        rtlib::hittable::Hitters::$klass(
            $klass::new(
                $( $p, )*))
    }
}
 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
