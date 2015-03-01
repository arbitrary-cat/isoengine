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
    vec2 dimens;
} to_geo[];

void main() {
    float tl_x = gl_in[0].gl_Position.x;
    float tl_y = gl_in[0].gl_Position.y;

    gl_Position = vec4(tl_x, tl_y, 0, 1.0);
    EmitVertex();

    gl_Position = vec4(tl_x + to_geo[0].dimens.x, tl_y, 0, 1.0);
    EmitVertex();

    gl_Position = vec4(tl_x, tl_y - to_geo[0].dimens.y, 0, 1.0);
    EmitVertex();

    gl_Position = vec4(tl_x + to_geo[0].dimens.x, tl_y - to_geo[0].dimens.y, 0, 1.0);
    EmitVertex();

    EndPrimitive();
}
