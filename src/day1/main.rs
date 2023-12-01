const INPUT: &str = include_str!("input.txt");

static NUMBERS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn preprocess_line(mut line: String) -> String {
    for (s, n) in NUMBERS {
        // Replace "one" with "one1one". This ensures that overlapping numbers like "twone" still
        // parse properly regardless of the replace order. The extraneous chars are stripped anyway.
        line = line.replace(s, &*format!("{s}{n}{s}"));
    }
    line
}

fn compute_sum<I>(lines: I) -> u32
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    lines
        // Convert each line into a vec of digits
        .map(|line| {
            line.as_ref()
                .chars()
                .filter_map(|c| char::to_digit(c, 10))
                .collect::<Vec<_>>()
        })
        // Combine first and last digit
        .map(|nums| nums.first().unwrap() * 10 + nums.last().unwrap())
        .sum()
}

fn main() {
    println!("Part 1: {}", compute_sum(INPUT.lines()));

    println!(
        "Part 2: {}",
        compute_sum(INPUT.lines().map(|l| preprocess_line(l.to_owned())))
    );
}
