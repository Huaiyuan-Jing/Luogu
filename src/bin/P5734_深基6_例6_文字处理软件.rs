use std::io;

fn main() {
    let n = read_numbers_from_input()[0];
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("读取输入失败");
    text = text.trim().to_string();
    for _ in 0..n {
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("读取输入失败");
        let mut op: Vec<String> = op.split_whitespace().map(|word| word.to_string()).collect();
        match op[0].as_str() {
            "1" => {
                text.push_str(&op[1]);
                println!("{}", text);
            }
            "2" => {
                let st = op[1].parse::<usize>().unwrap();
                let length = op[2].parse::<usize>().unwrap();
                text = text[st..st + length].to_string(); 
                println!("{}", text);
            }
            "3" => {
                let pos: usize = op[1].parse().unwrap();
                let s = op[2].clone();
                text.insert_str(pos, &s);
                println!("{}", text);
            }
            "4" => {
                let word = op[1].clone();
                let ans: usize = text.find(&word).unwrap_or(0x3f3f3f3f);
                println!("{}", if ans == 0x3f3f3f3f { -1 } else { ans as i32 });
            }
            _ => {
                // Handle other cases here
            }
        }
    }
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
