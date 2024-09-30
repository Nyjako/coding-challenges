#version 150

uniform mat4 uModelViewProjection;

in vec4 ciPosition;
in vec2 ciTexCoord0;

out vec2 vTexCoord;

void main() {
    vTexCoord = ciTexCoord0;
    gl_Position = uModelViewProjection * ciPosition;
}