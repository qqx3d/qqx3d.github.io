use super::{Window, WindowBuilder};
use winit::{
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder},
    dpi::PhysicalSize
};
use vulkano::{
    swapchain::{Surface, Swapchain, SurfaceTransform, PresentMode, FullscreenExclusive, ColorSpace},
    device::{DeviceExtensions, Device, Queue},
    image::ImageUsage
};
use vulkano_win::VkSurfaceBuild;
use std::sync::Arc;

pub(crate) struct WindowImpl {
    pub surface: Arc <Surface <WinitWindow>>,
    pub swapchain: Arc <Swapchain <WinitWindow>>
}

static mut WINDOWS: Vec <WindowImpl> = Vec::new();

pub unsafe fn build(builder: WindowBuilder) -> Window {
    let surface = WinitWindowBuilder::new()
        .with_inner_size(PhysicalSize::from(builder.size))
        .with_title(builder.title)
        .build_vk_surface(crate::init::impls::eventloop(), crate::init::impls::instance().clone())
        .unwrap();

    let (swapchain, images) = {
        let caps = surface.capabilities(crate::init::impls::physical()).unwrap();

        let alpha = caps.supported_composite_alpha.iter().next().unwrap();

        let format = caps.supported_formats[0].0;

        Swapchain::new(
            crate::init::impls::device().clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            surface.window().inner_size().into(),
            1,
            ImageUsage::color_attachment(),
            &*crate::init::impls::queue(),
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            FullscreenExclusive::Default,
            true,
            ColorSpace::SrgbNonLinear
        ).unwrap()
    };

    WINDOWS.push(WindowImpl {
        surface,
        swapchain
    });

    Window(WINDOWS.len() - 1)
}

pub(crate) fn data(w: Window) -> &'static mut WindowImpl {
    unsafe { &mut WINDOWS[w.0] }
}
