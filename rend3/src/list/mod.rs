pub use default::*;
use fnv::FnvHashMap;
pub use passes::*;
pub use resources::*;

mod default;
mod passes;
mod resources;

pub struct RenderList {
    pub(crate) sets: Vec<RenderPassSet>,
    pub(crate) shaders: FnvHashMap<String, ShaderSource>,
    pub(crate) images: FnvHashMap<String, ImageResourceDescriptor>,
    pub(crate) buffers: FnvHashMap<String, BufferResourceDescriptor>,
}

impl RenderList {
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
            shaders: FnvHashMap::default(),
            images: FnvHashMap::default(),
            buffers: FnvHashMap::default(),
        }
    }

    pub fn start_render_pass_set(&mut self, desc: RenderPassSetDescriptor) {
        self.sets.push(RenderPassSet {
            run_rate: desc.run_rate,
            render_passes: Vec::new(),
        })
    }

    pub fn create_shader(&mut self, name: impl ToString, shader: ShaderSource) {
        self.shaders.insert(name.to_string(), shader);
    }

    pub fn create_image(&mut self, name: impl ToString, image: ImageResourceDescriptor) {
        self.images.insert(name.to_string(), image);
    }

    pub fn create_buffer(&mut self, name: impl ToString, buffer: BufferResourceDescriptor) {
        self.buffers.insert(name.to_string(), buffer);
    }

    pub fn add_render_pass(&mut self, desc: RenderPassDescriptor) {
        self.sets
            .last_mut()
            .expect("Added render pass with no active render pass sets")
            .render_passes
            .push(RenderPass { desc, ops: Vec::new() });
    }

    pub fn add_render_op(&mut self, desc: RenderOpDescriptor) {
        self.sets
            .last_mut()
            .expect("Added render pass with no active render pass sets")
            .render_passes
            .last_mut()
            .expect("Added render op with no active render pass")
            .ops
            .push(desc);
    }
}

pub(crate) struct RenderPassSet {
    run_rate: RenderPassSetRunRate,
    render_passes: Vec<RenderPass>,
}

pub(crate) struct RenderPass {
    desc: RenderPassDescriptor,
    ops: Vec<RenderOpDescriptor>,
}
