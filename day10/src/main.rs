#![allow(unused)]
use std::{collections::HashMap, fmt::Debug, vec::Vec};

struct Stack<T> {
    data: Vec<T>,
}

impl<T: Default + Copy> Stack<T> {
    fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    fn peek(&self) -> Option<T> {
        if self.data.len() > 0 {
            Some(self.data[self.data.len() - 1])
        } else {
            None
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        let l = self.data.len();
        if l > 0 {
            let val: T = self.data.remove(l - 1);
            Some(val)
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}

fn get_score(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_opening(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => c,
    }
}

fn get_closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => c,
    }
}

fn get_autocorrect_score(c: char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn main() {
    let data = include_str!("input.txt");
    let mut syn_sc: i64 = 0;
    let mut ac_sc = 0_i64;
    let tmp: Vec<(i64, i64)> = data
        .lines()
        .map(|x| {
            let mut st: Stack<char> = Stack::new();
            let mut syntax_score = 0_i64;
            let mut autocorrect_score = 0_i64;
            //println!("{}", x);
            for ch in x.trim().chars() {
                match ch {
                    '(' | '[' | '{' | '<' => {
                        st.push(ch);
                    }
                    ')' | ']' | '}' | '>' => {
                        if let Some(c) = st.pop() {
                            if c != get_opening(ch) {
                                syntax_score = get_score(ch) as i64;
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
            if syntax_score == 0 {
                loop {
                    if let Some(c) = st.pop() {
                        let c2 = get_closing(c);
                        autocorrect_score =
                            autocorrect_score * 5 + get_autocorrect_score(c2) as i64;
                    } else {
                        break;
                    }
                }
            }
            (autocorrect_score, syntax_score)
        })
        .collect();

    syn_sc = tmp.iter().map(|x| x.1).sum();
    let mut acsc: Vec<i64> = tmp.iter().filter(|x| x.0 > 0).map(|x| x.0).collect();
    acsc.sort();
    //println!("{:?}", acsc);
    ac_sc = acsc[acsc.len() / 2];
    println!("Syntax Score = {}", syn_sc);
    println!("Autocorrect Score = {}", ac_sc);
    //println!("{:?}", tmp);
    //    println!("acscore={} syntax_score = {}", ac_sc, syn_sc);
}
