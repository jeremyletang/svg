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

#[crate_id = "svg_test#0.0.1"];

extern crate svg;

use std::io::{BufferedWriter, File, Truncate, ReadWrite};
use std::hashmap::HashMap;

use svg::Svg;
use svg::Transform;
// use svg::{Circle, Rect, RoundedRect};

pub fn main() {
	let mut output = BufferedWriter::new(File::open_mode(&Path::new("output.svg"), Truncate, ReadWrite)).unwrap();
	let mut image = Svg::new(12, 12);
	let mut attribs = HashMap::new();
	attribs.insert(~"fill", ~"green");
	attribs.insert(~"stroke", ~"orange");
	attribs.insert(~"stroke-width", ~"2");

	let mut t = Transform::new();
	t.translate(100, 200);
	t.translate(10, 32);
	println!("{}", t.get());

	image.view_box(0, 0, 1200, 400);
	image.circle(600, 200, 100, "fill=red stroke=blue stroke-width=10");
	image.rect(700, 200, 200, 200, "fill=red stroke=blue stroke-width=10");
	image.rounded_rect(800, 600, 200, 200, 60, 30, "fill=red stroke=blue stroke-width=10");
	image.set_title("Svg library test Main !");
	image.set_desc("A simple main test for the rust svg generation library");

	match image.finalize(&mut output) {
		Ok(_) 		=> {},
		Err(err) 	=> fail!(err)
	}
}