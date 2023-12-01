use std::collections::HashMap;


struct TrieNode {
    children: HashMap<char, TrieNode>,
    value: Option<String>,
}
impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            value: None,
        }
    }

    fn insert(&mut self, word: &str, value: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.value = Some(value);
    }

    fn number_to_digit() -> Self {
        let mut root = TrieNode::new();
        root.insert("one", "1.".into());
        root.insert("two", "2.".into());
        root.insert("three", "3.".into());
        root.insert("four", "4.".into());
        root.insert("five", "5.".into());
        root.insert("six", "6.".into());
        root.insert("seven", "7.".into());
        root.insert("eight", "8.".into());
        root.insert("nine", "9.".into());
        root
    }

    fn search(&self, input: &str) -> Option<(usize, &String)> {
        let mut node = self;
        for (i, c) in input.chars().enumerate() {
            if let Some(next_node) = node.children.get(&c) {
                node = next_node;
                if let Some(value) = &node.value {
                    return Some((i, value));
                }
            } else {
                break;
            }
        }
        None
    }
}

fn replace_numbers_to_digit(input: &str) -> String {
    let root = TrieNode::number_to_digit();
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        if let Some((end, digit)) = root.search(&input[i..]) {
            result.push_str(digit.as_str());
            i += end;
        } else {
            result.push(input.chars().nth(i).unwrap());
            i += 1;
        }
    }

    result
}


fn get_line_coordinate(line: String) -> u32 {
    let mut digits = line.chars().filter(|c| c.is_numeric());

    let first = digits.next().unwrap().to_digit(10);
    let last = digits.next_back().and_then(|c| c.to_digit(10)).or(first);

    first.unwrap() * 10 + last.unwrap()
}


fn main() {
    let input = include_str!("1.input");

    let mut sum = 0;
    for line in input.lines() {
        let substituted = replace_numbers_to_digit(line);
        sum += get_line_coordinate(substituted);
    }

    println!("{}", sum);
}
