use std::io;

fn main() {
    let nums = read_numbers_from_input();
    let r = nums[0] as usize;
    let c = nums[1] as usize;
    let mut g = vec![String::new(); r as usize];
    for i in 0..r {
        io::stdin().read_line(&mut g[i]).expect("读取输入失败");
        g[i] = g[i].trim().to_string();
    }
    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            if i + 3 < r {
                if (g[i].chars().nth(j).unwrap() == 'h'
                    && g[i + 1].chars().nth(j).unwrap() == 'e'
                    && g[i + 2].chars().nth(j).unwrap() == 'h'
                    && g[i + 3].chars().nth(j).unwrap() == 'e')
                    || (g[i].chars().nth(j).unwrap() == 'e'
                        && g[i + 1].chars().nth(j).unwrap() == 'h'
                        && g[i + 2].chars().nth(j).unwrap() == 'e'
                        && g[i + 3].chars().nth(j).unwrap() == 'h')
                {
                    ans += 1;
                }
            }
            if j + 3 < c {
                if (g[i].chars().nth(j).unwrap() == 'h'
                    && g[i].chars().nth(j + 1).unwrap() == 'e'
                    && g[i].chars().nth(j + 2).unwrap() == 'h'
                    && g[i].chars().nth(j + 3).unwrap() == 'e')
                    || (g[i].chars().nth(j).unwrap() == 'e'
                        && g[i].chars().nth(j + 1).unwrap() == 'h'
                        && g[i].chars().nth(j + 2).unwrap() == 'e'
                        && g[i].chars().nth(j + 3).unwrap() == 'h')
                {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
fn read_numbers_from_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}
