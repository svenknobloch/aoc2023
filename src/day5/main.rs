use std::collections::BTreeMap;
use std::ops::Range;

const INPUT: &'static str = include_str!("input.txt");

fn parse_seeds(input: &str) -> Vec<Range<i64>> {
    input
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .map(|n| n..n + 1)
        .collect()
}

fn parse_seeds_v2(input: &str) -> Vec<Range<i64>> {
    let inputs = input
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    inputs
        .chunks_exact(2)
        .map(|vals| vals[0]..vals[0] + vals[1])
        .take(1)
        .collect()
}

fn parse_map(input: &str) -> BTreeMap<i64, (i64, i64)> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            let mut iter = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            let dst = iter.next().unwrap();
            let src = iter.next().unwrap();
            let len = iter.next().unwrap();

            (src, (dst, len))
        })
        .collect()
}

fn apply_map(map: &BTreeMap<i64, (i64, i64)>, sources: &[Range<i64>]) -> Vec<Range<i64>> {
    sources
        .iter()
        .flat_map(|source| {
            let (source, mut ranges) = map
                .range(0..source.end)
                .rev()
                .map(|(src, (dst, len))| (*src..*src + *len, *dst..*dst + *len))
                .take_while(move |(src, _)| source.start <= src.end)
                .fold(
                    (source.clone(), Vec::<Range<i64>>::new()),
                    |(head, mut vec), (src, dst)| {
                        let start = src.start.max(head.start);
                        let end = src.end.min(head.end);

                        let head = head.start..start;

                        let offset = dst.start - src.start;
                        let mid = (start + offset)..(end + offset);
                        if !mid.is_empty() {
                            vec.push(mid);
                        }

                        let tail = end..head.end;
                        if !tail.is_empty() {
                            vec.push(tail);
                        }

                        (head, vec)
                    },
                );

            if !source.is_empty() {
                ranges.push(source)
            }

            ranges
        })
        .collect()
}

fn min_range(input: &[Range<i64>]) -> i64 {
    input.iter().map(|range| range.start).min().unwrap()
}

fn main() {
    let mut iter = INPUT.split("\n\n");

    let seeds_input = iter.next().unwrap();
    let seeds_to_soil = parse_map(iter.next().unwrap());
    let soil_to_fertilizer = parse_map(iter.next().unwrap());
    let fertilizer_to_water = parse_map(iter.next().unwrap());
    let water_to_light = parse_map(iter.next().unwrap());
    let light_to_temperature = parse_map(iter.next().unwrap());
    let temperature_to_humidity = parse_map(iter.next().unwrap());
    let humidity_to_location = parse_map(iter.next().unwrap());

    let soil = apply_map(&seeds_to_soil, &parse_seeds(seeds_input));
    // panic!();
    let fertilizer = apply_map(&soil_to_fertilizer, &soil);
    let water = apply_map(&fertilizer_to_water, &fertilizer);
    let light = apply_map(&water_to_light, &water);
    let temperature = apply_map(&light_to_temperature, &light);
    let humidity = apply_map(&temperature_to_humidity, &temperature);
    let location = apply_map(&humidity_to_location, &humidity);

    println!("{:?}", min_range(&location));

    let soil = dbg!(apply_map(
        &seeds_to_soil,
        &dbg!(parse_seeds_v2(seeds_input))
    ));
    let fertilizer = apply_map(&soil_to_fertilizer, &soil);
    let water = apply_map(&fertilizer_to_water, &fertilizer);
    let light = apply_map(&water_to_light, &water);
    let temperature = apply_map(&light_to_temperature, &light);
    let humidity = apply_map(&temperature_to_humidity, &temperature);
    let location = apply_map(&humidity_to_location, &humidity);

    println!("{:?}", min_range(&location));
}
