use crate::{Vec3, Vec4, Color};
use super::Drawable;
use std::sync::Arc;
use once_cell::sync::OnceCell;
use vulkano::{
    buffer::{CpuAccessibleBuffer, BufferUsage},
    command_buffer::{AutoCommandBufferBuilder, DynamicState},
    framebuffer::{RenderPassAbstract, Subpass},
    pipeline::{
        GraphicsPipeline,
        vertex::SingleBufferDefinition
    },
    descriptor::PipelineLayoutAbstract
};

#[derive(Default, Clone)]
pub struct Vertex {
    pos:   Vec3 <f32>,
    color: Vec4 <f32>
}

impl Vertex {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn position(mut self, pos: Vec3 <f32>) -> Self {
        self.pos = pos;
        self
    }

    #[inline]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color.into();
        self
    }
}

vulkano::impl_vertex!(Vertex, pos, color);

#[derive(Clone)]
pub struct Shape {
    vxs: Vec <Vertex>
}

impl Shape {
    #[inline]
    pub const fn new() -> Self {
        Self { vxs: Vec::new() }
    }

    #[inline]
    pub fn vertex(mut self, vertex: Vertex) -> Self {
        self.vxs.push(vertex);
        self
    }

    #[inline]
    pub fn build(self) -> Arc <BuiltShape> {
        Arc::new(BuiltShape::new(self))
    }
}

#[derive(Clone)]
pub struct BuiltShape {
    buf: Arc <CpuAccessibleBuffer <[Vertex]>>
}

impl BuiltShape {
    pub(crate) fn new(shape: Shape) -> Self {
        Self {
            buf: CpuAccessibleBuffer::from_iter(
                crate::init::Stt::device().clone(),
                BufferUsage::all(),
                false,
                shape.vxs.iter().cloned()
            ).unwrap()
        }
    }
}

impl Drawable for BuiltShape {
    fn draw(&self, builder: &mut AutoCommandBufferBuilder, renderpass: Arc <dyn RenderPassAbstract + Send + Sync>, dynamic: &mut DynamicState) {
        static PIPELINE: OnceCell <Arc <GraphicsPipeline <SingleBufferDefinition <Vertex>, Box <dyn PipelineLayoutAbstract + Send + Sync>, Arc <dyn RenderPassAbstract + Send + Sync>>>> = OnceCell::new();
        builder.draw(PIPELINE.get_or_init(|| {
            mod vs {
                vulkano_shaders::shader! {
                    ty: "vertex",
                    src: "
                        #version 450
                        layout(location = 0) in vec3 pos;
                        layout(location = 1) in vec4 color;
                        layout(location = 2) out vec4 f_color;
                        void main() {
                            gl_Position = vec4(pos, 1.0);
                            f_color = color;
                        }
                    "
                }
            }

            mod fs {
                vulkano_shaders::shader! {
                    ty: "fragment",
                    src: "
                        #version 450
                        layout(location = 0) out vec4 color;
                        layout(location = 2) in vec4 f_color;
                        void main() {
                            color = f_color;
                        }
                    "
                }
            }

            let vs = vs::Shader::load(crate::init::Stt::device().clone()).unwrap();
            let fs = fs::Shader::load(crate::init::Stt::device().clone()).unwrap();

            Arc::new(GraphicsPipeline::start()
                .vertex_input_single_buffer()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(renderpass.clone(), 0).unwrap())
                .build(crate::init::Stt::device().clone())
                .unwrap())
        }).clone(), dynamic, self.buf.clone(), (), ()).unwrap();
    }
}
