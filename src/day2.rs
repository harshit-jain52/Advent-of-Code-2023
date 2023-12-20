use std::cmp::max;
use std::fs;

fn main() {
    let day = 2;
    let file_path = format!("src/input/input{day}.txt");
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    let mut ans = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let game: Vec<&str> = line.split(":").collect();
        let id: i32 = game[0][5..game[0].len()].trim().parse().unwrap();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        let sets: Vec<&str> = game[1].split(";").collect();
        for set in sets {
            let cubes: Vec<&str> = set.split(",").collect();
            for cube in cubes {
                let num_color: Vec<&str> = cube.trim().split(" ").collect();
                let num: i32 = num_color[0].trim().parse().unwrap();
                let color = num_color[1];
                if color == "red" {
                    red = max(red, num);
                } else if color == "green" {
                    green = max(green, num);
                } else if color == "blue" {
                    blue = max(blue, num);
                }
            }
        }

        if red <= red_max && blue <= blue_max && green <= green_max {
            ans += id;
        }
    }
    println!("{}", ans);
}
