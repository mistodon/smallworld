#version 140

in vec2 v_uv;
out vec4 color;

uniform sampler2D colormap;

void main() {
    vec4 pix = texture(colormap, v_uv);
    color = pix;
}
