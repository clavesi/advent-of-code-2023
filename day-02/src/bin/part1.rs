use std::collections::HashMap;

fn main() {
    let input = include_str!("../example.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> u32 {
    let max_counts: HashMap<&str, u8> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut total: u32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    'line_loop: for line in lines {
        let split_game_and_sets: Vec<&str> = line.split(": ").collect();

        let game_info: Vec<&str> = split_game_and_sets[0].split(' ').collect();
        let game_number = game_info[1];
        // println!("{game_number}");

        let sets: Vec<&str> = split_game_and_sets[1].split("; ").collect();
        for set in sets.iter() {
            let cubes = set.split(", ");
            for cube in cubes {
                let [count, color]: [&str; 2] = cube
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap_or_default();
                let max_amount = *max_counts.get(color).unwrap();
                if count.parse::<u8>().unwrap() > max_amount {
                    continue 'line_loop;
                }
            }
        }

        println!("Contains {game_number}");
        total += game_number.to_string().parse::<u32>().unwrap();
    }

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        dbg!("{result}");
        assert_eq!(result, 8);
    }
}
