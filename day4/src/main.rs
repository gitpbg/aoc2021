use std::collections::HashMap;
#[derive(Debug)]
struct Game {
    numbers: Vec<i32>,
    boards: Vec<Board>,
    cur_number: usize,
}

enum AnsiColors {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn get_code(ac: AnsiColors) -> i32 {
    match ac {
        AnsiColors::Reset => 0,
        AnsiColors::Black => 30,
        AnsiColors::Red => 31,
        AnsiColors::Green => 32,
        AnsiColors::Yellow => 33,
        AnsiColors::Blue => 34,
        AnsiColors::Magenta => 35,
        AnsiColors::Cyan => 36,
        AnsiColors::White => 37,
    }
}
fn set_color(clr: AnsiColors) {
    let start = "\x1b[";
    let end = "m";
    print!("{}{}{}", start, get_code(clr), end);
}

fn print_in_color(s: &str, clr: AnsiColors) {
    let start = "\x1b[";
    let end = "m";
    print!("{}{}{}{}", start, get_code(clr), end, s);
}

#[derive(Debug)]
enum BingoCell {
    Unhit(i32),
    Hit(i32),
}
#[derive(Debug)]
struct Board {
    number: i32,
    board: Vec<BingoCell>,
    numbermap: HashMap<i32, i32>,
    idx: usize,
    win: bool,
    score: i32,
    last_num: i32,
}
impl Board {
    fn new() -> Board {
        Board {
            number: 0,
            board: Vec::new(),
            numbermap: HashMap::new(),
            idx: 0,
            last_num: -1,
            score: -1,
            win: false,
        }
    }

    fn add_row(&mut self, v: &str) {
        println!("Add row {} for board {}", v, self.number);
        let res: Vec<&str> = v.split(" ").collect();
        println!("{:?}", res);
        v.split(" ").for_each(|x| {
            let tmp = x.trim();
            if tmp.len() == 0 {
            } else {
                let tmp2 = tmp.parse().unwrap();
                self.board.push(BingoCell::Unhit(tmp2));
                //println!("Mapping {} to {}", tmp2, self.idx);
                self.numbermap.insert(tmp2, self.idx as i32);
                self.idx += 1;
            }
        });
        println!("Board {} items {}", self.number, self.board.len());
    }

    fn apply_number(&mut self, num: i32) {
        self.last_num = num;
        if self.numbermap.contains_key(&num) {
            let idx = self.numbermap[&num];
            if let Some(v) = self.board.get_mut(idx as usize) {
                if let BingoCell::Unhit(v2) = v {
                    //println!("v2 == num {} v2 = {}, num = {}", *v2 == num, v2, num);
                }
                *v = BingoCell::Hit(num);
            }
        } else {
            println!("board does not contain {}", num)
        }
    }

    fn has_win(&mut self) -> bool {
        println!("Board is {} long", self.board.len());
        let mut ctr = 0;
        for i in (0..25).step_by(5) {
            ctr = 0;
            for j in i..i + 5 {
                if j >= self.board.len() {
                    println!(
                        "j = {}, len = {}, number = {}",
                        j,
                        self.board.len(),
                        self.number
                    );
                }
                if let BingoCell::Unhit(v) = self.board[j] {
                    break;
                } else {
                    ctr += 1;
                }
            }
            if ctr == 5 {
                self.win = true;
                self.compute_score();
                return true;
            }
        }
        for i in 0..5 {
            ctr = 0;
            for j in (i..i + 21).step_by(5) {
                if let BingoCell::Unhit(v) = self.board[j] {
                    break;
                } else {
                    ctr = ctr + 1;
                }
            }
            if ctr == 5 {
                self.win = true;
                self.compute_score();
                return true;
            }
        }
        return false;
    }

    fn compute_score(&mut self) {
        self.score = 0;
        if !self.win {
            println!("ERROR: compute_score called for non winning board");
            return;
        }
        let mut score = 0;
        self.board
            .iter()
            .filter(|x| {
                if let BingoCell::Unhit(v) = *x {
                    true
                } else {
                    false
                }
            })
            .for_each(|x| {
                if let BingoCell::Unhit(v) = *x {
                    score += v;
                }
            });
        self.score = score;
        println!(
            "Score = {} last num = {} prod = {}",
            self.score,
            self.last_num,
            self.score * self.last_num
        );
        self.score = self.score * self.last_num;
    }
    fn set_number(&mut self, val: i32) {
        self.number = val;
    }
    fn print_board(&self) {
        println!("Board {}", self.number);
        if (self.win) {
            println!("Winning board Score = {}", self.score);
        }
        for i in (0..25).step_by(5) {
            for j in (i..i + 5) {
                match self.board[j] {
                    BingoCell::Hit(v) => {
                        set_color(AnsiColors::Green);
                        print!("{:2} ", v);
                        set_color(AnsiColors::Reset);
                    }
                    BingoCell::Unhit(v) => {
                        print!("{:2} ", v);
                    }
                }
            }
            println!();
        }
    }
}

impl Game {
    fn new() -> Game {
        Game {
            numbers: Vec::new(),
            boards: Vec::new(),
            cur_number: 0,
        }
    }

