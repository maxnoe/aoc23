use crate::input::get_input;
use phf::phf_map;


static NUMBERS: phf::Map<char, phf::Map<&str, char>> = phf_map! {
    'z' =>  phf_map! {
        "zero" => '0',
    },
    'o' => phf_map! {
        "one" => '1',
    },
    't' => phf_map! {
        "two" => '2',
        "three" => '3',
    },
    'f' => phf_map! {
        "four" => '4',
        "five" => '5',
    },
    's' => phf_map! {
        "six" => '6',
        "seven" => '7',
    },
    'e' => phf_map! {
        "eight" => '8',
    },
    'n' => phf_map! {
        "nine" => '9',
    }
};

const ZERO: i32 = '0' as i32;


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

fn calibration_value_strings(line: &str) -> i32 {
    let mut first = '\0';
    let mut last = '\0';

    for (pos, chr) in line.chars().enumerate() {

        if chr.is_digit(10) {
            if first == '\0' {
                first = chr;
            }
            last = chr;
            continue;
        }

        match NUMBERS.get(&chr) {
            Some(numbers) => {
                for (number, val) in numbers.entries() {
                    if pos + number.len() > line.len() {
                        continue;
                    }

                    if &&line[pos..pos + number.len()] == number {
                        if first == '\0' {
                            first = *val;
                        }
                        last = *val;
                        break;
                    }

                }
            },
            None => {}
        }
    }

    if first == '\0' || last == '\0' {
        panic!("Must not happen");
    }

    return 10 * (first as i32 - ZERO) + (last as i32 - ZERO);
}

fn part1(input: &str) -> i32 {
    return input.lines().map(calibration_value).sum();
}

fn part2(input: &str) -> i32 {
    return input.lines().map(calibration_value_strings).sum();
}

pub fn day1() {
    let input = get_input(1, 2023).expect("Error getting input");
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
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

    #[test]
    fn test_calibration_value_strings() {
        assert_eq!(calibration_value_strings("two1nine"), 29);
        assert_eq!(calibration_value_strings("eightwothree"), 83);
        assert_eq!(calibration_value_strings("abcone2threexyz"), 13);
        assert_eq!(calibration_value_strings("xtwone3four"), 24);
        assert_eq!(calibration_value_strings("4nineeightseven2"), 42);
        assert_eq!(calibration_value_strings("zoneight234"), 14);
        assert_eq!(calibration_value_strings("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_part2() {
        let input: String = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".into();
        assert_eq!(part2(&input), 281);
    }
}
