svg
===

__libsvg__ allow you to create svg graphics using Rust.

###A simple example from the SVG spec

```Rust
extern crate svg;

use std::old_io::{BufferedWriter, File, Truncate, ReadWrite};
use svg::SVG;

fn main() {
    // Create the SVG object
    let mut image = SVG::new(12, 4);
    image.view_box(0, 0, 1200, 400);
    // Add a little description
    image.desc("Example circle01 - circle filled with red and stroked with blue");
    // ... a rectangle
    image.rect(1, 1, 1198, 398, "fill=none stroke=blue stroke-width=2");
    // ... and circle
    image.circle(600, 200, 100, "fill=red stroke=blue stroke-width=10");

    // Create an ouput and export the svg image inside
    let mut output = BufferedWriter::new(File::open_mode(&Path::new("output.svg"),
                                         Truncate,
                                         ReadWrite)).unwrap();
    image.finalize(&mut output);
}

```

This is the simple way to use __libsvg__. Inside the SVG object a Shape struct is
create to represent each object. You can too create a Shape, modify it, and add it
to the image using the method `SVG::add()`.
