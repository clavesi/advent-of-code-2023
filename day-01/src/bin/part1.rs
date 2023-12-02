fn main() {
    let input = include_str!("../example.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> i32 {
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

        // Collect only the characters in the line that are a number and concatenate them into a string
        let mut line_number: String = "".to_string();
        for char in line.chars() {
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
    fn part1_test() {
        let result = part1("1abc2\r\npqr3stu8vwx\r\na1b2c3d4e5f\r\ntreb7uchet");
        assert_eq!(result, 142);
    }
}
