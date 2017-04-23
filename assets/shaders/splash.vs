#version 140

in vec2 offset;
in vec2 uv;
out vec2 v_uv;

void main() {
    v_uv = uv;
    vec2 aspect = vec2(1.0, (256.0 / 144.0));
    gl_Position = vec4(offset * aspect * 2.0, 0.0, 1.0);
}
