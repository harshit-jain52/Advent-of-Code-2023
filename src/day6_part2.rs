use std::fs;

fn main() {
    let day = 6;
    let file_path = format!("src/input/input{day}.txt");
    let mut ans = 0;

    let data: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let time: Vec<&str> = data[0].split(":").collect();
    let mut times: Vec<&str> = time[1].split(" ").collect();
    times.retain(|&x| x != "");

    let distance: Vec<&str> = data[1].split(":").collect();
    let mut distances: Vec<&str> = distance[1].split(" ").collect();
    distances.retain(|&x| x != "");

    let mut tim = String::from("");
    let mut dist = String::from("");

    for i in 0..times.len() {
        tim.push_str(times[i]);
        dist.push_str(distances[i]);
    }

    let tim: u128 = tim.trim().parse().unwrap();
    let dist: u128 = dist.trim().parse().unwrap();

    for j in 0..=tim {
        let d = j * (tim - j);
        if d > dist {
            ans += 1;
        }
    }
    println!("{}", ans);
}
