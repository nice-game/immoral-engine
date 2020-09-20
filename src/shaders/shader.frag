#version 420 core

in vec3 WorldPosition;
in vec3 WorldNormal;
in vec4 UVMapping;

out vec4 FragColor;

void main() {
	FragColor = vec4(UVMapping.xy, 0.0, 1.0);
}
