#version 420 core

in float TextureIndex;
in vec3 WorldPosition;
in vec3 WorldNormal;
in vec4 UVMapping;

out vec4 FragColor;

uniform sampler2DArray tex;

void main() {
	if (TextureIndex == -1) {
		FragColor = vec4(1, 0.1, 0.1, 1);
	} else {
		FragColor = texture(tex, vec3(UVMapping.xy, TextureIndex));
	}
}
