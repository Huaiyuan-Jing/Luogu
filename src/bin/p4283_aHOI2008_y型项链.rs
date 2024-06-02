#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut necklaces = vec![];
    for _ in 0..3 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        necklaces.push(s[1].clone());
    }
    let mut ans: i32 = (0..3)
        .map(|x| steps_to_target(necklaces[x].as_str(), ""))
        .sum();
    for i in 0..3 {
        for j in 0..necklaces[i].len() {
            let target = &necklaces[i][..j + 1];
            ans = ans.min(
                (0..3)
                    .map(|x| steps_to_target(necklaces[x].as_str(), target))
                    .sum(),
            );
        }
    }
    println!("{}", ans);
}
fn steps_to_target(necklace: &str, target: &str) -> i32 {
    let mut common_len = 0;
    while common_len < target.len()
        && common_len < necklace.len()
        && necklace.chars().nth(common_len) == target.chars().nth(common_len)
    {
        common_len += 1;
    }
    (necklace.len() + target.len() - common_len * 2) as i32
}
