mod trie {
    pub struct Tree {
        pub children: Vec<Vec<i32>>,
        pub is_word: Vec<bool>,
        pub contains: Vec<bool>,
    }
    impl Tree {
        pub fn new() -> Self {
            Self {
                children: vec![vec![-1; 26]; 1],
                is_word: vec![false],
                contains: vec![false],
            }
        }
        pub fn insert(&mut self, word: String) {
            let mut node = 0;
            for c in word.chars() {
                let c = c as usize - 'a' as usize;
                if self.children[node][c] == -1 {
                    self.children[node][c] = self.children.len() as i32;
                    self.children.push(vec![-1; 26]);
                    self.is_word.push(false);
                    self.contains.push(false);
                }
                node = self.children[node][c] as usize;
            }
            self.is_word[node] = true;
        }
        fn get_contains_helper(&self, id: usize, ans: &mut Vec<String>, word: &mut String) {
            println!(
                "{}: contains: {} is_word: {}",
                word, self.contains[id], self.is_word[id]
            );
            if self.contains[id] && self.is_word[id] {
                ans.push(word.clone());
            }
            for i in 0..26 {
                let next_id = self.children[id][i];
                if next_id == -1 {
                    continue;
                }
                word.push(('a' as u8 + i as u8) as char);
                self.get_contains_helper(next_id as usize, ans, word);
                word.pop();
            }
        }
        pub fn get_contains(&self) -> Vec<String> {
            let mut ans = vec![];
            self.get_contains_helper(0, &mut ans, &mut String::new());
            ans
        }
    }
}

impl Solution {
    fn dfs(
        x: i32,
        y: i32,
        board: &Vec<Vec<char>>,
        vis: &mut Vec<Vec<bool>>,
        id: usize,
        trie: &mut trie::Tree,
    ) {
        if x < 0 || x >= board.len() as i32 || y < 0 || y >= board[0].len() as i32 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if vis[x][y] {
            return;
        }
        vis[x][y] = true;
        let c = board[x][y] as usize - 'a' as usize;
        if trie.children[id][c] == -1 {
            vis[x][y] = false;
            return;
        }
        let next_id = trie.children[id][c] as usize;
        if trie.is_word[next_id] {
            trie.contains[next_id] = true;
        }

        Solution::dfs(x as i32 - 1, y as i32, board, vis, next_id, trie);
        Solution::dfs(x as i32 + 1, y as i32, board, vis, next_id, trie);
        Solution::dfs(x as i32, y as i32 - 1, board, vis, next_id, trie);
        Solution::dfs(x as i32, y as i32 + 1, board, vis, next_id, trie);
        vis[x][y] = false;
    }
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = trie::Tree::new();
        for word in words {
            trie.insert(word);
        }
        let mut vis = vec![vec![false; board[0].len()]; board.len()];
        for i in (0..board.len()).rev() {
            for j in (0..board[0].len()).rev() {
                Solution::dfs(i as i32, j as i32, &board, &mut vis, 0, &mut trie);
            }
        }
        trie.get_contains()
    }
}
struct Solution;
fn main() {
    let board = vec![vec!['a']];
    let words = vec!["a".to_string()];
    println!("{:?}", Solution::find_words(board, words));
}
