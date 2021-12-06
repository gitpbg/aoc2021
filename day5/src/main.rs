//use core::ops::{Add, AddAssign};
use std::collections::HashMap;

fn parse_line<'a>(rv: &mut Vec<i32>, s: &'a str) {
    s.split("->").for_each(|x| {
        x.trim()
            .split(",")
            .for_each(|x| rv.push(x.parse().unwrap()))
    });
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // fn delta(&self, other: &Point) -> Point {
    //     let dx = other.x - self.x;
    //     let dy = other.y - self.y;
    //     if dx != 0 && dy != 0 && dx.abs() != dx.abs() {
    //         panic!(
    //             "Points {:?} and {:?} are not horizontal vertical or diagonal",
    //             self, other
    //         );
    //     }
    //     Point::new(dx / dx.abs(), dy / dy.abs())
    // }

    // fn swap(p1: &mut Point, p2: &mut Point) {
    //     let tmp = p1.x;
    //     p1.x = p2.x;
    //     p2.x = tmp;
    //     let tmp = p1.y;
    //     p1.y = p2.y;
    //     p2.y = tmp;
    // }
    // fn add(&self, rhs: &Point) -> Point {
    //     Point::new(self.x + rhs.x, self.y + rhs.y)
    // }
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        Line {
            p1: Point::new(x1, y1),
            p2: Point::new(x2, y2),
        }
    }
    fn points(&self, allow_diagonals: bool) -> LineIterator {
        LineIterator::new(&self, allow_diagonals)
    }
}
struct LineIterator {
    cur: Point,
    delta: Point,
    steps: i32,
}

impl LineIterator {
    fn new(l: &Line, allow_diagonals: bool) -> LineIterator {
        let dx = l.p2.x - l.p1.x;
        let dy = l.p2.y - l.p1.y;
        let mut delta = Point::new(0, 0);
        let mut steps = 0;
        if dx == 0 && dy != 0 {
            // vertical line
            delta.x = 0;
            delta.y = dy / dy.abs();
            steps = dy.abs();
        } else if dx != 0 && dy == 0 {
            // horizontal line
            delta.x = dx / dx.abs();
            delta.y = 0;
            steps = dx.abs();
        } else if dx.abs() == dy.abs() {
            if allow_diagonals {
                delta.x = dx / dx.abs();
                delta.y = dy / dy.abs();
                steps = dx.abs();
            } else {
                steps = -1;
            }
        }
        LineIterator {
            cur: Point::new(l.p1.x, l.p1.y),
            delta: delta,
            steps: steps,
        }
    }
}
impl Iterator for LineIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.steps < 0 {
            return None;
        }
        let p = Some(Point::new(self.cur.x, self.cur.y));
        self.cur.add_assign(&self.delta);
        self.steps -= 1;
        p
    }
}

struct SeafloorMap {
    hm: HashMap<(i32, i32), i32>,
}

impl SeafloorMap {
    fn new() -> SeafloorMap {
        SeafloorMap { hm: HashMap::new() }
    }

    fn mark(&mut self, x: i32, y: i32) {
        let k = (x, y);
        if self.hm.contains_key(&k) {
            let v = self.hm[&k] + 1;
            *(self.hm.get_mut(&k).unwrap()) = v;
        } else {
            self.hm.insert(k, 1);
        }
    }

    fn count_gt2(&self) -> usize {
        self.hm.values().filter(|x| **x > 1).count()
    }
}
fn main() {
    let data = include_str!("input.txt");
    let mut v: Vec<i32> = Vec::new();
    data.lines().for_each(|x| parse_line(&mut v, x));
    let mut v1: Vec<((i32, i32), (i32, i32))> = Vec::new();
    let mut vlines: Vec<Line> = Vec::new();
    for i in (0..v.len()).step_by(4) {
        let a = v[i];
        let b = v[i + 1];
        let c = v[i + 2];
        let d = v[i + 3];
        vlines.push(Line::new(a, b, c, d));
        v1.push(((a, b), (c, d)));
    }
    let mut sm = SeafloorMap::new();
    for line in &vlines {
        //println!("Line {:?}", line);
        for p in line.points(false) {
            //println!("{:?}", p);
            sm.mark(p.x, p.y);
        }
    }
    println!("Part 1 - Intersection locations: {}", sm.count_gt2());
    let mut sm = SeafloorMap::new();
    for line in &vlines {
        for p in line.points(true) {
            sm.mark(p.x, p.y);
        }
    }
    println!("Part 2 - Intersection locations: {}", sm.count_gt2());
}
