use std::fs;

fn main() {
    let day = 1;
    let file_path = format!("src/input/input{day}.txt");
    let mut ans = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let str_line = line.to_string();
        let len = str_line.len();
        let mut first_digit = 0;
        let mut last_digit = 0;

        for (i, c) in str_line.chars().enumerate() {
            let digit = get_digit(c, i, len, line);
            if digit != 0 {
                first_digit = digit;
                break;
            }
        }

        for (i, c) in str_line.chars().enumerate() {
            let digit = get_digit(c, i, len, line);
            if digit != 0 {
                last_digit = digit;
            }
        }
        ans += first_digit * 10 + last_digit;
    }

    println!("{}", ans);
}

fn get_digit(c: char, i: usize, len: usize, line: &str) -> u32 {
    if c.is_digit(10) {
        return c.to_digit(10).unwrap();
    }

    if i + 2 < len {
        let num = &line[i..i + 3];
        if num == "one" {
            return 1;
        } else if num == "two" {
            return 2;
        } else if num == "six" {
            return 6;
        }
    }

    if i + 3 < len {
        let num = &line[i..i + 4];
        if num == "four" {
            return 4;
        } else if num == "five" {
            return 5;
        } else if num == "nine" {
            return 9;
        }
    }

    if i + 4 < len {
        let num = &line[i..i + 5];
        if num == "three" {
            return 3;
        } else if num == "seven" {
            return 7;
        } else if num == "eight" {
            return 8;
        }
    }

    0
}
