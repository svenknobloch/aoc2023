use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::once;
use std::iter::repeat;
use std::mem;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let adjacent = INPUT
        .lines()
        .enumerate()
        .flat_map(|(row, line)| repeat(row).zip(line.chars().enumerate()))
        .filter(|(_, (_, char))| !char.is_numeric() && *char != '.')
        .flat_map(|(row, (col, _))| {
            let row = row as i32;
            let col = col as i32;

            (-1..=1).flat_map(move |i| (-1..=1).map(move |j| (row + i, col + j)))
        })
        .collect::<HashSet<_>>();

    let adjacent = &adjacent;
    let sum = INPUT
        .lines()
        .enumerate()
        .flat_map(|(row, line)| repeat(row).zip(line.chars().chain(once('.')).enumerate()))
        .scan(
            (String::new(), false),
            move |(s, is_adjacent), (row, (col, char))| {
                if char.is_numeric() {
                    s.push(char);
                    *is_adjacent |= adjacent.contains(&(row as i32, col as i32));
                    Some(None)
                } else if s.is_empty() {
                    Some(None)
                } else {
                    let num = s.parse::<u32>().unwrap();
                    s.clear();

                    if mem::replace(is_adjacent, false) {
                        Some(Some(num))
                    } else {
                        Some(None)
                    }
                }
            },
        )
        .flatten()
        .sum::<u32>();

    println!("{}", sum);

    let mut nums = HashMap::<u32, u32>::new();
    let (adjacent, _, _) = INPUT
        .lines()
        .enumerate()
        .flat_map(|(row, line)| repeat(row).zip(line.chars().chain(once('.')).enumerate()))
        .fold(
            (HashMap::<(i32, i32), u32>::new(), 0, String::new()),
            |(mut map, seq, mut s), (row, (col, char))| {
                if char.is_numeric() {
                    map.insert((row as i32, col as i32), seq);
                    s.push(char);
                    (map, seq, s)
                } else {
                    if !s.is_empty() {
                        let num = s.parse::<u32>().unwrap();
                        nums.insert(seq, num);
                        s.clear();
                    }

                    (map, seq + 1, s)
                }
            },
        );

    let adjacent = &adjacent;
    let nums = &nums;
    let product = INPUT
        .lines()
        .enumerate()
        .flat_map(|(row, line)| repeat(row).zip(line.chars().enumerate()))
        .filter(|(_, (_, char))| *char == '*')
        .map(|(row, (col, _))| {
            (-1..=1)
                .flat_map(|i| {
                    (-1..=1).flat_map(move |j| adjacent.get(&(row as i32 + i, col as i32 + j)))
                })
                .collect::<HashSet<_>>()
        })
        .filter(|seqs| seqs.len() == 2)
        .map(|seqs| {
            seqs.iter()
                .map(|seq| nums.get(&seq).unwrap())
                .product::<u32>()
        })
        .sum::<u32>();

    println!("{}", product);
}
