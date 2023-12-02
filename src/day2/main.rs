const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_round(input: &str) -> Round {
    let mut round = Round::default();

    for color in input.split(",") {
        if let Some(num) = color.strip_suffix("red") {
            round.red = num.parse().unwrap();
        } else if let Some(num) = color.strip_suffix("green") {
            round.green = num.parse().unwrap();
        } else if let Some(num) = color.strip_suffix("blue") {
            round.blue = num.parse().unwrap();
        } else {
            panic!()
        }
    }

    round
}

fn parse_game(line: &str) -> Game {
    let line = line.replace(" ", "");
    let line = line.strip_prefix("Game").unwrap();

    let (id, line) = {
        let mut iter = line.split(":");
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut rounds = Vec::new();
    for round in line.split(";") {
        rounds.push(parse_round(round));
    }

    Game {
        id: id.parse().unwrap(),
        rounds,
    }
}

fn main() {
    let games = INPUT.lines().map(parse_game).collect::<Vec<_>>();
    println!(
        "{}",
        games
            .iter()
            .filter(|game| game
                .rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14))
            .map(|game| game.id)
            .sum::<u32>()
    );

    println!(
        "{}",
        games
            .iter()
            .map(|game| {
                let (r, g, b) = game.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                    (r.max(round.red), g.max(round.green), b.max(round.blue))
                });
                r * g * b
            })
            .sum::<u32>()
    )
}
