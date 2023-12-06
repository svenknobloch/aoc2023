const INPUT: &'static str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    times
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .zip(
            distances
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap()),
        )
        .collect()
}

fn parse_input_v2(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    let time = times
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = distances
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    vec![(time, distance)]
}

fn main() {
    let records = parse_input(INPUT);

    let product = records
        .iter()
        .map(|&(time, distance)| {
            (1..=time)
                .map(|wind| wind * (time - wind))
                .filter(move |d| *d > distance)
                .count() as u64
        })
        .product::<u64>();

    println!("{}", product);

    let records = parse_input_v2(INPUT);

    let product = records
        .iter()
        .map(|&(time, distance)| {
            (1..=time)
                .map(|wind| wind * (time - wind))
                .filter(move |d| *d > distance)
                .count() as u64
        })
        .product::<u64>();

    println!("{}", product);
}
