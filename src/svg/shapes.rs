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

use std::hashmap::HashMap;

use SVGEntity;

#[deriving(Show, Eq, ToStr)]
pub struct Circle {
    x: i32,
    y: i32,
    radius: u32,
    attribs: HashMap<~str, ~str>
}

#[deriving(Show, Eq, ToStr)]
pub struct RoundedRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    round_x: u32,
    round_y: u32,
    attribs: HashMap<~str, ~str>
}

#[deriving(Show, Eq, ToStr)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    attribs: HashMap<~str, ~str>
}

fn insert_attribs(mut o: ~str, attribs: &HashMap<~str, ~str>) -> ~str {
    for (at, value) in attribs.iter() {
        o.push_str(format!(" {}=\"{}\"", *at, *value))
    }
    o.push_str(" />\n");
    o
}

impl SVGEntity for Circle {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\"",
                           self.x, self.y, self.radius));
        insert_attribs(o, &self.attribs)
    }
}

impl SVGEntity for Rect {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
                           self.x, self.y, self.width, self.height));
        insert_attribs(o, &self.attribs)
    }
}

impl SVGEntity for RoundedRect {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" \
                           rx=\"{}\" ry=\"{}\"",
                           self.x, self.y, self.width, self.height, 
                           self.round_x, self.round_y));
        insert_attribs(o, &self.attribs)
    }
}
