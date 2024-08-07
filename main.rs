use eframe::{App, NativeOptions};
use genawaiter::stack::*;
use genawaiter::*;
use itertools::Itertools;
use rand::{*, seq::SliceRandom, rngs::ThreadRng};
use std::collections::HashSet;
mod gui;

macro_rules! neighbours_iter {
    ($nev:ident, $x:expr, $y:expr) => {
        let_gen!($nev, {
            for x in 0..9 {
                if x == $x {
                    continue;
                }
                yield_!((x, $y));
            }
            for y in 0..9 {
                if y == $y {
                    continue;
                }
                yield_!(($x, y));
            }
            let low_x = ($x / 3)*3;
            let high_x = low_x + 3;
            let low_y = ($y / 3)*3;
            let high_y = low_y + 3;
            for x in low_x..high_x {
                for y in low_y..high_y {
                    if x == $x || y == $y {
                        continue;
                    }
                    yield_!((x, y));
                }
            }
        })
    };
}
#[derive(Clone)]
struct SudokuApp {
    board: Vec<Vec<Option<u8>>>,
    rng: ThreadRng,
    selected_tile: (usize, usize),
    possible_states: Vec<Vec<HashSet<u8>>>,
}
impl SudokuApp {
    fn new() -> Self {
        let mut app = SudokuApp {
            board: vec![vec![None; 9]; 9],
            rng: thread_rng(),
            selected_tile: (0, 0),
            possible_states: vec![vec![HashSet::from_iter(1..=9); 9]; 9]
        };
        let mut vals = (1..=9).collect_vec();
        let mut coords = (0..9).cartesian_product(0..9).collect_vec();
        coords.shuffle(&mut app.rng);
        
        for i in 0..21 {
            let (x, y) = coords[i];
            vals.shuffle(&mut app.rng);
            for val in &vals {
                if app.solvable((x, y), *val) {
                    app.board[x][y] = Some(*val);
                    app.collapse((x, y), *val);
                    break;
                }
            }
        }
        app
    }
    fn has_neighbour_with_val(&self, loc: (usize, usize), val: u8) -> bool {
        neighbours_iter!(it, loc.0, loc.1);
        for (x, y) in it {
            let tile = &self.board[x][y];
            if tile.is_some() && tile.unwrap() == val {
                return true;
            }
        }
        false
    }
    fn solve(&mut self) -> bool {
        // collapse the tile with minimum entropy, collapse it to
        // one of it's possible values, then repeat recursively
        // return wether we found a solution.
        // if one value is not a solution, decollapse it and try the next one
        let next = self.find_min_entropy();
        if next.is_none() {
            return true;
        }
        let (x, y) = next.unwrap();
        for i in 1..=9 {
            if self.has_neighbour_with_val((x, y), i) {
                continue;
            }
            let removed = self.collapse((x, y), i);
            if self.solve() {
                return true;
            } else {
                self.decollapse((x, y), &removed)
            }
        }
        return false;
    }
    fn collapse(&mut self, loc: (usize, usize), val: u8) -> Vec<(usize, usize)> {
        self.board[loc.0][loc.1] = Some(val);
        neighbours_iter!(neighbours, loc.0, loc.1);
        let mut r = vec![];
        for (x, y) in neighbours {
            if self.possible_states[x][y].remove(&val) {
                r.push((x, y))
            }
        }
        r
    }
    fn decollapse(&mut self, loc: (usize, usize), removed: &Vec<(usize, usize)>) {
        let val = self.board[loc.0][loc.1];
        self.board[loc.0][loc.1] = None;
        for (x, y) in removed {
            self.possible_states[*x][*y].insert(val.unwrap());
        }
    }
    fn find_min_entropy(&self) -> Option<(usize, usize)> {
        // tile with fewest possible states has minimum entropy
        let mut r = None;
        for x in 0..9 {
            for y in 0..9 {
                if r.is_none() {
                    if self.board[x][y].is_none() {
                        r = Some((x, y));
                    }
                    continue;
                }
                if self.board[x][y].is_some() {
                    continue;
                }
                let n = r.unwrap();
                if self.possible_states[x][y].len() < self.possible_states[n.0][n.1].len() {
                    r = Some((x, y));
                }
            }
        }
        r
    }
    fn solvable(&mut self, loc: (usize, usize), newval: u8) -> bool {
        if self.has_neighbour_with_val(loc, newval) {
            return false;
        }
        let removed = self.collapse(loc, newval);
        let next = self.find_min_entropy();
        if next.is_none() {
            self.decollapse(loc, &removed);
            return true;
        }
        for i in 1..=9 {
            if self.solvable(next.unwrap(), i) {
                self.decollapse(loc, &removed);
                return true;
            }
        }
        self.decollapse(loc, &removed);
        return false;
    }
}


fn main() {
    eframe::run_native(
        "Rudoku",
        NativeOptions::default(),
        Box::new(|_cc| Box::new(SudokuApp::new()) as Box<dyn App>),
    ).unwrap();
}