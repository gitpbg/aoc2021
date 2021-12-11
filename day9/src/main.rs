#![allow(unused)]

struct TwoDArray<T> {
    w: usize,
    h: usize,
    data: Vec<T>,
}
impl<T> TwoDArray<T>
where
    T: Default + Clone + Copy,
{
    fn new(w: usize, h: usize) -> TwoDArray<T> {
        let mut rv = TwoDArray {
            w: w,
            h: h,
            data: Vec::new(),
        };

        rv.data.resize(w * h, Default::default());
        rv
    }

    fn index(&self, r: i32, c: i32) -> Option<usize> {
        if r >= 0 && c >= 0 && (r as usize) < self.h && (c as usize) < self.w {
            Some(r as usize * self.w + c as usize)
        } else {
            None
        }
    }

    fn get(&self, r: i32, c: i32) -> Option<T> {
        if let Some(idx) = self.index(r, c) {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn set(&mut self, r: i32, c: i32, v: T) {
        if let Some(idx) = self.index(r, c) {
            self.data[idx] = v;
        }
    }
}

struct HeightMap {
    heights: TwoDArray<i32>,
    visited: TwoDArray<bool>,
}

impl HeightMap {
    fn new(w: usize, h: usize) -> HeightMap {
        let mut hm = HeightMap {
            heights: TwoDArray::new(w, h),
            visited: TwoDArray::new(w, h),
        };
        hm
    }

    fn locate_lowest(&self) -> Vec<(usize, usize, i32)> {
        let mut lps: Vec<(usize, usize, i32)> = Vec::new();
        for r in 0..self.heights.h {
            let ir = r as i32;
            for c in 0..self.heights.w {
                let ic = c as i32;
                let mut ct = 0;
                let cur = self.heights.get(ir, ic).unwrap();
                if let Some(v) = self.heights.get(ir - 1, ic) {
                    if v > cur {
                        ct += 1;
                    }
                } else {
                    //count boundary points as passing the test
                    ct += 1;
                }
                if let Some(v) = self.heights.get(ir + 1, ic) {
                    if v > cur {
                        //println!("v = {} cur = {}", v, cur);
                        ct += 1;
                    }
                } else {
                    ct += 1;
                }
                if let Some(v) = self.heights.get(ir, ic - 1) {
                    if v > cur {
                        //println!("v = {} cur = {}", v, cur);
                        ct += 1;
                    }
                } else {
                    ct += 1;
                }
                if let Some(v) = self.heights.get(ir, ic + 1) {
                    if v > cur {
                        //println!("v = {} cur = {}", v, cur);
                        ct += 1;
                    }
                } else {
                    ct += 1;
                }
                if ct == 4 {
                    //println!("pushing {}", cur);
                    lps.push((r, c, cur));
                }
            }
        }
        lps
    }

    fn count_neighbors(&mut self, r: usize, c: usize) -> i32 {
        let mut ct = 1;
        let cur = self.heights.get(r as i32, c as i32).unwrap();
        //println!("r={} c={} cur={}", r, c, cur);
        let ic = c as i32;
        let ir = r as i32;
        if let Some(v) = self.heights.get(ir - 1, ic) {
            if v < 9 && v > cur && !self.visited.get(ir - 1, ic).unwrap() {
                ct += self.count_neighbors(r - 1, c);
            }
        }
        if let Some(v) = self.heights.get(ir + 1, ic) {
            if v < 9 && v > cur && !self.visited.get(ir + 1, ic).unwrap() {
                ct += self.count_neighbors(r + 1, c);
            }
        }
        if let Some(v) = self.heights.get(ir, ic - 1) {
            if v < 9 && v > cur && !self.visited.get(ir, ic - 1).unwrap() {
                ct += self.count_neighbors(r, c - 1);
            }
        }
        if let Some(v) = self.heights.get(ir, ic + 1) {
            if v < 9 && v > cur && !self.visited.get(ir, ic + 1).unwrap() {
                ct += self.count_neighbors(r, c + 1);
            }
        }
        self.visited.set(ir, ic, true);
        return ct;
    }
    fn find_basin(&mut self, pt: &(usize, usize, i32)) -> i32 {
        let mut rv: i32 = 0;
        rv = self.count_neighbors(pt.0, pt.1);
        rv
    }
}

fn main() {
    let data = include_str!("input.txt");
    let mut w: usize = 0;
    let h: usize = data
        .lines()
        .map(|x| {
            w = x.len();
            1
        })
        .count();
    let mut hm = HeightMap::new(w, h);
    let mut row = 0;
    data.lines().for_each(|x| {
        let mut col = 0;
        x.trim().chars().for_each(|c| {
            let v = c as i32 - '0' as i32;
            hm.heights.set(row, col, v);
            col += 1;
        });
        row += 1;
    });

    let lps = hm.locate_lowest();
    //println!("{:?}", lps);
    println!("{:?}", lps.iter().map(|(_, _, x)| *x + 1).sum::<i32>());
    let mut res: Vec<i32> = lps
        .iter()
        .map(|x| {
            //println!();
            hm.find_basin(x)
        })
        .collect();
    res.sort();
    res.reverse();
    println!("{:?}", res[0] * res[1] * res[2]);
}
