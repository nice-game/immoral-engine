use crate::Ctx;
use nalgebra::{Quaternion, Unit};
use simba::simd::SimdValue;
use std::rc::Rc;

use gl::types::{GLenum, GLint, GLuint};
use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, DimName, Scalar, VectorN};

#[macro_export]
macro_rules! implement_vertex {
	($struct:ident, $($field:ident),+) => {
		impl $crate::vertex::Vertex for $struct {
			fn format() -> Vec<$crate::vertex::VertexAttributeFormat> {
				fn glformat<T: $crate::vertex::VertexAttribute>(_: Option<&T>)
					-> ($crate::gl::types::GLint, $crate::gl::types::GLenum)
				{
					(<T as $crate::vertex::VertexAttribute>::size(), <T as $crate::vertex::VertexAttribute>::typ())
				}

				vec![ $( {
					let offset = $crate::memoffset::offset_of!($struct, $field) as _;
					let (size, typ) = glformat(None::<&$struct>.map(|x| &x.$field));
					$crate::vertex::VertexAttributeFormat { offset, size, typ }
				} ),+ ]
			}
		}
	};
}

macro_rules! implement_attribute {
	($ty:ty, $size:expr, $typ:expr) => {
		impl VertexAttribute for $ty {
			fn size() -> GLint {
				$size
			}

			fn typ() -> GLenum {
				$typ
			}
		}
	};
}

pub trait Vertex {
	fn format() -> Vec<VertexAttributeFormat>;
}

pub trait VertexAttribute {
	fn size() -> GLint;
	fn typ() -> GLenum;
}
implement_attribute!(u8, 1, gl::UNSIGNED_BYTE);
implement_attribute!(f32, 1, gl::FLOAT);
impl<N: Scalar + VertexAttribute, D: Dim + DimName> VertexAttribute for VectorN<N, D>
where
	DefaultAllocator: Allocator<N, D>,
{
	fn size() -> GLint {
		D::try_to_usize().unwrap() as _
	}

	fn typ() -> GLenum {
		N::typ()
	}
}
impl<N: Scalar + SimdValue + VertexAttribute> VertexAttribute for Quaternion<N> {
	fn size() -> GLint {
		4
	}

	fn typ() -> GLenum {
		N::typ()
	}
}
impl<T: VertexAttribute> VertexAttribute for Unit<T> {
	fn size() -> GLint {
		T::size()
	}

	fn typ() -> GLenum {
		T::typ()
	}
}

pub struct VertexAttributeFormat {
	pub offset: GLuint,
	pub size: GLint,
	pub typ: GLenum,
}

pub struct VertexArray {
	ctx: Rc<Ctx>,
	handle: GLuint,
	formats: Vec<Vec<VertexAttributeFormat>>,
	next_attrib: GLuint,
}
impl VertexArray {
	pub fn new(ctx: &Rc<Ctx>) -> Self {
		let mut handle = 0;
		unsafe { ctx.gl.CreateVertexArrays(1, &mut handle) };
		Self { ctx: ctx.clone(), handle, formats: vec![], next_attrib: 0 }
	}

	pub fn enable_vertices<V: Vertex>(&mut self, divisor: GLuint) {
		let format = V::format();
		let gl = &self.ctx.gl;
		for &VertexAttributeFormat { offset, size, typ } in &format {
			unsafe {
				gl.EnableVertexArrayAttrib(self.handle, self.next_attrib);
				gl.VertexArrayAttribFormat(self.handle, self.next_attrib, size, typ, gl::FALSE, offset);
				gl.VertexArrayAttribBinding(self.handle, self.next_attrib, self.formats.len() as _);
				gl.VertexArrayBindingDivisor(self.handle, self.next_attrib, divisor);
			}
		}
		self.formats.push(format);
		self.next_attrib += 1;
	}
}
impl Drop for VertexArray {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteVertexArrays(1, &self.handle) };
	}
}
