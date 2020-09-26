#version 420 core

layout (location = 0) in int TextureIndex;
layout (location = 1) in vec3 VertexPosition;
layout (location = 2) in vec4 VertexRotation;
layout (location = 3) in vec4 VertexUVMapping;
layout (location = 4) in vec4 VertexBoneIDs;
layout (location = 5) in vec4 VertexBoneWeights;

out vec3 WorldPosition;
out vec3 WorldNormal;
out vec4 UVMapping;

layout (std140, binding = 0) uniform Camera {
	vec4 proj;
	vec4 rot;
	vec3 pos;
} cam;

vec4 quat_inv(vec4 q) {
	return vec4(q.xyz, -q.w);
}

vec3 quat_mul(vec4 quat, vec3 vec) {
	return cross(quat.xyz, cross(quat.xyz, vec) + vec * quat.w) * 2.0 + vec;
}

vec4 perspective(vec4 Projection, vec3 Position) {
	return vec4(Position.xy * Projection.xy, Position.z * Projection.z + Projection.w, -Position.z);
}

void main() {
	WorldPosition = VertexPosition; // FIXME: apply model transform
	WorldNormal = quat_mul(VertexRotation, vec3(0.0, 0.0, 1.0));
	UVMapping = VertexUVMapping;
	vec3 EyePosition = quat_mul(quat_inv(cam.rot), WorldPosition - cam.pos);
	gl_Position = perspective(cam.proj, vec3(EyePosition.xz, -EyePosition.y));
}
