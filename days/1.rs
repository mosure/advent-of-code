fn main() {
    let input = include_str!("1.input");

    let mut sum = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter(|c| c.is_numeric());

        let first = digits.next().unwrap().to_digit(10);
        let last = digits.next_back().and_then(|c| c.to_digit(10)).or(first);

        sum += first.unwrap() * 10 + last.unwrap();
    }

    println!("{}", sum);
}
