//! Solver for Arrows and Arrows Reddit Daily Programmer challenge.
//!
//! **See**: [Original Link](http://www.reddit.com/r/dailyprogrammer/comments/2m82yz/20141114_challenge_188_hard_arrows_and_arrows/)

use std::io::{File, BufferedReader};
use std::os;
use std::collections::HashMap;

fn step(pos: uint, delta: int, dimension: uint) -> uint {
    let dim = dimension as int;
    let new = (pos as int) + delta;
    match new >= 0 {
        true  => (new % dim) as uint,
        false => (dim + new) as uint, 
    }
}

/// An expression of direction in 2-space
#[deriving(Show, Clone)]
pub enum Direction {
    /// North; expressed by `^`
    Up, 
    /// South; expressed by `v`
    Down, 
    /// West; expressed by `<`
    Left, 
    ///  East; expressed by `>`
    Right
}

impl Direction {
    /// Creates a new `Direction` from a symbol.
    pub fn from_glyph(c: char) -> Direction {
        match c {
            '^'   => Direction::Up,
            'v'   => Direction::Down,
            '<'   => Direction::Left,
            '>'   => Direction::Right,
            other => panic!("{} is not a recognizable direction", other),
        }
    }

    /// Gets the symbolic representation of this `Direction`
    pub fn to_glyph(&self) -> char {
        match *self {
            Direction::Up    => '^',
            Direction::Down  => 'v',
            Direction::Left  => '<',
            Direction::Right => '>',
        }
    }
}

/// Representation of the input graph and purported bounds.
#[deriving(Show)]
pub struct GraphMeta {
    pub width: uint,
    pub height: uint,
    /// the representation of directional pointers between nodes in the graph;
    /// expressed in row-major order
    pub pointers: Vec<Vec<Direction>>
}

impl GraphMeta {
    /// Creates a new `GraphMeta` from a file at the path *fname*.
    pub fn from_input_file(fname: &str) -> GraphMeta {
        let path = Path::new(fname);
        let mut file = BufferedReader::new(File::open(&path));
        let file_lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

        // get purported bounds of grid
        let (width, height) = match file_lines[0].as_slice().trim().split(' ')
            .filter_map(from_str).collect::<Vec<uint>>().as_slice() {
            [width, height] => (width, height),
            other => panic!("Dimensions line has {} elements when it must have 2", 
                           other.len()),
        };

        // build up grid of Directions from input
        let mut pointers = Vec::new();
        for line in file_lines.iter().skip(1) {
            let line_pointers: Vec<Direction> = 
                line.as_slice().trim().chars().map(Direction::from_glyph)
                .collect();
            if line_pointers.len() != width {
                panic!("Line contains {} pointers when it should contain {}", 
                      line_pointers.len(), width);
            }
            pointers.push(line_pointers);
        }
        if pointers.len() != height {
            panic!(
                "File contains {} lines of pointers when it should contain {}", 
                pointers.len(), height);
        }

        GraphMeta { width: width, height: height, pointers: pointers }
    }
    
    /// Finds a `Cycle` rooted at the point given by (*x*, *y*). 
    ///
    /// (*x*, *y*) need not be a part of the returned `Cycle`, it may simply be u
    /// a *prelude* to a cycle.
    pub fn get_cycle_from(&self, x: uint, y: uint) -> Cycle {
        if x >= self.width || y >= self.height {
            panic!("x or y position out of bounds.");
        }

        let mut cycle = Vec::new();
        let mut node_coords = HashMap::new();
        let mut cur_x = x;
        let mut cur_y = y;
        let mut i = 0u;

        // build up cycle (and potentially prelude to cycle)
        while ! node_coords.contains_key(&(cur_x, cur_y)) {
            node_coords.insert((cur_x, cur_y), i);
            let pointer = self.pointers[cur_y][cur_x];
            cycle.push(Node { x: cur_x, y: cur_y, pointer: pointer });

            let (next_x, next_y) = match pointer {
                Direction::Up    => (cur_x, step(cur_y, -1, self.height)),
                Direction::Down  => (cur_x, step(cur_y, 1, self.height)), 
                Direction::Left  => (step(cur_x, -1, self.width), cur_y),
                Direction::Right => (step(cur_x, 1, self.width), cur_y),
            };

            cur_x = next_x;
            cur_y = next_y;
            i += 1;
        }

        // trim prelude
        let cycle_start = *node_coords.get(&(cur_x, cur_y)).unwrap();
        if cycle_start != 0u {
            // case: a prelude exists; trim it off
            std::vec::as_vec(cycle.slice_from_or_fail(&cycle_start)).deref()
                .clone()
        } else {
            cycle
        }
    }

    /// Returns the `Cycle` of maximum length present. 
    ///
    /// Ties are broken in favor of Cycles rooted by a position in the graph
    /// closer to (*0*, *0*).
    fn get_max_cycle(&self) -> Cycle {
        let mut max_length = 0u;
        let mut max_cycle: Cycle = Vec::new();

        for x in range(0, self.width) {
            for y in range(0, self.height) {
                let cycle = self.get_cycle_from(x, y);
                if cycle.len() > max_length {
                    max_length = cycle.len(); 
                    max_cycle = cycle;
                }
            }
        }
        max_cycle
    }
}

/// Representation of a single position in the input graph.
#[deriving(Show, Clone)]
pub struct Node {
    pub x: uint,
    pub y: uint,
    pub pointer: Direction,
}

/// Representation of a cycle in the input graph.
pub type Cycle = Vec<Node>;

/// Prints a textual representation of a cycle to *stdout*.
fn print_cycle(cycle: &Cycle, meta: &GraphMeta) {
    let mut lines = Vec::new();
    for _ in range(0, meta.height) {
        let mut line = Vec::new();
        for _ in range(0, meta.width) {
            line.push(' '); 
        }
        lines.push(line);
    }

    for node in cycle.iter() {
        lines[node.y][node.x] = node.pointer.to_glyph();
    } 

    for (_, line) in lines.iter().enumerate() {
        println!("{}", String::from_chars(line.as_slice()));
    } 
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("Insufficient num args! _uitting.");
        return
    }

    let graph_meta = GraphMeta::from_input_file(args[1].as_slice());
    let max_cycle = graph_meta.get_max_cycle();
    println!("Longest cycle: {}", max_cycle.len());
    println!("Position:");
    print_cycle(&max_cycle, &graph_meta);
}
