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

in FromGeo {
    vec2  tex_coord;
    float depth;
};

out vec4 color;

uniform sampler2D color_tex;
uniform sampler2D depth_tex;


void main() {

    // Configurable constant.
    // We will deal with things that are at most `max_depth` meters from the camera.
    float max_depth = 100.0;


    // Configurable constant.
    // The units of the depth texture, how far in meters is the origin from the camera?
    float depth_scale = 5.0;

    float depth_sample = texture(depth_tex, tex_coord).r - 0.5;
    vec4  color_sample = texture(depth_tex, tex_coord);

    if (depth_sample > 0.48 || color_sample.a < 0.5) {
        discard;
    }

    gl_FragDepth = (depth_sample*depth_scale + depth) / max_depth;

    color = color_sample;
}
