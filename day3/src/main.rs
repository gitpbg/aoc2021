fn get_data<'a>() -> &'a str {
    include_str!("input.txt")
}

fn find_max_bits(max: u64) -> usize {
    let mut v = max;
    let mut ct = 0_usize;
    while v > 0 {
        v >>= 1;
        ct += 1;
    }
    ct
}

fn part1(tmp: &Vec<u64>, maxbitpos: usize) {
    let mut gamma = 0;
    for bitpos in (0..maxbitpos).rev() {
        gamma <<= 1;
        let mask = 1_u64 << bitpos;
        println!("Mask = {:b}", mask);
        let ct = tmp.iter().filter(|x| (*x) & mask == mask).count();
        println!("ct={} len={}", ct, tmp.len());
        if ct * 2 >= tmp.len() {
            gamma |= 1;
        } else {
            //println!("0")
            gamma |= 0;
        }
    }
    let epsilon = ((1u64 << maxbitpos) - 1) & !gamma;

    println!(
        "gamma={} epsilon={} prod={}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn find_oxygen_num(data: &Vec<u64>, bitpos: usize) -> u64 {
    let mask = 1u64 << bitpos;
    let one_ct = data.iter().filter(|x| ((*x) & mask == mask)).count();
    let zero_ct = data.len() - one_ct;
    println!("one={}, zero={}", one_ct, zero_ct);
    let buf: Vec<u64> = if one_ct >= zero_ct {
        data.iter()
            .filter(|x| (*x) & mask == mask)
            .map(|x| *x)
            .collect()
    } else {
        data.iter()
            .filter(|x| (*x) & mask == 0)
            .map(|x| *x)
            .collect()
    };
    println!("bitpos {} buf={:?}", bitpos, buf);
    if buf.len() == 0 {
        panic!("something bad happened")
    }
    if buf.len() == 1 {
        return buf[0];
    }
    if bitpos == 0 {
        panic!("bitpos is 0 and we have not converged")
    }
    return find_oxygen_num(&buf, bitpos - 1);
}

fn find_co2_num(data: &Vec<u64>, bitpos: usize) -> u64 {
    let mask = 1u64 << bitpos;
    let one_ct = data.iter().filter(|x| ((*x) & mask == mask)).count();
    let zero_ct = data.len() - one_ct;
    println!("one={}, zero={}", one_ct, zero_ct);
    let buf: Vec<u64> = if zero_ct <= one_ct {
        data.iter()
            .filter(|x| (*x) & mask == 0)
            .map(|x| *x)
            .collect()
    } else {
        data.iter()
            .filter(|x| (*x) & mask == mask)
            .map(|x| *x)
            .collect()
    };
    println!("bitpos {} buf={:?}", bitpos, buf);
    if buf.len() == 0 {
        panic!("something bad happened")
    }
    if buf.len() == 1 {
        return buf[0];
    }
    if bitpos == 0 {
        panic!("bitpos is 0 and we have not converged")
    }
    return find_co2_num(&buf, bitpos - 1);
}

fn part2(data: &Vec<u64>, maxbitpos: usize) {
    //    let d = data.clone();
    let o2num = find_oxygen_num(data, maxbitpos - 1);
    let co2num = find_co2_num(data, maxbitpos - 1);
    println!(
        "o2num = {} co2num={} prod = {}",
        o2num,
        co2num,
        o2num * co2num
    );
}

fn main() {
    let part = 2;
    let data = get_data();
    let tmp: Vec<u64> = data
        .lines()
        .map(|s| u64::from_str_radix(s, 2).unwrap())
        .collect();
    let max = *tmp.iter().max().unwrap();
    let maxbitpos = find_max_bits(max);
    println!("Max Val = {} Binary={:b} MaxBitPos={}", max, max, maxbitpos);
    if part == 1 {
        part1(&tmp, maxbitpos);
    } else if part == 2 {
        part2(&tmp, maxbitpos);
    }
    //    part2(data);
}
