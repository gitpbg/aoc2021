fn main() {
    let part = 2;
    let data = include_str!("input.txt");
    let mut data: Vec<i32> = data
        .split(",")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect();

    data.sort();
    if part == 1 {
        let l = data.len();
        let median = if (l % 2) == 0 {
            println!("{} {}", data[l / 2], data[l / 2 - 1]);
            (data[l / 2] + data[l / 2 - 1]) / 2
        } else {
            println!("{}", data[l / 2]);
            data[l / 2] as i32
        };
        println!("{:?} len = {}", data, l);
        println!("Median {}", median);
        let sum: i32 = data.into_iter().map(|x| (median - x).abs()).sum();
        println!("sum = {}", sum);
    } else {
        let sum: i32 = data.iter().sum();
        let avg = sum as f64 / data.len() as f64;
        println!("Average {}", &avg);
        let avg = (avg.floor() as i32, avg.ceil() as i32);
        println!("Average {:?}", avg);
        let sum1: i32 = data
            .clone()
            .into_iter()
            .map(|x| {
                let steps = (avg.0 - x).abs();
                let v = (steps * (1 + steps)) / 2;
                //println!("x = {} steps = {} v = {}", x, steps, v);
                v
            })
            .sum();
        println!("Sum = {}", sum1);
        // Use the other integer part of round
        let sum2: i32 = data
            .into_iter()
            .map(|x| {
                let steps = (avg.1 - x).abs();
                let v = (steps * (1 + steps)) / 2;
                //println!("x = {} steps = {} v = {}", x, steps, v);
                v
            })
            .sum();
        println!("Sum = {}", sum2);
        println!("Answer = {}", std::cmp::min(sum1, sum2));
    }
}
