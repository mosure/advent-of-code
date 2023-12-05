use std::collections::HashSet;


fn parse_card(card: &str) -> (HashSet<i32>, HashSet<i32>) {
    let parts: Vec<&str> = card.split(':').nth(1).unwrap().split('|').collect();

    let parse_numbers = |s: &str|
        s.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
    (
        parse_numbers(parts[0]),
        parse_numbers(parts[1])
    )
}

fn calculate_scratchcard_points(cards: &[&str]) -> i32 {
    cards.iter()
        .map(|&card| {
            let (winning_numbers, your_numbers) = parse_card(card);
            winning_numbers.intersection(&your_numbers).count()
        })
        .filter(|&matches| matches > 0)
        .map(|matches| 1 << (matches - 1))
        .sum()
}

fn count_total_scratchcards(cards: &[&str]) -> i32 {
    let matching_numbers: Vec<usize> = cards.iter()
        .map(|&card| {
            let (winning_numbers, your_numbers) = parse_card(card);
            winning_numbers.intersection(&your_numbers).count()
        })
        .collect();

    let mut total_instances = vec![1; cards.len()];

    for (i, &matches) in matching_numbers.iter().enumerate() {
        let current_value = total_instances[i];
        total_instances.iter_mut()
            .skip(i + 1)
            .take(matches)
            .for_each(|instance| *instance += current_value);
    }

    total_instances.iter().sum()
}

fn main() {
    let data_in: Vec<&str> = include_str!("4.input").lines().collect();

    println!("{}", calculate_scratchcard_points(&data_in));
    println!("{}", count_total_scratchcards(&data_in));
}
