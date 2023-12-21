use regex::Regex;
use std::fs;

fn main() {
    let day = 3;
    let file_path = format!("src/input/input{day}.txt");
    let re = Regex::new("[^.0-9]").unwrap();
    let mut ans = 0;

    let data: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let sz = data.len();
    for i in 0..sz {
        let n = data[i].len();
        let mut l = n;
        let mut r: usize;

        for (idx, c) in data[i].chars().enumerate() {
            if l == n && c.is_digit(10) {
                l = idx;
                continue;
            }
            if l != n && !c.is_digit(10) {
                r = idx - 1;
                ans += get_num(l, r, n, i, sz, &data, &re);
                l = n;
            }
        }

        if l != n {
            r = n - 1;
            ans += get_num(l, r, n, i, sz, &data, &re);
        }
    }

    println!("{}", ans);
}

fn get_num(
    l: usize,
    r: usize,
    n: usize,
    i: usize,
    sz: usize,
    data: &Vec<String>,
    re: &Regex,
) -> u32 {
    let num: u32 = data[i][l..r + 1].trim().parse().unwrap();
    let lo = if l > 0 { l - 1 } else { l };
    let hi = if r < n - 1 { r + 1 } else { r };

    if i > 0 && check(&data[i - 1][lo..hi + 1], re) {
        return num;
    }

    if i < sz - 1 && check(&data[i + 1][lo..hi + 1], re) {
        return num;
    }

    if check(&data[i][lo..lo + 1], re) || check(&data[i][hi..hi + 1], re) {
        return num;
    }

    0
}

fn check(str: &str, re: &Regex) -> bool {
    re.is_match(str)
}
