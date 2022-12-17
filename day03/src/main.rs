use std::collections::HashSet;

fn solve_part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut ans = 0;

    for line in lines {
        let (r1, r2) = line.split_at(line.len() / 2);
        let r1_set: HashSet<u8> = r1.bytes().into_iter().collect();
        let r2_set: HashSet<u8> = r2.bytes().into_iter().collect();
        for i in r1_set.intersection(&r2_set) {
            if i.is_ascii_uppercase() {
                ans += *i as i32 - b'A' as i32 + 1i32 + 26i32;
            } else {
                ans += *i as i32 - b'a' as i32 + 1i32;
            }
        }
    }

    return ans;
}

fn solve_part2(input: &str) -> i32 {
    let mut ans = 0;
    let mut groups: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    for group in groups.chunks(3) {
        let mut sets: Vec<HashSet<u8>> = Vec::new();
        sets.push(group.to_owned()[0].bytes().into_iter().collect());
        sets.push(group.to_owned()[1].bytes().into_iter().collect());
        sets.push(group.to_owned()[2].bytes().into_iter().collect());

        // let intersected_set: HashSet<u8> = sets[0].intersection(&sets[1]).intersection(&sets[2]).union().collect();
        let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });

        for i in intersection {
            if i.is_ascii_uppercase() {
                ans += i as i32 - b'A' as i32 + 1i32 + 26i32;
            } else {
                ans += i as i32 - b'a' as i32 + 1i32;
            }
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
