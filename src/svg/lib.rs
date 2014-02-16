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

#[crate_id = "svg#0.0.1"];
#[desc = "SVG generation in Rust"];
#[license = "MIT"];
#[crate_type = "dylib"];
#[crate_type = "rlib"];
// #[warn(missing_doc)];
#[allow(dead_code)];

use std::io::{Writer, IoResult};
use std::hashmap::HashMap;

pub use shapes::{Circle, Rect, RoundedRect};

mod shapes;

static DOC_TYPE: &'static str = "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
static XMLNS: &'static str = "version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" \
xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n";
static STANDALONE_YES: &'static str = "<?xml version=\"1.0\" standalone=\"yes\"?>\n";
static STANDALONE_NO: &'static str = "<?xml version=\"1.0\" standalone=\"no\"?>\n";

trait SVGEntity {
    fn gen_output(&self) -> ~str;
}

pub fn rgb(red: u32,
           green: u32, 
           blue: u32) -> ~str {
    format!("rgb({}, {}, {})", red, green, blue)
}

pub fn rgba(red: u32, 
            green: u32, 
            blue: u32, 
            alpha: f32) -> ~str {
    format!("rgba({}, {}, {}, {})", red, green, blue, alpha)
}

struct Head {
    standalone: bool,
    width: i32,
    height: i32,
    view_box: Option<(i32, i32, i32, i32)>,
    
}

impl Head {
    pub fn new(width: i32, height: i32) -> Head {
        Head {
            standalone: false,
            width: width,
            height: height,
            view_box: None
        }
    }
}

pub struct Svg<'a> {
    priv head: Head,
    priv content: ~str
}

fn make_attribs(attribs: &str) -> HashMap<~str, ~str>{
    let mut h = HashMap::new();
    for s in attribs.split(' ') {
        let t: ~[&str] = s.split('=').collect();
        h.insert(t[0].to_owned(), t[1].to_owned());
    }
    h
}

impl<'a> Svg<'a> {
    pub fn new(width: i32, height: i32) -> Svg {
        Svg {
            head: Head::new(width, height),
            content: ~""
        }
    }
    
    pub fn standalone(&mut self, is_standalone: bool) {
        self.head.standalone = is_standalone;
    }

    pub fn view_box(&mut self, 
                    orig_x: i32, 
                    orig_y: i32, 
                    width: i32, 
                    height: i32) {
        self.head.view_box = Some((orig_x, orig_y, width, height));
    }

    pub fn add<T: SVGEntity>(&mut self, new_entity: &T) {
        self.content.push_str(new_entity.gen_output());
    }

    pub fn circle(&mut self, 
                  x: i32, 
                  y: i32, 
                  radius: u32, 
                  attribs: &str) {
        self.content.push_str(Circle {
                x: x,
                y: y,
                radius: radius,
                attribs: make_attribs(attribs)
            }.gen_output())
    }
    
    pub fn rect(&mut self, 
                x: i32, 
                y: i32, 
                width: i32, 
                height: i32, 
                attribs: &str) {
        self.content.push_str(Rect {
                x: x,
                y: y,
                width: width,
                height: height,
                attribs: make_attribs(attribs)
            }.gen_output())
    }
    
    pub fn rounded_rect(&mut self, 
                        x: i32, 
                        y: i32, 
                        width: i32, 
                        height: i32, 
                        round_x: u32, 
                        round_y: u32, 
                        attribs: &str) {
        self.content.push_str(RoundedRect {
                x: x,
                y: y,
                width: width,
                height: height,
                round_x: round_x,
                round_y: round_y,
                attribs: make_attribs(attribs)
            }.gen_output())
    }
    
    pub fn finalize(&mut self, output: &'a mut Writer) -> IoResult<()>{
        let mut o = ~"";
        
        // Head
        match self.head.standalone {
            true    => o.push_str(STANDALONE_YES),
            false   => o.push_str(STANDALONE_NO)
        };
        o.push_str(DOC_TYPE);
        o.push_str(format!("<svg width=\"{}cm\" height=\"{}cm\" ", 
                           self.head.width, self.head.height));
        match self.head.view_box {
            Some((x, y, width, height)) => {
                o.push_str(format!("viewBox=\"{} {} {} {}\" ", x, y, width, height))
            },
            None                        => {/* nothing to do */}
        }
        o.push_str(XMLNS);
        
        // Body
        o.push_str(self.content);
        
        // Close
        o.push_str(&"</svg>\n");
        output.write_str(o)
    }
}
