use ffi::{AiMesh, AiVector3D, AiBone, AiVertexWeight, AiColor4D};

use math::vector3::{Vector3D, Vector3DIter};
use math::color4::{Color4D, Color4DIter};
use super::face::{Face, FaceIter};

use math::Matrix4x4;

define_type_and_iterator_indirect! {
    /// Mesh type (incomplete)
    struct Mesh(&AiMesh)
    /// Mesh iterator type.
    struct MeshIter
}

define_type_and_iterator_indirect! {
    /// Bone type
    struct Bone(&AiBone)
    /// Bone iterator type.
    struct BoneIter
}

define_type_and_iterator_indirect! {
    /// Vertex weight type
    struct VertexWeight(&AiVertexWeight)
    /// Vertex weight iterator type.
    struct VertexWeightIter
}

impl<'a> Mesh<'a> {
    // TODO return as PrimitiveType enum
    pub fn primitive_types(&self) -> u32 {
        self.primitive_types
    }

    pub fn num_vertices(&self) -> u32 {
        self.num_vertices
    }

    pub fn vertex_iter(&self) -> Vector3DIter {
        Vector3DIter::new(self.vertices,
                          self.num_vertices as usize)
    }

    pub fn get_vertex(&self, id: u32) -> Option<Vector3D> {
        self.vertex_data(self.vertices, id)
    }

    pub fn normal_iter(&self) -> Vector3DIter {
        Vector3DIter::new(self.normals,
                          self.num_vertices as usize)
    }

    pub fn get_normal(&self, id: u32) -> Option<Vector3D> {
        self.vertex_data(self.normals, id)
    }

    pub fn tangent_iter(&self) -> Vector3DIter {
        Vector3DIter::new(self.tangents,
                          self.num_vertices as usize)
    }

    pub fn get_tangent(&self, id: u32) -> Option<Vector3D> {
        self.vertex_data(self.tangents, id)
    }

    pub fn bitangent_iter(&self) -> Vector3DIter {
        Vector3DIter::new(self.bitangents,
                          self.num_vertices as usize)
    }

    pub fn get_bitangent(&self, id: u32) -> Option<Vector3D> {
        self.vertex_data(self.bitangents, id)
    }

    pub fn vertex_color_iter(&self, set_id: usize) -> Color4DIter {
        Color4DIter::new(self.colors[set_id],
                         self.num_vertices as usize)
    }

    pub fn get_vertex_color(&self, set_id: usize, id: u32) -> Option<Color4D> {
        self.color_data(self.colors[set_id], id)
    }

    pub fn texture_coords_iter(&self, channel_id: usize) -> Vector3DIter {
        Vector3DIter::new(self.texture_coords[channel_id],
                          self.num_vertices as usize)
    }

    pub fn get_texture_coord(&self, channel_id: usize, id: u32) -> Option<Vector3D> {
        self.vertex_data(self.texture_coords[channel_id], id)
    }

    pub fn num_faces(&self) -> u32 {
        self.num_faces
    }

    pub fn face_iter(&self) -> FaceIter {
        FaceIter::new(self.faces,
                      self.num_faces as usize)
    }

    pub fn get_face(&self, id: u32) -> Option<Face> {
        if id < self.num_faces {
            unsafe { Some(Face::from_raw(self.faces.offset(id as isize))) }
        } else {
            None
        }
    }

    pub fn num_bones(&self) -> u32 {
        self.num_bones
    }

    pub fn bone_iter(&self) -> BoneIter {
        BoneIter::new(self.bones as *const *const AiBone,
                      self.num_bones as usize)
    }

    pub fn get_bone(&self, id: u32) -> Option<Bone> {
        if id < self.num_bones {
            unsafe { Some(Bone::from_raw(*(self.bones.offset(id as isize)))) }
        } else {
            None
        }
    }

    #[inline]
    fn vertex_data(&self, array: *mut AiVector3D, id: u32) -> Option<Vector3D> {
        if id < self.num_vertices {
            unsafe { Some(Vector3D::from_raw(array.offset(id as isize))) }
        } else {
            None
        }
    }

    #[inline]
    fn color_data(&self, array: *mut AiColor4D, id: u32) -> Option<Color4D> {
        if id < self.num_vertices {
            unsafe { Some(Color4D::from_raw(array.offset(id as isize))) }
        } else {
            None
        }
    }
}

impl<'a> Bone<'a> {
    /// Returns the name of the bone.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the bones's offset transformation matrix.
    pub fn offset_matrix(&self) -> Matrix4x4 {
        Matrix4x4::from_raw(&self.offset_matrix)
    }

    pub fn num_weights(&self) -> u32 {
        self.num_weights
    }

    pub fn weight_iter(&self) -> VertexWeightIter {
        VertexWeightIter::new(self.weights as *const *const AiVertexWeight,
                      self.num_weights as usize)
    }

    pub fn get_weight(&self, id: u32) -> Option<VertexWeight> {
        if id < self.num_weights {
            unsafe { Some(VertexWeight::from_raw(self.weights.offset(id as isize))) }
        } else {
            None
        }
    }
}
