#version 430

layout (location = 0) in vec3 aPos;

layout (std140, binding = 0) uniform Camera {
	vec4 rot;
	vec3 pos;
} cam;

vec3 quat_mul(vec4 quat, vec3 vec) {
	return cross(quat.xyz, cross(quat.xyz, vec) + vec * quat.w) * 2.0 + vec;
}

void main() {
	// vec3 pos = quat_mul(cam.rot, aPos) - cam.pos;
	gl_Position = vec4(aPos, 1.0);
}
