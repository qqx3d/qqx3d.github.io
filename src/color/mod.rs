use core::convert::From;
use crate::{Vec3, Vec4};
use glium::{
    vertex::{Attribute, AttributeType},
    uniforms::{AsUniformValue, UniformValue}
};

///
/// Represents RGBA Color
///
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Default for Color {
    #[inline(always)]
    fn default() -> Self { Self::BLACK }
}

unsafe impl Attribute for Color {
    fn get_type() -> AttributeType {
        AttributeType::U8U8U8U8
    }
}

impl AsUniformValue for Color {
    fn as_uniform_value(&self) -> UniformValue <'_> {
        UniformValue::Vec4(Vec4::from(*self).into())
    }
}

impl From <Vec3 <u8>> for Color {
    fn from(x: Vec3 <u8>) -> Self {
        Self::rgb(x.x, x.y, x.z)
    }
}

impl From <Color> for Vec3 <u8> {
    fn from(x: Color) -> Self {
        Self::new(x.r, x.g, x.b)
    }
}

impl From <Vec4 <u8>> for Color {
    fn from(x: Vec4 <u8>) -> Self {
        Self::rgba(x.x, x.y, x.z, x.w)
    }
}

impl From <Color> for Vec4 <u8> {
    fn from(x: Color) -> Self {
        Self::new(x.r, x.g, x.b, x.a)
    }
}

impl From <u32> for Color {
    fn from(x: u32) -> Self {
        Self::rgba((x & 0xFF) as u8, ((x >> 8) & 0xFF) as u8, ((x >> 16) & 0xFF) as u8, (x >> 24) as u8)
    }
}

impl From <Color> for u32 {
    fn from(x: Color) -> Self {
        (x.r as Self) | ((x.g as Self) << 8) | ((x.b as Self) << 16) | ((x.g as Self) << 24)
    }
}

impl From <Vec4 <f32>> for Color {
    fn from(x: Vec4 <f32>) -> Self {
        Self::rgba(Self::cv(x.x), Self::cv(x.y), Self::cv(x.z), Self::cv(x.w))
    }
}

impl From <Color> for Vec4 <f32> {
    fn from(x: Color) -> Self {
        let v = Vec3::from(x);
        Self::new(v.x, v.y, v.z, Color::vc(x.a))
    }
}

impl From <Vec3 <f32>> for Color {
    fn from(x: Vec3 <f32>) -> Self {
        Self::from(Vec4::new(x.x, x.y, x.z, 1.0))
    }
}

impl From <Color> for Vec3 <f32> {
    fn from(x: Color) -> Self {
        Self::new(Color::vc(x.r), Color::vc(x.g), Color::vc(x.b))
    }
}

impl Color {
    ///
    /// Constructs Color from RGBA values
    ///
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    ///
    /// Constructs Color from RGB values(with Alpha = MAX)
    ///
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, u8::MAX)
    }
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const RED:   Color = Color::rgb(u8::MAX, 0, 0);
    pub const GREEN: Color = Color::rgb(0, u8::MAX, 0);
    pub const BLUE:  Color = Color::rgb(0, 0, u8::MAX);
    pub const WHITE: Color = Color::rgb(u8::MAX, u8::MAX, u8::MAX);

    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
}

impl Color {
    pub(crate) const fn cv(x: f32) -> u8 {
        (x * (u8::MAX as f32)) as u8
    }

    pub(crate) const fn vc(x: u8) -> f32 {
        (x as f32) / (u8::MAX as f32)
    }
}
