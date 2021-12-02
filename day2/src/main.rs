//use std::fs::File;
//use std::io::{self, BufRead};

fn part1() {
    let data = include_str!("input.txt");
    let mut hpos = 0;
    let mut depth = 0;
    for line in data.lines() {
        let mut tmp = line.split(" ");
        let action = tmp.next().unwrap();
        let amount = tmp.next().unwrap().parse::<i32>().unwrap();
        match action {
            "forward" => hpos += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => {}
        }
    }
    println!(
        "hpos = {} depth = {} hpos * depth = {}",
        hpos,
        depth,
        hpos * depth,
    );
}

fn part2() {
    let data = include_str!("input.txt");
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in data.lines() {
        let mut tmp = line.split(" ");
        let action = tmp.next().unwrap();
        let amount = tmp.next().unwrap().parse::<i32>().unwrap();
        match action {
            "forward" => {
                hpos += amount;
                depth += amount * aim
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => {}
        }
    }
    println!(
        "hpos = {} depth = {} hpos * depth = {}",
        hpos,
        depth,
        hpos * depth,
    );
}

/*
fn opart1() {
    let filename = "./src/input.txt";
    let file = File::open(filename).unwrap();
    let buf = io::BufReader::new(file);
    let lines = buf.lines();
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in lines {
        if let Ok(theline) = line {
            //println!("{}", theline);
            let mut it = theline.split(" ");
            let s = it.next().unwrap();
            let v = (it.next().unwrap()).parse::<i32>().unwrap();
            match s {
                "forward" => {
                    hpos += v;
                    depth += (v * aim);
                }
                "down" => aim += v,
                "up" => aim -= v,
                _ => println!(),
            }
        }
    }
    println!(
        "HPOS = {} DEPTH = {}. Product = {}",
        hpos,
        depth,
        hpos * depth
    );
}
*/

fn main() {
    part1();
    part2();
}
