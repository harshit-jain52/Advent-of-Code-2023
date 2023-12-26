use std::fs;

fn main() {
    let day = 6;
    let file_path = format!("src/input/input{day}.txt");
    let mut ans = 1;

    let data: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let time: Vec<&str> = data[0].split(":").collect();
    let mut times: Vec<&str> = time[1].split(" ").collect();
    times.retain(|&x| x != "");
    let times: Vec<u32> = times.iter().map(|&x| x.trim().parse().unwrap()).collect();

    let distance: Vec<&str> = data[1].split(":").collect();
    let mut distances: Vec<&str> = distance[1].split(" ").collect();
    distances.retain(|&x| x != "");
    let distances: Vec<u32> = distances
        .iter()
        .map(|&x| x.trim().parse().unwrap())
        .collect();

    for i in 0..times.len() {
        let mut ct = 0;
        for j in 0..=times[i] {
            let d = j * (times[i] - j);
            if d > distances[i] {
                ct += 1;
            }
        }
        ans *= ct;
    }
    println!("{}", ans);
}
