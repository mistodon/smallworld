#version 140

in vec2 offset;
in vec2 uv;
out vec2 v_uv;

uniform vec2 projection;
uniform vec2 position;

void main() {
    v_uv = uv;
    gl_Position = vec4((position + offset) * projection, 0.0, 1.0);
}
