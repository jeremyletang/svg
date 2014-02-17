// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[deriving(Clone, Show)]
pub struct Transform {
    priv output: ~str
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            output: ~"transform=\"\""
        }
    }
    
    fn insert(&mut self, mut tr: ~str) {
        let len = self.output.len();
        if self.output.char_at(len - 2) != '\"' { tr.insert_char(0, ' ') }
        self.output.insert(len - 1, tr)
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.insert(format!("translate({}, {})", x, y))
    }

    pub fn rotate(&mut self, angle: i32) {
        self.insert(format!("rotate({})", angle))
    }

    pub fn skew_x(&mut self, factor: i32) {
        self.insert(format!("skewX({})", factor))
    }

    pub fn skew_y(&mut self, factor: i32) {
        self.insert(format!("skewY({})", factor))
    }

    pub fn scale(&mut self, x_scale: i32, y_scale: i32) {
        self.insert(format!("scale({}, {})", x_scale, y_scale))
    }

    pub fn get(&self) -> ~str {
        self.output.clone()
    }
}
