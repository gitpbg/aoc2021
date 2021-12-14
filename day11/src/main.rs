#![allow(unused)]
#![allow(dead_code)]

use core::num;
use std::{collections::HashMap, hash::Hasher};

#[derive(Clone, Copy, Debug)]
struct Cell {
    life: i32,
    flashed: bool,
}

impl Cell {
    fn new(v: i32) -> Cell {
        Cell {
            life: v,
            flashed: false,
        }
    }

    fn incr_life(&mut self) {
        if !self.flashed {
            self.life += 1;
        }
    }
    fn can_flash(&self) -> bool {
        !self.flashed && self.life > 9
    }

    fn do_flash(&mut self) {
        self.flashed = true;
    }

    fn flash_done(&mut self) {
        if self.flashed {
            self.life = 0;
            self.flashed = false;
        }
    }
}

#[derive(Debug)]
struct Flasher {
    width: i32,
    height: i32,
    data: Vec<Cell>,
    num_flashed: usize,
    step: usize,
    step_flash: usize,
}

impl Flasher {
    fn new() -> Flasher {
        Flasher {
            width: 0,
            height: 0,
            data: Vec::new(),
            num_flashed: 0,
            step: 0,
            step_flash: 0,
        }
    }

    fn parse(&mut self, dstr: &str) {
        //println!("Parse called, {}", dstr);
        dstr.lines().for_each(|x| {
            println!("{}", x);
            let x = x.trim();
            if self.width == 0 {
                self.width = x.len() as i32;
            }
            self.height += 1;
            x.chars().for_each(|ch| {
                self.data.push(Cell::new(ch as i32 - '0' as i32));
            });
        });
        println!("{} {}", self.width, self.height);
    }

    fn index_to_coord(&self, idx: usize) -> (i32, i32) {
        (idx as i32 / self.width, idx as i32 % self.width)
    }

    fn coord_to_index(&self, r: i32, c: i32) -> usize {
        (r * self.width + c) as usize
    }

    fn tick(&mut self) {
        let mut last_count = self.num_flashed;
        let neighbors = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        self.step = self.step + 1;
        for i in 0..self.data.len() {
            self.data[i].incr_life();
        }
        loop {
            for i in 0..self.data.len() {
                if self.data[i].can_flash() {
                    //println!("{} - {:?}", i, self.data[i]);
                    self.data[i].do_flash();
                    self.num_flashed += 1;
                    let (pr, pc) = self.index_to_coord(i);
                    for (ofr, ofc) in neighbors {
                        let (nr, nc) = (pr + ofr, pc + ofc);
                        if nr >= 0 && nr < self.height && nc >= 0 && nc < self.width {
                            let idx = self.coord_to_index(nr, nc);
                            self.data[idx].incr_life();
                        }
                    }
                }
            }

            if last_count != self.num_flashed {
                last_count = self.num_flashed;
            } else {
                break;
            }
        }
        self.step_flash = self.data.iter().filter(|x| x.flashed).count();

        for i in 0..self.data.len() {
            self.data[i].flash_done();
        }
    }
    fn print(&self) {
        println!("Step {}", self.step);
        for i in 0..self.data.len() {
            if i % (self.width as usize) == 0 {
                print!("\n{}", self.data[i].life);
            } else {
                print!("{}", self.data[i].life)
            }
        }
        println!();
        println!("Num Flashed = {}", self.num_flashed);
    }
}

fn main() {
    let dstr = include_str!("input.txt");
    //let dstr = "000\n086\n000";
    let mut f = Flasher::new();
    f.parse(dstr);
    //    f.print();
    let part = 2;
    if part == 1 {
        for step in 0..100 {
            f.tick();
            //f.print();
        }
        f.print();
    } else if part == 2 {
        while f.step_flash != (f.width * f.height) as usize {
            f.tick();
            if f.step == 2000 {
                break;
            }
        }
        println!("{}", f.step);
    }
}
