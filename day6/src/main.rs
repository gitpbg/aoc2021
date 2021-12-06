/*
This is one was so easy...
Build a shift register of 9 cells each by the count per day
Save the count in cell 0
Shift all the cells left
Then take the saved count from cell 0 and add to the count in cell 6 and copy to cell 8
(adds to cell 6 because counts from cell 7 may have shifted to cell 6)

Sum up the counts in all the cells to get the number of fish.
*/

fn main() {
    let data = include_str!("input.txt");
    let mut counts = [0u64; 9];
    data.split(",").for_each(|x| {
        if let Ok(v) = x.parse::<usize>() {
            counts[v] += 1;
        }
    });
    println!("Counts = {:?}", counts);
    for i in 0..256 {
        let tmp = counts[0];
        for j in 1..=8 {
            counts[j - 1] = counts[j];
        }
        counts[6] += tmp;
        counts[8] = tmp;
        //println!("{:?}", counts);
        println!("Day {} Count = {}", i + 1, counts.iter().sum::<u64>());
    }
}

/*
// Naive solution will not scale for final input.
use std::collections::{BinaryHeap};

fn main() {
    let data = include_str!("input.txt");
    let mut bh: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    data.split(",").for_each(|x| {
        if let Ok(v) = x.parse() {
            bh.push(Reverse(v));
        }
    });
    println!("fish = {:?}", bh);
    let mut time = 0;
    let mut first = 0;
    let mut start = bh.len();
    while time < 100 {
        //        println!("Time {}", time);
        while bh.peek().unwrap().0 == time - 1 {
            //println!("Before {}", bh.len());
            if let Some(tmp) = bh.pop() {
                bh.push(Reverse(time + 6));
                bh.push(Reverse(time + 8));
            }
        }
        let cur = bh.len();
        if first == 0 && cur > 2 * start {
            println!("first doubling at time {}", time);
            first = time;
        }
        if ((time - first) % 8) == 0 {
            println!("After Day {} - {} fish", time, bh.len());
        }
        //        println!("Ratio = {}", cur as f64 / last as f64);
        time += 1;
    }
    println!("{} fish", bh.len())
}
*/
