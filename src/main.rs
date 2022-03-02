mod maze;

use maze::{Maze, Color};
use std::convert::TryInto;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::Encoder;
use clap::{App, Arg};

fn main() {

  let matches = App::new("Maze generator")
                              .version("0.1.0")
                              .author("Computer Science Student")
                              .about("Generates mazes in png format")
                              .arg(Arg::with_name("filename")
                                    .short("f")
                                    .long("file")
                                    .value_name("FILE")
                                    .help("Output png with this filename")
                                    .takes_value(true)
                                    .default_value("maze"))
                              .arg(Arg::with_name("width")
                                    .short("w")
                                    .long("width")
                                    .value_name("INT")
                                    .help("Number of horizontal cells")
                                    .takes_value(true)
                                    .default_value("201"))
                              .arg(Arg::with_name("height")
                                    .short("h")
                                    .long("height")
                                    .value_name("INT")
                                    .help("Number of vertical cells")
                                    .takes_value(true)
                                    .default_value("201"))
                              .get_matches();
  let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
  let height = matches.value_of("height").unwrap().parse::<u32>().unwrap();

  if width > 1000 || height > 1000 {
    panic!("Dimensions too large")
  }

  let file = matches.value_of("filename").unwrap();
  
  let m = Maze::prim_random((width-2).try_into().unwrap(), (height-2).try_into().unwrap());

  write_maze_image("maze.png", &m.to_bitmap(false), width, height);
  write_maze_image("maze_solved.png", &m.to_bitmap(true), width, height);
}

fn write_maze_image(filename: &str, maze: &Vec<Color>, width: u32, height: u32) {
  let file = File::create(Path::new(filename)).unwrap();
  let ref mut w = BufWriter::new(file);

  let mut encoder = Encoder::new(w, width, height);
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

  for Color(r, g, b) in maze {
    data.push(*r);
    data.push(*g);
    data.push(*b);
  }

  writer.write_image_data(data.as_slice()).unwrap();

}