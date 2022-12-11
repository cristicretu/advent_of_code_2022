use std::{cmp};

fn solve_part2(input: &str) -> i32 {
    let lines = input.lines();
    let mut sums: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for line in lines {
        if line == "" {
            sums.push(sum);
            sum = 0;
            continue;
        }

        let num: i32 = line.parse::<i32>().unwrap();
        sum += num;
    }

    sums.sort_by(|a, b| b.cmp(a));
    return sums.iter().take(3).sum();
}

fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut ans: i32 = 0;
    let mut sum: i32 = 0;
    for line in lines {
        // check for empty line
        if line == "" {
            ans = cmp::max(ans, sum); 
            sum = 0;
            continue;
        }
        let num = line.parse::<i32>().unwrap();
        if num > 0 {
            sum += num;
        } else {
            ans = cmp::max(ans, sum); 
            sum = 0;
        }
    }
    return ans;
}

fn main() {
    let input = include_str!("input.txt");

    let ans1 = solve_part1(&input);
    let ans2: i32 = solve_part2(&input);
    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
