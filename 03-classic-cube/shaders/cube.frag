#version 150

in vec2 vTexCoord;
out vec4 oColor;

void main() {
    // Simple color based on texture coordinates for a cool effect
    oColor = vec4(abs(sin(vTexCoord.x * 3.14159)), abs(sin(vTexCoord.y * 3.14159)), 0.5, 1.0);
}