    fn init_numbers(&mut self, data: &str) {
        data.split(",").for_each(|x| {
            if let Ok(v) = x.parse() {
                self.numbers.push(v);
            } else {
                println!("Encountered bad split [{}]", x);
            }
        })
    }

    fn play_one_number(&mut self) {
        if self.cur_number < self.numbers.len() {
            let num = self.numbers[self.cur_number];
            println!("Playing {}", num);
            for b in self.boards.iter_mut() {
                if b.win {
                    continue;
                } else {
                    b.apply_number(num);
                }
            }
            self.cur_number += 1;
        }
    }

    fn play_one_board(&mut self) {
        let num = self.numbers[self.cur_number];
        println!("Playing {}", num);
        self.boards[0].apply_number(num);
        self.boards[0].print_board();
        self.cur_number += 1;
    }

    fn has_win(&mut self) -> bool {
        for b in &mut self.boards {
            if b.has_win() {
                return true;
            }
        }
        return false;
    }

    fn add_board(&mut self, num: i32) {
        self.boards.push(Board::new());
        self.boards.last_mut().unwrap().set_number(num);
    }
    fn pop_board(&mut self) {
        self.boards.pop();
    }

    fn add_row_to_board(&mut self, v: &str) {
        let b = self.boards.last_mut().unwrap();
        b.add_row(v);
    }

    fn print(&self) {
        for b in &self.boards {
            b.print_board();
            println!();
        }
    }

    fn has_more_numbers(&self) -> bool {
        return self.cur_number < self.numbers.len();
    }

    fn part1(&mut self) {
        loop {
            self.play_one_number();
            if self.has_win() {
                println!("win");
                self.print();
                break;
            }
        }
    }

    fn part2(&mut self) {
        let mut done = false;
        let mut last_board_number: i32 = -1;
        while !done && self.has_more_numbers() {
            self.play_one_number();
            for b in self.boards.iter_mut() {
                if b.win {
                    continue;
                }
                if b.has_win() {
                    last_board_number = b.number;
                }
            }
        }
        for b in &self.boards {
            if b.number == last_board_number {
                println!("Last Board");
                b.print_board();
                println!("Score = {}", b.score);
            }
        }
    }
}

fn main() {
    let mut boardnum = 1;
    let data = include_str!("input.txt");
    let mut lines = data.lines();
    let v = loop {
        let tmp = lines.next().unwrap();
        if tmp.len() == 0 {
        } else if tmp.chars().nth(0).unwrap() == '#' {
            println!("Skipping comment {}", tmp);
        } else {
            break tmp;
        }
    };
    println!("Now reading numbers");
    let mut game: Game = Game::new();
    game.init_numbers(v);
    let mut add_new_board = true;
    for line in lines {
        if line.trim().len() == 0 {
            add_new_board = true
        } else {
            if add_new_board == true {
                game.add_board(boardnum);
                add_new_board = false;
                boardnum += 1;
            }
            game.add_row_to_board(line.trim())
        }
    }
    let part = 2;
    if part == 1 {
        game.part1();
    } else {
        game.part2();
    }
    //println!("g={:?}", game);
    // println!("\x1B[92mHello\x1B[31m World\x1b[0m - Reset");
    // print_in_color("Red Text\n", AnsiColors::Red);
    // println!("Continuing in Red");
    // print_in_color("Green Text\n", AnsiColors::Green);
    // println!("Continuing in Green");
    // print_in_color("Reset Text\n", AnsiColors::Reset);
    // set_color(AnsiColors::Blue);
    // println!("This is now blue text and will remain so until i reset it");
    // println!("here we go");
    // println!("Next line will be magenta");
    // set_color(AnsiColors::Magenta);
    // println!("viola!!!");
    // set_color(AnsiColors::Reset);
}
