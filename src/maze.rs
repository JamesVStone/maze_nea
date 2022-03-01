use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};


// 2D Maze Object
pub struct Maze {
  pub end: Cell,
  pub grid: Vec<Vec<bool>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell(usize, usize);

impl Cell {
  fn random_cell(width: usize, height: usize, rng: &mut ThreadRng) -> Self {
    Self(rng.gen_range(0, width-1), rng.gen_range(0, height-1))
  }
  fn opposite(&self, width: usize, height: usize, parent: Cell) -> Option<Cell> {
    let Cell(x, y) = *self;
    let Cell(px, py) = parent;

    // vertical
    if px == x {
      if y > 0 && py == y-1 && y < height-1 {
        return Some(Cell(x, y+1))
      } else if y > 0 {
        return Some(Cell(x, y-1))
      }
    } else if py == y {
      if x > 0 && px == x-1 && x < width -1 {
        return Some(Cell(x+1, y))
      } else if x > 0 {
        return Some(Cell(x-1, y))
      }
    }

    None
  }
}

impl Maze {
  pub fn prim_random(width: usize, height: usize) -> Self {
    let mut grid: Vec<Vec<bool>> = Vec::with_capacity(height);
    let mut frontier: Vec<Cell> = Vec::new();
    let mut rng = thread_rng();
    let mut end = Cell(0, 0);

    // initialize grid
    for i in 0..height {
      grid.push(Vec::with_capacity(width));
      for _ in 0..width {
        grid[i].push(false)
      }
    }
    
    // select random starting position on grid
    //let Cell(x, y) = Cell::random_cell(width, height, &mut rng);
    let Cell(x, y) = Cell(0, 0);
    grid[y][x] = true;
    println!("Start location ({} {})", x, y);


    // add walls to vec
    if x > 0 {
      frontier.push(Cell(x-1, y));
    }
    if x < width - 1 {
      frontier.push(Cell(x+1, y));
    }
    if y > 0 {
      frontier.push(Cell(x, y-1));
    }
    if y < height - 1 {
      frontier.push(Cell(x, y+1));
    }


    while !frontier.is_empty() {
      // get random wall from frontier
      let mut nwall = 0;
      if frontier.len() > 1 {
        nwall = rng.gen_range(0, frontier.len()-1);
      }
      let wall = frontier[nwall];
      let Cell(x, y) = wall;
      let mut s: Option<Cell> = None;

      if x > 0 && grid[y][x-1] {
        s = Some(Cell(x-1, y))
      }
      if x < width - 1 && grid[y][x+1] {
        s = Some(Cell(x+1, y))
      }
      if y > 0 && grid[y-1][x] {
        s = Some(Cell(x, y-1))
      }
      if y < height - 1 && grid[y+1][x] {
        s = Some(Cell(x, y+1))
      }

      if let Some(parent) = s {
        let op = wall.opposite(width, height, parent);
        if let Some(Cell(nx, ny)) = op {
          if !grid[y][x] && !grid[ny][nx] {
            grid[y][x] = true;
            grid[ny][nx] = true;

            end = Cell(nx, ny);

            if nx > 0 {
              frontier.push(Cell(nx-1, ny));
            }
            if nx < width - 1 {
              frontier.push(Cell(nx+1, ny));
            }
            if ny > 0 {
              frontier.push(Cell(nx, ny-1));
            }
            if ny < height - 1 {
              frontier.push(Cell(nx, ny+1));
            }
          }
        }
      }

      /*
      println!("({}, {}) adding neighbour ({}, {})", x, y, nx, ny);
      grid[ny][nx] = true;

      println!("{:?}", frontier);
      if nx > 0 { if !grid[ny][nx-1] { frontier.push(Cell(nx-1, ny))} }
      if nx < width-1 { if !grid[ny][nx+1] { frontier.push(Cell(nx+1, ny))} }
      if ny > 0 { if !grid[ny-1][nx] { frontier.push(Cell(nx, ny-1))} }
      if ny < height-1 { if !grid[ny+1][nx] { frontier.push(Cell(nx, ny+1))} }
      println!("{:?}", frontier);*/

      frontier.remove(nwall);
    }

    Self {
      end,
      grid
    }
  }
}

#[test]
fn test_cell_opposite_right() {
  let c = Cell(10, 10);
  let p = Cell(9, 10);

  assert_eq!(c.opposite(100, 100, p), Some(Cell(11, 10)));
}

#[test]
fn test_cell_opposite_down() {
  let c = Cell(10, 10);
  let p = Cell(10, 9);

  assert_eq!(c.opposite(100, 100, p), Some(Cell(10, 11)));
}

#[test]
fn test_cell_opposite_left() {
  let c = Cell(10, 10);
  let p = Cell(11, 10);

  assert_eq!(c.opposite(100, 100, p), Some(Cell(9, 10)));
}

#[test]
fn test_cell_opposite_up() {
  let c = Cell(10, 10);
  let p = Cell(10, 11);

  assert_eq!(c.opposite(100, 100, p), Some(Cell(10, 9)));
}