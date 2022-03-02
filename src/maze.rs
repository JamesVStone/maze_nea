

use std::collections::{VecDeque, HashSet, HashMap};
use std::hash::Hash;

use rand::{thread_rng, Rng};


pub struct Color(pub u8, pub u8, pub u8);

// 2D Maze Object
pub struct Maze {
  pub end: Cell,
  pub grid: Vec<Vec<bool>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell(usize, usize);

impl Cell {
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
  fn neighbors(&self, grid: &Vec<Vec<bool>>) -> Vec<Cell> {
    let h = grid.len();
    let w = grid[0].len();
    let Cell(x, y) = *self;

    let mut neighbours = Vec::new();
    if x > 0 && grid[y][x-1] {
      neighbours.push(Cell(x-1, y))
    }
    if y > 0 && grid[y-1][x] {
      neighbours.push(Cell(x, y-1))
    }
    if x < w-1 && grid[y][x+1] {
      neighbours.push(Cell(x+1, y))
    }
    if y < h-1 && grid[y+1][x] {
      neighbours.push(Cell(x, y+1))
    }
    
    neighbours
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

      frontier.remove(nwall);
    }

    Self {
      end,
      grid
    }
  }
  pub fn solve(&self) -> Option<Vec<Cell>> {
    let mut visited: HashSet<Cell> = HashSet::new();
    let mut parent: HashMap<Cell, Cell> = HashMap::new();
    let mut path: Vec<Cell> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(Cell(0, 0));

    while let Some(current_cell) = queue.pop_front() {
        path.push(current_cell);

        // Verify if this vertex is end of the maze
        if current_cell == self.end {
            // Follow the parent chain to find the path
            let mut p = self.end;
            let mut ppath: Vec<Cell> = Vec::new();
            ppath.push(p);

            while p != Cell(0, 0) {
              if let Some(l) = parent.get(&p) {
                p = *l;
                ppath.push(p);
              }
            }

            return Some(ppath);
        }

        // Check each neighbour of the current cell
        for neighbor in current_cell.neighbors(&self.grid).into_iter().rev() {
            if visited.insert(neighbor) {
                // Add the neighbor on front of the dequeue
                queue.push_front(neighbor);
                parent.insert(neighbor, current_cell);
            }
        }
    }

    // Return none if no path is found
    None
  }
  pub fn to_bitmap(&self, solved: bool) -> Vec<Color> {
    let mut data: Vec<Color> = Vec::with_capacity((self.grid.len() + 2) * (self.grid[0].len()+2));
    let mut solve = None;
    if solved {
      solve = self.solve();
    }

    for _ in 0..self.grid[0].len()+2 {
      data.push(Color(0, 0, 0));
    }

    for (i, row) in self.grid.iter().enumerate() {
      data.push(Color(0, 0, 0));
      for (j, cell) in row.iter().enumerate() {
        if Cell(j, i) == self.end {
          data.push(Color(255, 0, 0));
        } else if Cell(j, i) == Cell(0, 0) {
          data.push(Color(0, 255, 0))
        } else if *cell {
          let mut f = false;
          if let Some(ss) = &solve {
            for s in ss {
              if *s == Cell(j, i) { f = true; }
            }
          }

          if f {
            data.push(Color(0, 0, 255));
          } else {
            data.push(Color(255, 255, 255));
          }
        } else {
          data.push(Color(0, 0, 0))
        }
      }
      data.push(Color(0, 0, 0));
    }

    for _ in 0..self.grid[0].len()+2 {
      data.push(Color(0, 0, 0));
    }

    data
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