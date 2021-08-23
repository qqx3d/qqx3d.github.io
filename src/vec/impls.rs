use glium::{
    glutin::dpi::PhysicalSize,
    vertex::{Attribute, AttributeType}
};
use core::convert::From;

macro_rules! vec {
    ($($ty:ident : $uniform:ident : $size:literal : $($elem:ident $count:literal)* : $i8:ident : $u8:ident : $i16:ident : $u16:ident : $i32:ident : $u32:ident : $i64:ident : $u64:ident : $f32:ident : $f64:ident,)*) => {$(
        #[derive(Debug)]
        pub struct $ty <T> {
            $(pub $elem : T,)*
        }

        impl <T> Clone for $ty <T> where T: Clone {
            fn clone(&self) -> Self {
                Self { $($elem: self.$elem.clone(),)* }
            }
        }

        impl <T> Copy for $ty <T> where T: Copy { }

        impl <T> Default for $ty <T> where T: Default {
            fn default() -> Self {
                Self { $($elem: T::default(),)* }
            }
        }

        vec!{ @op $ty, Add, AddAssign, add, add_assign, +, $($elem)* }
        vec!{ @op $ty, Sub, SubAssign, sub, sub_assign, -, $($elem)* }
        vec!{ @op $ty, Mul, MulAssign, mul, mul_assign, *, $($elem)* }
        vec!{ @op $ty, Div, DivAssign, div, div_assign, /, $($elem)* }
        vec!{ @op $ty, Rem, RemAssign, rem, rem_assign, %, $($elem)* }

        impl <T> core::ops::Neg for $ty <T> where T: core::ops::Neg <Output = T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self { $($elem: -self.$elem,)* }
            }
        }

        impl <T> From <[T; $size]> for $ty <T> where T: Copy {
            fn from(x: [T; $size]) -> Self {
                Self { $($elem: x[$count],)* }
            }
        }

        impl <T> From <$ty <T>> for [T; $size] where T: Copy {
            fn from(x: $ty <T>) -> Self {
                [$(x.$elem,)*]
            }
        }

        impl glium::uniforms::AsUniformValue for $ty <f32> {
            fn as_uniform_value(&self) -> glium::uniforms::UniformValue <'_> {
                glium::uniforms::UniformValue::$uniform((*self).into())
            }
        }

        impl <T> $ty <T> {
            pub fn new($($elem: T,)*) -> Self {
                Self { $($elem,)* }
            }
        }

        vec!{ @at $ty i8 $i8 }
        vec!{ @at $ty u8 $u8 }
        vec!{ @at $ty i16 $i16 }
        vec!{ @at $ty u16 $u16 }
        vec!{ @at $ty i32 $i32 }
        vec!{ @at $ty u32 $u32 }
        vec!{ @at $ty i64 $i64 }
        vec!{ @at $ty u64 $u64 }
        vec!{ @at $ty f32 $f32 }
        vec!{ @at $ty f64 $f64 }

    )*};

    (@op $ty:ident, $big:ident, $big_assign:ident, $low:ident, $low_assign:ident, $op:tt, $($elem:ident)*) => {
        impl <T> core::ops::$big for $ty <T> where T: core::ops::$big <Output = T> {
            type Output = Self;

            fn $low(self, rhs: Self) -> Self::Output {
                Self::Output { $($elem: self.$elem $op rhs.$elem,)* }
            }
        }

        impl <T> core::ops::$big_assign for $ty <T> where T: core::ops::$big <Output = T> + Copy {
            fn $low_assign(&mut self, rhs: Self) {
                *self = *self $op rhs
            }
        }

        impl <T> core::ops::$big <T> for $ty <T> where T: core::ops::$big <Output = T> + Copy {
            type Output = Self;

            fn $low(self, rhs: T) -> Self::Output {
                Self::Output { $($elem: self.$elem $op rhs,)* }
            }
        }

        impl <T> core::ops::$big_assign <T> for $ty <T> where T: core::ops::$big <Output = T> + Copy {
            fn $low_assign(&mut self, rhs: T) {
                *self = *self $op rhs
            }
        }
    };

    (@at $name:ident $ty:ident $id:ident) => {
        unsafe impl Attribute for $name <$ty> {
            #[inline]
            fn get_type() -> AttributeType {
                AttributeType::$id
            }
        }
    };
}

vec! {
    Vec1 : Float : 1 : x 0             : I8       : U8       : I16          : U16          : I32          : U32          : I64          : U64          : F32          : F64,
    Vec2 : Vec2  : 2 : x 0 y 1         : I8I8     : U8U8     : I16I16       : U16U16       : I32I32       : U32U32       : I64I64       : U64U64       : F32F32       : F64F64,
    Vec3 : Vec3  : 3 : x 0 y 1 z 2     : I8I8I8   : U8U8U8   : I16I16I16    : U16U16U16    : I32I32I32    : U32U32U32    : I64I64I64    : U64U64U64    : F32F32F32    : F64F64F64,
    Vec4 : Vec4  : 4 : x 0 y 1 z 2 w 3 : I8I8I8I8 : U8U8U8U8 : I16I16I16I16 : U16U16U16U16 : I32I32I32I32 : U32U32U32U32 : I64I64I64I64 : U64U64U64U64 : F32F32F32F32 : F64F64F64F64,
}

impl <T> From <T> for Vec1 <T> {
    fn from(x: T) -> Self {
        Self { x }
    }
}

impl From <Vec1 <f32>> for f32 {
    fn from(x: Vec1 <f32>) -> Self {
        x.x
    }
}

impl <T> From <PhysicalSize <T>> for Vec2 <T> {
    fn from(x: PhysicalSize <T>) -> Self {
        Self {
            x: x.width,
            y: x.height
        }
    }
}

impl <T> From <Vec2 <T>> for PhysicalSize <T> {
    fn from(x: Vec2 <T>) -> Self {
        Self {
            width:  x.x,
            height: x.y
        }
    }
}
