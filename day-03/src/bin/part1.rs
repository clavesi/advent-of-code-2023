fn main() {
    let input = include_str!("../example.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> i32 {
    let total: i32 = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1(
            "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..",
        );
        assert_eq!(result, 4361);
    }
}
