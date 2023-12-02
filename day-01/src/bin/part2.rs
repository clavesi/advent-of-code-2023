use regex::{Captures, Regex};

fn main() {
    let input = include_str!("../example.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> i32 {
    // Overall total, aka what we're solving for
    let mut total: i32 = 0;
    // Convert text file into vector by removing newlines, which in Windows is \r\n
    let lines: Vec<String> = input.split("\r\n").map(|s| s.to_string()).collect();

    for line in &lines {
        // Ignore line at end of file, otherwise converting the string to an int throws an error
        // Probably a different way to handle that problem, but this is my first draft
        if line.is_empty() {
            continue;
        }

        // Use regex to replace all digits spelled out including one letter overlaps with their corresponding digit(s)
        let rx = Regex::new("(oneight|twone|threeight|fiveight|sevenine|eightwo|eighthree|nineight|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let line_fixed = rx.replace_all(line, |cap: &Captures| {
            match &cap[0] {
                "oneight" => "18",
                "twone" => "21",
                "threeight" => "38",
                "fiveight" => "58",
                "sevenine" => "79",
                "eightwo" => "82",
                "eighthree" => "83",
                "nineight" => "98",
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => panic!("Error!"),
            }
            .to_string()
        });

        // Collect only the characters in the line that are a number and concatenate them into a string
        let mut line_number: String = "".to_string();
        for char in line_fixed.chars() {
            if char.is_numeric() {
                line_number.push_str(&char.to_string());
            }
        }
        let first_and_last_digit: String = line_number.chars().nth(0).unwrap().to_string()
            + &line_number.chars().nth_back(0).unwrap().to_string();

        // Convert that string from above into an integer and add it to the total
        total += first_and_last_digit.parse::<i32>().unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let result: i32 = part2("two1nine\r\neightwothree\r\nabcone2threexyz\r\nxtwone3four\r\n4nineeightseven2\r\nzoneight234\r\n7pqrstsixteen");
        assert_eq!(result, 281);
    }
}
