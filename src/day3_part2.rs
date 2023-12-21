use std::fs;
#[derive(Copy, Clone, Debug)]

struct Gear {
    ct: u32,
    res: u32,
}

fn main() {
    let day = 3;
    let file_path = format!("src/input/input{day}.txt");
    let mut ans = 0;

    let data: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let sz = data.len();
    let n = data[0].len();
    let mut arr = vec![Gear { ct: 0, res: 1 }; sz * n];

    for i in 0..sz {
        let mut l = n;
        let mut r: usize;

        for (idx, c) in data[i].chars().enumerate() {
            if l == n && c.is_digit(10) {
                l = idx;
                continue;
            }
            if l != n && !c.is_digit(10) {
                r = idx - 1;
                get_num(l, r, n, i, sz, &data, &mut arr);
                l = n;
            }
        }

        if l != n {
            r = n - 1;
            get_num(l, r, n, i, sz, &data, &mut arr);
        }
    }

    for it in 0..n * sz {
        if arr[it].ct == 2 {
            ans += arr[it].res;
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
    arr: &mut Vec<Gear>,
) {
    let num: u32 = data[i][l..r + 1].trim().parse().unwrap();
    let lo = if l > 0 { l - 1 } else { l };
    let hi = if r < n - 1 { r + 1 } else { r };

    if i > 0 {
        for j in lo..=hi {
            if &data[i - 1][j..j + 1] == "*" {
                arr[j * n + i - 1].ct += 1;
                arr[j * n + i - 1].res *= num;
            }
        }
    }

    if i < sz - 1 {
        for j in lo..=hi {
            if &data[i + 1][j..j + 1] == "*" {
                arr[j * sz + i + 1].ct += 1;
                arr[j * sz + i + 1].res *= num;
            }
        }
    }

    if &data[i][lo..lo + 1] == "*" {
        arr[lo * sz + i].ct += 1;
        arr[lo * sz + i].res *= num;
    }

    if &data[i][hi..hi + 1] == "*" {
        arr[hi * sz + i].ct += 1;
        arr[hi * sz + i].res *= num;
    }
}
