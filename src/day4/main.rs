use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let value = INPUT
        .lines()
        .map(|line| {
            let (_, card) = line.split_once(':').unwrap();
            let (numbers, candidate) = card.split_once('|').unwrap();

            let numbers = numbers
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let num_wins = candidate
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .filter(|num| numbers.contains(num))
                .count();

            if num_wins >= 1 {
                1 << (num_wins - 1)
            } else {
                0
            }
        })
        .sum::<u32>();

    println!("{}", value);

    let cards = INPUT
        .lines()
        .fold(HashMap::<u32, u32>::default(), |mut cards, line| {
            let (card_num, card) = line.split_once(':').unwrap();
            let card_num = card_num
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let (numbers, candidate) = card.split_once('|').unwrap();

            let numbers = numbers
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let num_wins = candidate
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .filter(|num| numbers.contains(num))
                .count() as u32;

            // Increment card count by 1 for the original one we have
            *cards.entry(card_num).or_default() += 1;
            let count = cards[&card_num];

            // Increment subsequent cards by number of copies of current card
            for i in (card_num + 1)..(card_num + num_wins + 1) {
                *cards.entry(i).or_default() += count;
            }

            cards
        });

    println!("{}", cards.values().sum::<u32>());
}
