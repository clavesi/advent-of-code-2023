fn main() {
    let input = include_str!("../example.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> i32 {
    // Overall total, aka what we're solving for
    let mut total_points: i32 = 0;

    let lines: Vec<String> = input.lines().map(|s| String::from(s.trim())).collect();
    for line in lines {
        let mut card_points = 0;
        let mut first_found = true;

        let cards: Vec<&str> = line.split(&[':', '|']).filter(|&r| r != "").collect();
        // let card_name = cards[0];
        let winning_numbers: Vec<&str> = cards[1].trim().split_whitespace().collect();
        let scratch_numbers: Vec<&str> = cards[2].trim().split_whitespace().collect();

        for number in winning_numbers {
            if scratch_numbers.contains(&number) {
                if first_found {
                    card_points = 1;
                    first_found = false;
                    continue;
                }
                card_points *= 2;
            }
        }

        println!("Card points: {}", card_points);
        total_points += card_points;
    }

    return total_points;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
