fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        // let num = line.parse::<i32>().unwrap();
        let chars: Vec<&str> = line.split(' ').collect();
        
        match chars[0] {
            "A" => match chars[1] {
                "X" => ans += 4,
                "Y" => ans += 8,
                "Z" => ans += 3,
                _ => ans += 0
            },
            "B" => match chars[1] {
                "X" => ans += 1,
                "Y" => ans += 5,
                "Z" => ans += 9,
                _ => ans += 0 
            },
            "C" => match chars[1] {
                "X" => ans += 7,
                "Y" => ans += 2,
                "Z" => ans += 6,
                _ => ans += 0 
            },
            _ => println!("Empty")
        }
        
    }

    return ans;

}

fn solve_part2(input: &str) -> i32 {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        let chars: Vec<&str> = line.split(' ').collect();
        
        match chars[0] {
            "A" => match chars[1] {
                "X" => ans += 3,
                "Y" => ans += 4,
                "Z" => ans += 8,
                _ => ans += 0
            },
            "B" => match chars[1] {
                "X" => ans += 1,
                "Y" => ans += 5,
                "Z" => ans += 9,
                _ => ans += 0 
            },
            "C" => match chars[1] {
                "X" => ans += 2,
                "Y" => ans += 6,
                "Z" => ans += 7,
                _ => ans += 0 
            },
            _ => println!("Empty")
        }
        
    }

    return ans;

}

fn main() {
    let input = include_str!("input.txt");

    let ans1 = solve_part1(&input);
    let ans2 = solve_part2(&input);
    println!("{}", ans1);
    println!("{}", ans2);
}
