use std::fs;

fn main() {
    let day = 4;
    let file_path = format!("src/input/input{day}.txt");
    let mut ans = 0;

    let data: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let sz = data.len();
    let mut card_ct: Vec<u32> = vec![1; sz];

    for line in data {
        let card: Vec<&str> = line.split(":").collect();
        let id: usize = card[0][5..card[0].len()].trim().parse().unwrap();
        
        let nums: Vec<&str> = card[1].split("|").collect();
        let mut winning_nums: Vec<&str> = nums[0].trim().split(" ").collect();
        let mut have_nums: Vec<&str> = nums[1].trim().split(" ").collect();

        winning_nums.retain(|&x| x != "");
        have_nums.retain(|&x| x != "");

        let winning_nums: Vec<u32> = winning_nums
            .iter()
            .map(|&x| x.trim().parse().unwrap())
            .collect();
        let have_nums: Vec<u32> = have_nums
            .iter()
            .map(|&x| x.trim().parse().unwrap())
            .collect();

        let mut ct = 0;

        for num in &have_nums {
            for win in &winning_nums {
                if num == win {
                    ct += 1;
                }
            }
        }

        for card in id + 1..=id + ct {
            card_ct[card - 1] += card_ct[id - 1];
        }
    }

    for card in 0..sz {
        ans += card_ct[card];
    }

    println!("{}", ans);
}
