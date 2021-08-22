use super::{Window, WindowBuilder, WindowError};
use crate::{
    Color,
    polygon::Drawable
};
use winit::{
    window::{Window as WinitWindow, WindowBuilder as WinitWindowBuilder, WindowId},
    dpi::PhysicalSize
};
use vulkano::{
    swapchain::{self, Surface, Swapchain, SurfaceTransform, PresentMode, FullscreenExclusive, ColorSpace, SwapchainCreationError, AcquireError},
    image::{ImageUsage, SwapchainImage},
    framebuffer::{RenderPassAbstract, FramebufferAbstract, Framebuffer},
    command_buffer::{DynamicState, AutoCommandBufferBuilder, AutoCommandBuffer, SubpassContents},
    pipeline::viewport::Viewport,
    sync::{self, GpuFuture, FlushError}
};
use vulkano_win::VkSurfaceBuild;
use std::sync::Arc;

// TODO: pub(crate)
pub struct WindowImpl {
    pub surface: Arc <Surface <WinitWindow>>,
    pub swapchain: Arc <Swapchain <WinitWindow>>,
    pub renderpass: Arc <dyn RenderPassAbstract + Send + Sync>,
    pub dynamic_state: DynamicState,
    pub framebuffers: Vec <Arc <dyn FramebufferAbstract + Send + Sync>>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option <Box <dyn GpuFuture>>
}

static mut WINDOWS: Vec <WindowImpl> = Vec::new();

fn window_update_on_resize(images: &[Arc <SwapchainImage <WinitWindow>>],
                           renderpass: Arc <dyn RenderPassAbstract + Send + Sync>,
                           dynamic_state: &mut DynamicState) -> Vec <Arc <dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(renderpass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap()
            ) as Arc <dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec <_>>()
}

pub unsafe fn build(builder: WindowBuilder) -> Window {
    let surface = WinitWindowBuilder::new()
        .with_inner_size(PhysicalSize::from(builder.size))
        .with_title(builder.title)
        .build_vk_surface(crate::init::Stt::eventloop(), crate::init::Stt::instance().clone())
        .unwrap();

    let (swapchain, images) = {
        let caps = surface.capabilities(crate::init::Stt::physical()).unwrap();

        let alpha = caps.supported_composite_alpha.iter().next().unwrap();

        let format = caps.supported_formats[0].0;

        Swapchain::new(
            crate::init::Stt::device().clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            surface.window().inner_size().into(),
            1,
            ImageUsage::color_attachment(),
            &*crate::init::Stt::queue(),
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            FullscreenExclusive::Default,
            true,
            ColorSpace::SrgbNonLinear
        ).unwrap()
    };

    let renderpass = Arc::new(vulkano::single_pass_renderpass!(
        crate::init::Stt::device().clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());

    let mut dynamic_state = DynamicState::none();

    let framebuffers = window_update_on_resize(&images, renderpass.clone(), &mut dynamic_state);

    let recreate_swapchain = false;

    let previous_frame_end = Some(sync::now(crate::init::Stt::device().clone()).boxed());

    WINDOWS.push(WindowImpl {
        surface,
        swapchain,
        renderpass,
        dynamic_state,
        framebuffers,
        recreate_swapchain,
        previous_frame_end
    });

    Window(WINDOWS.len() - 1)
}

pub fn draw(w: Window, clear: Color, drawable: Arc <dyn Drawable>) -> Result <(), WindowError> {
    let (image, suboptimal, future) = match swapchain::acquire_next_image(w.data().swapchain.clone(), None) {
        Ok(x) => x,
        Err(AcquireError::OutOfDate) => {
            w.data().recreate_swapchain = true;
            return Err(WindowError::RerunLoop)
        },
        Err(e) => panic!("Failed to acquire next image: {:?}", e)
    };

    if suboptimal { w.data().recreate_swapchain = true }

    let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
        crate::init::Stt::device().clone(),
        crate::init::Stt::queue().family(),
    ).unwrap();

    builder
        .begin_render_pass(w.data().framebuffers[image].clone(), SubpassContents::Inline, clear.into())
        .unwrap();
    drawable
        .draw(&mut builder, w.data().renderpass.clone(), &mut w.data().dynamic_state);
    builder
        .end_render_pass()
        .unwrap();

    let command = builder.build().unwrap();

    let future = w.data().previous_frame_end
        .take()
        .unwrap()
        .join(future)
        .then_execute(crate::init::Stt::queue().clone(), command)
        .unwrap()
        .then_swapchain_present(crate::init::Stt::queue().clone(), w.data().swapchain.clone(), image)
        .then_signal_fence_and_flush();

    match future {
        Ok(future) => w.data().previous_frame_end = Some(future.boxed()),
        Err(e) => {
            if let FlushError::OutOfDate = e {
                w.data().recreate_swapchain = true
            } else {
                println!("Failed to flush future: {:?}", e)
            }
            w.data().previous_frame_end = Some(sync::now(crate::init::Stt::device().clone()).boxed())
        }
    }

    Ok(())
}

impl Window {
    #[inline]
    pub(crate) fn data(self) -> &'static mut WindowImpl {
        unsafe { &mut WINDOWS[self.0] }
    }

    pub(crate) fn by_wid(wid: WindowId) -> Self {
        let mut i = 0;
        unsafe {
            while i < WINDOWS.len() {
                if WINDOWS[i].surface.window().id() == wid { return Window(i) }
                i += 1
            }
        }
        unimplemented!()
    }

    pub(crate) fn update_on_frame(&self) -> Result <(), WindowError> {
        self.data().previous_frame_end.as_mut().unwrap().cleanup_finished();
        if self.data().recreate_swapchain {
            let (swapchain, images) = match self.data().swapchain.recreate_with_dimensions(self.data().surface.window().inner_size().into()) {
                Ok(x) => x,
                Err(SwapchainCreationError::UnsupportedDimensions) => return Err(WindowError::RerunLoop),
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e)
            };

            self.data().swapchain = swapchain;
            self.data().framebuffers = window_update_on_resize(&images, self.data().renderpass.clone(), &mut self.data().dynamic_state);

            self.data().recreate_swapchain = false
        }
        Ok(())
    }
}
