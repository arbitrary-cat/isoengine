// Copyright (c) 2015, Sam Payson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#version 150

layout(points) in;

layout(triangle_strip, max_vertices = 4) out;

in FromVert {
    vec2 screen_BR;
    vec2 tex_TL;
    vec2 tex_BR;
    float depth;
} to_geo[];

out FromGeo {
    vec2  tex_coord;
    float depth;
} to_frag;

void main() {
    vec2  screen_TL = gl_in[0].gl_Position.xy;
    vec2  screen_BR = to_geo[0].screen_BR;
    vec2  tex_TL    = to_geo[0].tex_TL;
    vec2  tex_BR    = to_geo[0].tex_BR;
    float depth     = to_geo[0].depth;

    gl_Position       = vec4(screen_TL.x, screen_TL.y, 0, 1.0);
    to_frag.tex_coord = vec2(tex_TL.x, tex_TL.y);
    to_frag.depth     = depth;
    EmitVertex();

    gl_Position       = vec4(screen_BR.x, screen_TL.y, 0, 1.0);
    to_frag.tex_coord = vec2(tex_BR.x, tex_TL.y);
    to_frag.depth     = depth;
    EmitVertex();

    gl_Position       = vec4(screen_TL.x, screen_BR.y, 0, 1.0);
    to_frag.tex_coord = vec2(tex_TL.x, tex_BR.y);
    to_frag.depth     = depth;
    EmitVertex();

    gl_Position       = vec4(screen_BR.x, screen_BR.y, 0, 1.0);
    to_frag.tex_coord = vec2(tex_BR.x, tex_BR.y);
    to_frag.depth     = depth;
    EmitVertex();

    EndPrimitive();
}
