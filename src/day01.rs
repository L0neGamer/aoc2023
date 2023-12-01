use crate::myutils::{files::read_lines};

pub fn sol01() -> i32 {
    let lines = read_lines("./inputs/day01_pt1");

    let mut total = 0;
    for line in lines {
        total += read_int(&line)
    }

    total
}

fn read_int(s: &str) -> i32 {
    let vecstr = s.chars();
    let filtered = vecstr.filter(|x| x.is_ascii_digit()).collect::<Vec<_>>();

    let snew = filtered[0].to_string() + &filtered.last().unwrap().to_string();

    snew.parse::<i32>().unwrap()
}

pub fn sol02() -> i32 {
    let lines = read_lines("./inputs/day01_pt1");

    let mut total = 0;
    for line in lines {
        let mut digits: Vec<char> = Vec::new();

        let s = recurse_on_str(&mut digits, &line);
        total += (s[0].to_string() + &s.last().unwrap().to_string())
            .parse::<i32>()
            .unwrap()
    }

    total
}

fn recurse_on_str<'a>(digits: &'a mut Vec<char>, s: &str) -> &'a mut Vec<char> {
    match s.chars().next() {
        None => {
            return digits;
        }
        Some(c) => {
            if c.is_ascii_digit() {
                digits.push(c);
                return recurse_on_str(digits, s.get(1..).unwrap());
            }
        }
    }

    let lookup = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for (name, d) in lookup {
        if s.starts_with(name) {
            digits.push(d);
            return recurse_on_str(digits, s.get(1..).unwrap());
        }
    }

    return recurse_on_str(digits, s.get(1..).unwrap());
}
