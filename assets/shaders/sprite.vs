#version 140

in vec2 position;
in vec2 uv;
out vec2 v_uv;

uniform vec2 projection;

void main() {
    v_uv = uv;
    gl_Position = vec4(position * projection, 0.0, 1.0);
}
