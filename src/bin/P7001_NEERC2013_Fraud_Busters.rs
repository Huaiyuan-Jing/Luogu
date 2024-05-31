use std::io;

fn main() {
    let mut mask = String::new();
    io::stdin().read_line(&mut mask).unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let mut ans = n;
    let mut passed_strings = vec![];
    for _ in 0..n {
        let mut s = String::new();
        let mut passed = true;
        io::stdin().read_line(&mut s).unwrap();
        for j in 0..s.len() {
            if mask.chars().nth(j).unwrap() == '*' || mask.chars().nth(j).unwrap() == s.chars().nth(j).unwrap() {
                continue;
            }
            passed = false;
            ans -= 1;
            break;
        }
        if passed {
            passed_strings.push(s.trim().to_string());
        }
    }
    println!("{}", ans);
    for s in passed_strings {
        println!("{}", s);
    }
}
