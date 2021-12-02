use itertools::Itertools;
// More powerful accepted

pub fn apart1() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count()
}

pub fn apart2() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(x1, x2, x3)| x1 + x2 + x3)
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count()
}

// mine
fn part1() {
    let mystr = include_str!("input.txt");
    let tmp: Vec<&str> = mystr.split("\n").into_iter().collect();
    let tmp2: Vec<i32> = tmp
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect();
    let tmp3: Vec<(&i32, &i32)> = tmp2.iter().zip(tmp2[1..].iter()).collect();
    let tmp4: Vec<i32> = tmp3
        .iter()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .collect();
    println!("{:?}", tmp4);
    let sum = tmp4.into_iter().reduce(|x, y| x + y).unwrap();
    println!("Sum is {}", sum);
}

fn part2() {
    let mystr = include_str!("input.txt");
    let tmp: Vec<&str> = mystr.split("\n").into_iter().collect();
    let tmp2: Vec<i32> = tmp
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap_or_default())
        .collect();
    let tmp3: Vec<(&i32, (&i32, &i32))> = tmp2
        .iter()
        .zip(tmp2[1..].iter().zip(tmp2[2..].iter()))
        .collect();
    let tmp4: Vec<i32> = tmp3.iter().map(|(a, (b, c))| **a + **b + *c).collect();
    //println!("{:?}", tmp4);
    let tmp5: Vec<i32> = tmp4
        .iter()
        .zip(tmp4[1..].iter())
        .map(|(a, b)| if *a < *b { 1 } else { 0 })
        .collect();
    //println!("{:?}", tmp5);
    let sum = tmp5.into_iter().reduce(|a, b| a + b).unwrap();
    println!("Sum is {}", sum);
}
fn main() {
    let v = 2;
    if v == 1 {
        part1();
        println!("{}", apart1())
    } else {
        part2();
        println!("{}", apart2())
    }
}
