mod maze;

use maze::{Maze, Cell};
use std::path::{PathBuf};
use std::fs::File;
use std::io::BufWriter;
use png::Encoder;


fn main() {
  let m = Maze::prim_random(99, 99);
  let mut filen = PathBuf::new();
  filen.push("image");

  write_maze_image(&mut filen, &m.end, &m.grid);
}

fn write_maze_image(filename: &mut PathBuf, end: &Cell, maze: &Vec<Vec<bool>>) {
  filename.set_extension("png");
  let file = File::create(filename.as_path()).unwrap();
  let ref mut w = BufWriter::new(file);

  let mut encoder = Encoder::new(w, 101, 101);
  encoder.set_color(png::ColorType::Rgb);
  encoder.set_depth(png::BitDepth::Eight);
  encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8));
  encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
  encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
  let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
    (0.31270, 0.32900),
    (0.64000, 0.33000),
    (0.30000, 0.60000),
    (0.15000, 0.06000)
  );
  encoder.set_source_chromaticities(source_chromaticities);
  let mut writer = encoder.write_header().unwrap();
  let mut data: Vec<u8> = Vec::new();

  for _ in 0..maze[0].len() + 2 {
    data.push(0);
    data.push(0);
    data.push(0);
  }

  for row in maze {
    data.push(0);
    data.push(0);
    data.push(0);
    for c in row {
      if *c {
        data.push(255);
        data.push(255);
        data.push(255);
      } else {
        data.push(0);
        data.push(0);
        data.push(0);
      }
    }
    data.push(0);
    data.push(0);
    data.push(0);
  }

  for _ in 0..maze[0].len() + 2 {
    data.push(0);
    data.push(0);
    data.push(0);
  }

  writer.write_image_data(data.as_slice()).unwrap();

}