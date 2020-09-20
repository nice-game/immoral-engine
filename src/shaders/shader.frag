#version 420 core

in vec3 WorldPosition;
in vec3 WorldNormal;

out vec4 FragColor;

void main() {
	FragColor = vec4(WorldNormal, 1.0f);
}
