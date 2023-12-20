use std::fs;

fn main() {
    let day = 1;
    let file_path = format!("src/input/input{day}.txt");
    let mut ans = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        for c in line.to_string().chars() {
            if c.is_digit(10) {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.to_string().chars() {
            if c.is_digit(10) {
                last_digit = c.to_digit(10).unwrap();
            }
        }
        ans += first_digit * 10 + last_digit;
    }

    println!("{}", ans);
}
