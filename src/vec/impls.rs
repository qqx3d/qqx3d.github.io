use winit::dpi::PhysicalSize;
use core::convert::From;

macro_rules! vec {
    ($($ty:ident : $size:literal : $($elem:ident $count:literal)*,)*) => {$(
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

        unsafe impl <T> vulkano::pipeline::vertex::VertexMember for $ty <T> where T: vulkano::pipeline::vertex::VertexMember {
            fn format() -> (vulkano::pipeline::vertex::VertexMemberTy, usize) {
                let (ty, sz) = <T as vulkano::pipeline::vertex::VertexMember>::format();
                (ty, sz * $size)
            }
        }

        impl <T> $ty <T> {
            pub fn new($($elem: T,)*) -> Self {
                Self { $($elem,)* }
            }
        }

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
}

vec! {
    Vec1 : 1 : x 0,
    Vec2 : 2 : x 0 y 1,
    Vec3 : 3 : x 0 y 1 z 2,
    Vec4 : 4 : x 0 y 1 z 2 w 3,
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
