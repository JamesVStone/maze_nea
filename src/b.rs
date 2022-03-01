#[allow(dead_code)]

use std::collections::HashMap;
use rand::Rng;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    println!("Hello, world!");

    #[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
    struct Cell(usize, usize);

    impl Cell {
        fn random(rng: &mut ThreadRng) -> Self {
            Self(rng.gen_range(0, WIDTH), rng.gen_range(0, HEIGHT))
        }
    }
    /*
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9
    - - - - - - - - - -
    0|1|2|3|4|5|6|7|8|9

    
    */
    // array structured as horizontal borders then vertical borders
    let mut grid = [[true; WIDTH-1]; 2*HEIGHT-1];
    let mut visited: HashMap<Cell, bool> = HashMap::new();

    let start = Cell::random(&mut rng);
    let mut walls: Vec<(usize, usize)> = Vec::new();

    visited.insert(start, true);

    // add walls to wall list
    let x = start.0;
    let y = start.1;

    println!("{:?}", start);

    if x > 0 {
        walls.push((x-1, 2*y))
    }
    if x < WIDTH {
        walls.push((x, 2*y))
    }
    if y > 0 {
        walls.push((x, 2*y-1))
    }
    if y < HEIGHT {
        walls.push((x, 2*y+1))
    }

    // left
    while !walls.is_empty() {
        let wall = walls.choose(&mut rng).unwrap().to_owned();

        let x = wall.0;
        let y = wall.1;

        let mut cell1 = Cell(0, 0);
        let mut cell2 = Cell(0, 0);

        if wall.1 % 2 == 0 {
            cell1 = Cell(x, y/2);
            cell2 = Cell(x+1, y/2);
        }
        else {
            cell1 = Cell(x, (y-1)/2);
            cell2 = Cell(x, (y+1)/2);
        }
        let c1c = visited.get(&cell1) == None;
        let c2c = visited.get(&cell2) == None;

        let mut newcell = Cell(0, 0);
        if c1c ^ c2c {
            if c1c {
                newcell = cell2;
            } else {
                newcell = cell1;
            }

            visited.insert(newcell, true);

            let x = newcell.0;
            let y = newcell.1;
        
            if x > 0 {
                walls.push((x-1, 2*y))
            }
            if x < WIDTH {
                walls.push((x, 2*y))
            }
            if y > 0 {
                walls.push((x, 2*y-1))
            }
            if y < HEIGHT {
                walls.push((x, 2*y+1))
            }

            grid[wall.0][wall.1] = false;
        }
        let i = walls.iter().position(|x| *x == wall).unwrap();
        walls.remove(i);
    }

    for (i, w) in grid.iter().enumerate() {
        println!("");
        if i % 2 == 0 {
            for b in w.iter() {
                if *b {
                    print!(" |");
                }
                else {
                    print!("  ");
                }
            }
        } else {
            for b in w.iter() {
                if *b {
                    print!("- ");
                }
                else {
                    print!("  ");
                }
            }
        }
    }
}
