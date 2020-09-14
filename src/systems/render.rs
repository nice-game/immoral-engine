use crate::{components::mesh::Mesh, Ctx};
use gl::types::GLuint;
use specs::{prelude::*, System};
use std::{ffi::CString, iter::repeat, ptr, sync::Arc};

pub struct Render {
	ctx: Arc<Ctx>,
	shader: GLuint,
}
impl Render {
	pub fn new(ctx: &Arc<Ctx>) -> Self {
		unsafe {
			let src = CString::new(include_str!("../shaders/shader.vert")).unwrap();
			let vshader = ctx.gl.CreateShader(gl::VERTEX_SHADER);
			ctx.gl.ShaderSource(vshader, 1, [src.as_ptr()].as_ptr(), ptr::null());
			ctx.gl.CompileShader(vshader);
			check_shader(ctx, vshader);

			let src = CString::new(include_str!("../shaders/shader.frag")).unwrap();
			let fshader = ctx.gl.CreateShader(gl::FRAGMENT_SHADER);
			ctx.gl.ShaderSource(fshader, 1, [src.as_ptr()].as_ptr(), ptr::null());
			ctx.gl.CompileShader(fshader);
			check_shader(ctx, fshader);

			let shader = ctx.gl.CreateProgram();
			ctx.gl.AttachShader(shader, vshader);
			ctx.gl.AttachShader(shader, fshader);
			ctx.gl.LinkProgram(shader);
			check_program(ctx, shader);

			ctx.gl.DeleteShader(fshader);
			ctx.gl.DeleteShader(vshader);

			Self { ctx: ctx.clone(), shader }
		}
	}
}
impl<'a> System<'a> for Render {
	type SystemData = ReadStorage<'a, Mesh>;

	fn run(&mut self, meshes: Self::SystemData) {
		unsafe {
			self.ctx.gl.UseProgram(self.shader);
			for mesh in meshes.join() {
				self.ctx.gl.DrawArrays(gl::TRIANGLES, 0, 3);
				self.ctx.gl.BindVertexArray(mesh.vao);
			}
		}
	}
}

unsafe fn check_shader(ctx: &Ctx, shader: GLuint) {
	let mut success = -1;
	ctx.gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
	if success == 0 {
		let mut len = 0;
		ctx.gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
		let mut info_log: String = repeat('\0').take(len as _).collect();
		ctx.gl.GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as _);
		panic!("{:?}", info_log);
	}
}

unsafe fn check_program(ctx: &Ctx, program: GLuint) {
	let mut success = -1;
	ctx.gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
	if success == 0 {
		let mut len = 0;
		ctx.gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
		let mut info_log: String = repeat('\0').take(len as _).collect();
		ctx.gl.GetShaderInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as _);
		panic!("{:?}", info_log);
	}
}
