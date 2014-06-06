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

#![crate_id = "svg_test#0.0.1"]

extern crate svg;

use std::io::{BufferedWriter, File, Truncate, ReadWrite};
use std::collections::HashMap;

use svg::SVG;
use svg::Transform;
// use svg::{Circle, Rect, RoundedRect};

pub fn main() {
   let mut output = BufferedWriter::new(File::open_mode(&Path::new("output.svg"), Truncate, ReadWrite)).unwrap();
   let mut image = SVG::new(12, 12);
   let mut attribs = HashMap::new();
   let polygon_points = vec!((350,75),  (379,161), (469,161), (397,215),
                          (423,301), (350,250), (277,301), (303,215),
                          (231,161), (321,161));
   attribs.insert(String::from_str("fill"), String::from_str("green"));
   attribs.insert(String::from_str("stroke"), String::from_str("orange"));
   attribs.insert(String::from_str("stroke-width"), String::from_str("2"));

   let mut t = Transform::new();
   t.translate(100, 200);
   t.translate(10, 32);
   println!("{}", t.get());

   image.view_box(0, 0, 1200, 400);
   image.g_begin(Some("First_Group"), Some(&t), Some(&attribs));
   image.g_begin(Some("First_Group"), Some(&t), Some(&attribs));
   // image.g_transform(t.clone());
   //image.g_rotate(15);
   image.circle(600, 200, 100, "id=jojo fill=red stroke=blue stroke-width=10");
   image.rect(700, 200, 200, 200, "fill=red stroke=blue stroke-width=10");
   image.rounded_rect(800, 600, 200, 200, 60, 30, "fill=red stroke=blue stroke-width=10");
   image.polygon(&polygon_points, "fill=red stroke=blue stroke-width=10");
   image.g_end();
   image.g_end();
   image.title("Svg library test Main !");
   image.desc("A simple main test for the rust svg generation library");

    match image.finalize(&mut output) {
        Ok(_)       => {},
        Err(err)    => fail!(err)
   }
}