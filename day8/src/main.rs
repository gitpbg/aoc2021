/*

*/

fn get_nums(digits: &Vec<String>, seq: &Vec<String>) -> usize {
    const TOP: usize = 0;
    const TL: usize = 1;
    const TR: usize = 2;
    const MIDDLE: usize = 3;
    const BL: usize = 4;
    const BR: usize = 5;
    const BOTTOM: usize = 6;

    // create an array of segments a-g indexed 0-6
    let mut segments = ['x'; 7];

    //find the one which is the shortest string
    let one = digits.iter().find(|x| x.len() == 2).unwrap();
    //println!("Shortest {}", one);
    // bottom right is in all digits except 2
    let br: char = one
        .chars()
        .map(|x| (x, digits.iter().filter(|s| s.contains(x)).count()))
        .filter(|x| x.1 == 9)
        .map(|x| x.0)
        .next()
        .unwrap();
    // the other character is the top right
    let tr: char = one.chars().filter(|x| *x != br).next().unwrap();
    segments[BR] = br;
    segments[TR] = tr;

    //seven is 1 and the top segment
    let seven = digits.iter().filter(|x| x.len() == 3).next().unwrap();
    //println!("{}", seven);
    let top: char = seven
        .chars()
        .filter(|x| (*x != tr) && (*x != br))
        .next()
        .unwrap();
    segments[TOP] = top;

    let four = digits.iter().filter(|x| x.len() == 4).next().unwrap();
    //println!("four {}", four);
    // 3, 5 & 2 have 5 segments each
    // Three has all the horizontal segments and the 1
    // Five has all the horizonal segments but not top right - any segment not in 3 is top left
    // two has all the horizontal segments except any segment not in 3 is bottom left
    let three = digits
        .iter()
        .filter(|x| x.len() == 5 && x.contains(tr) && x.contains(br))
        .next()
        .unwrap();
    let five = digits
        .iter()
        .filter(|x| x.len() == 5 && x.contains(br) && !x.contains(tr))
        .next()
        .unwrap();

    let two = digits
        .iter()
        .filter(|x| x.len() == 5 && x.contains(tr) && !x.contains(br))
        .next()
        .unwrap();
    let eight = digits.iter().filter(|x| x.len() == 7).next().unwrap();
    //println!("three {}", three);
    //println!("five {}", five);
    //println!("two {}", two);
    let tl = five.chars().filter(|x| !three.contains(*x)).next().unwrap();
    //println!("bl = {}", tl);
    segments[BL] = tl;
    let bl = two.chars().filter(|x| !three.contains(*x)).next().unwrap();
    segments[TL] = bl;
    let middle = four
        .chars()
        .filter(|x| (*x != tl) && (*x != tr) && (*x != br))
        .next()
        .unwrap();
    segments[MIDDLE] = middle;
    // bottom is the last one not in segments so extract it from three
    let bottom = three
        .chars()
        .filter(|x| !segments.contains(x))
        .next()
        .unwrap();
    segments[BOTTOM] = bottom;
    let zero = digits
        .iter()
        .filter(|x| x.len() == 6 && !x.contains(middle))
        .next()
        .unwrap();

    let nine = digits
        .iter()
        .filter(|x| (x.len() == 6) && !x.contains(bl))
        .next()
        .unwrap();
    let six = digits
        .iter()
        .filter(|x| x.len() == 6 && !x.contains(tr))
        .next()
        .unwrap();

    //println!("{:?}", segments);
    // build an array of all the digits
    let alldigits = vec![zero, one, two, three, four, five, six, seven, eight, nine];
    // build the number from the sequence of digits
    let mut res = 0;
    seq.iter().for_each(|x| {
        if let Some(idx) = alldigits.iter().position(|px| *(*px) == *x) {
            //println!("Num: {}", idx);
            res = res * 10 + idx
        } else {
            println!("not found {}", x);
        }
    });
    res
}

fn main() {
    let data = include_str!("input.txt");
    //use iterators to parse the data into a tuple of 2 vectors of strings and build a vector of tuples.
    let data: Vec<(Vec<String>, Vec<String>)> = data
        .lines()
        .map(|x| {
            let mut sp = x.split("|");
            let left = sp.next().unwrap().trim();
            let right = sp.next().unwrap().trim();
            let left: Vec<String> = left
                .split(" ")
                .map(|x| {
                    let mut tmp: Vec<char> = x.chars().collect();
                    tmp.sort();
                    tmp.iter().collect::<String>()
                })
                .collect();
            let right: Vec<String> = right
                .split(" ")
                .map(|x| {
                    let mut tmp: Vec<char> = x.chars().collect();
                    tmp.sort();
                    tmp.iter().collect::<String>()
                })
                .collect();
            (left, right)
        })
        .collect();
    let mut ct = 0;
    data.iter().for_each(|(_, right)| {
        let tmp = right
            .iter()
            .filter(|x| -> bool {
                let l = (*x).len();
                (l == 2) || (l == 3) || (l == 4) || (l == 7)
            })
            .count();
        ct += tmp;
    });
    println!("{:?}", ct);
    println!("{:?}", data[0].0);
    let sum: usize = data.iter().map(|(d, s)| get_nums(d, s)).sum();
    println!("Sum = {}", sum);
}
