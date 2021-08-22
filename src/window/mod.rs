mod impls;

use crate::{
    Vec2,
    Color,
    polygon::Drawable
};
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Try, FromResidual, ControlFlow}
};
use std::sync::Arc;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum WindowError {
    RerunLoop
}

impl Debug for WindowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::RerunLoop => "Try to rerun loop(use `?` in functions)"
        })
    }
}

impl FromResidual for WindowError {
    fn from_residual(_: <Self as Try>::Residual) -> Self {
        Self::RerunLoop
    }
}

impl Try for WindowError {
    type Output = ();
    type Residual = ();

    fn from_output(_: Self::Output) -> Self {
        Self::RerunLoop
    }

    fn branch(self) -> ControlFlow <Self::Residual, Self::Output> {
        match self {
            Self::RerunLoop => ControlFlow::Break(())
        }
    }
}


pub struct WindowBuilder {
    size: Vec2 <u32>,
    title: String
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            size: Vec2::new(640, 320),
            title: String::from("qqx")
        }
    }
}

impl WindowBuilder {
    #[inline]
    pub fn size(mut self, size: Vec2 <u32>) -> Self {
        self.size = size;
        self
    }

    #[inline]
    pub fn title <S> (mut self, title: S) -> Self where S: Into <String> {
        self.title = title.into();
        self
    }

    #[inline]
    pub fn build(self) -> Window {
        unsafe { impls::build(self) }
    }
}

#[derive(Copy, Clone)]
pub struct Window(usize);

impl Window {
    pub fn new() -> WindowBuilder {
        WindowBuilder::default()
    }

    pub fn draw(self, clear: Color, drawable: Arc <dyn Drawable>) -> Result <(), WindowError> {
        impls::draw(self, clear, drawable)
    }
}
