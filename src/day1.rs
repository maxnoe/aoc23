use crate::input::get_input;



fn calibration_value(line: &str) -> i32 {
    let mut first = '\0';
    let mut last = '\0';

    for chr in line.chars() {
        if !chr.is_digit(10) {continue};
        if first == '\0' {
            first = chr;
        }
        last = chr;
    }

    let zero = '0' as i32;
    return 10 * (first as i32 - zero) + (last as i32 - zero);
}

fn part1(input: &String) -> i32 {
    let mut answer = 0;

    for line in input.lines() {
        let val = calibration_value(line);
        answer += val;
    }
    return answer;
}

pub fn day1() {
    let input = get_input(1, 2023).expect("Error getting input");
    let answer = part1(&input);
    println!("{}", answer);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("1abc2"), 12);
        assert_eq!(calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(calibration_value("treb7uchet"), 77);
    }


    #[test]
    fn test_part1() {
        let input: String = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".into();
        assert_eq!(part1(&input), 142);
    }
}
