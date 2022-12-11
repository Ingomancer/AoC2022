use std::{fs::File, io::BufWriter, path::Path};

pub fn run(input: String) -> (String, String) {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut display = vec!["\n".to_owned()];
    let mut row = String::new();
    let mut cell = -1;
    for line in input.lines() {
        let ticks;
        let mut num = 0;
        if let Some((cmd, add)) = line.split_once(" ") {
            num = add.parse().unwrap();
            ticks = 2;
        } else {
            ticks = 1;
        }
        for i in 0..ticks {
            cycle += 1;
            cell += 1;
            if interesting_cycles.contains(&cycle) {
                sum += x * cycle;
            }
            if (cell - x).abs() < 2 {
                row += "#";
            } else {
                row += ".";
            }
            if (cycle % 40) == 0 {
                cell = -1;
                display.push(row);
                row = String::new();
            }
        }
        x += num;
    }
    let path = "word.png";
    make_png(display.clone(), path);
    let letters = read_image(path);
    (format!("{}", sum).to_owned(), letters.to_owned())
}

fn make_png(display: Vec<String>, path: &str) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(
        w,
        display[1].len().try_into().unwrap(),
        (display.len() - 1).try_into().unwrap(),
    ); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    let mut data = vec![]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    for line in display.iter().skip(1) {
        for pixel in line.chars() {
            if pixel == '#' {
                data.push(255);
                data.push(255);
                data.push(255);
            } else {
                data.push(0);
                data.push(0);
                data.push(0);
            }
            data.push(255);
        }
    }
    writer.write_image_data(&data).unwrap();
}

fn read_image(path: &str) -> String {
    tesseract::ocr(path, "eng").unwrap()
}
