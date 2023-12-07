use std::collections::HashMap;
use std::iter::once;

const INPUT: &'static str = include_str!("input.txt");

struct Entry {
    value: u32,
    bid: u32,
}

fn parse_input(input: &str, use_jokers: bool) -> Entry {
    let (hand, bid) = input.split_once(' ').unwrap();

    let cards = hand.chars().map(|char| match char {
        'A' => 0xE,
        'K' => 0xD,
        'Q' => 0xC,
        'J' if !use_jokers => 0xB,
        'J' if use_jokers => 0x1,
        'T' => 0xA,
        c => c.to_digit(16).unwrap(),
    });

    let mut map = HashMap::<u32, u32>::new();
    for card in cards.clone() {
        *map.entry(card).or_default() += 1;
    }

    let max_of_a_kind = map
        .iter()
        .filter(|(c, _)| **c != 0x1)
        .map(|(_, v)| *v)
        .max()
        .unwrap_or_default();
    let num_pairs = map
        .iter()
        .filter(|(c, _)| **c != 0x1)
        .filter(|(_, x)| **x >= 2)
        .count();
    let num_jokers = map.get(&0x1).copied().unwrap_or_default();

    let type_ = match (max_of_a_kind + num_jokers, num_pairs, num_jokers) {
        // Five of a kind
        (5, _, _) => 0x7,
        // Four of a kind
        (4, _, _) => 0x6,
        // Full house
        (3, 2, 0) => 0x5,
        (_, 2, 1) => 0x5,
        // Three of a kind
        (3, _, _) => 0x4,
        // Two pair
        (_, 2, _) => 0x3,
        (_, 1, 1) => 0x3,
        // One pair
        (_, 1, _) => 0x2,
        (_, _, 1) => 0x2,
        // High card
        (_, _, _) => 0x1,
    };

    let value = once(type_)
        .chain(cards.clone())
        .fold(0, |acc, d| acc * 16 + d);

    Entry {
        bid: bid.parse().unwrap(),
        value,
    }
}

fn main() {
    let mut hands = INPUT
        .lines()
        .map(|line| parse_input(line, false))
        .collect::<Vec<_>>();
    hands.sort_by_key(|e| e.value);

    let sum = hands
        .iter()
        .enumerate()
        // Zero indexed
        .map(|(i, e)| (i + 1) as u32 * e.bid)
        .sum::<u32>();

    println!("{}", sum);

    let mut hands = INPUT
        .lines()
        .map(|line| parse_input(line, true))
        .collect::<Vec<_>>();
    hands.sort_by_key(|e| e.value);

    let sum = hands
        .iter()
        .enumerate()
        // Zero indexed
        .map(|(i, e)| (i + 1) as u32 * e.bid)
        .sum::<u32>();

    println!("{}", sum);
}
