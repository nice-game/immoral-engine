use crate::vertex::VertexArray;
use gl::types::GLuint;
use std::ffi::c_void;

pub trait CommandBufferAbstract {
	fn vao(&self) -> &VertexArray;
	fn handle(&self) -> GLuint;
	fn len(&self) -> usize;
	fn indirect(&self) -> *const c_void;
}

pub struct CommandBuffer<'a> {
	vao: &'a VertexArray,
	cmds: Vec<DrawElementsIndirectCommand>,
}
impl<'a> CommandBuffer<'a> {
	pub fn new(vao: &'a VertexArray) -> Self {
		Self { vao, cmds: vec![] }
	}

	pub fn push(&mut self, count: u32, instance_count: u32, first_index: u32, base_vertex: u32, base_instance: u32) {
		self.cmds.push(DrawElementsIndirectCommand { count, instance_count, first_index, base_vertex, base_instance })
	}
}
impl<'a> CommandBufferAbstract for CommandBuffer<'a> {
	fn vao(&self) -> &VertexArray {
		self.vao
	}

	fn handle(&self) -> GLuint {
		0
	}

	fn len(&self) -> usize {
		self.cmds.len()
	}

	fn indirect(&self) -> *const c_void {
		self.cmds.as_ptr() as _
	}
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct DrawElementsIndirectCommand {
	pub count: u32,
	pub instance_count: u32,
	pub first_index: u32,
	pub base_vertex: u32,
	pub base_instance: u32,
}
