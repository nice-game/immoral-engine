#version 430

layout (location = 0) in vec3 aPos;

layout (std140, binding = 0) uniform Camera {
	vec4 rot;
	vec3 pos;
} cam;

void main() {
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
