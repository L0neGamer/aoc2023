use crate::myutils::{files::read_lines, trace};

pub fn sol01() -> i32 {
    let lines = read_lines("./inputs/day01_pt1");

    let mut total = 0;
    for line in lines {
        total += read_int(&line)
    }

    trace::trace("total", total)
}

pub fn read_int(s: &str) -> i32 {
    let vecstr = s.chars();
    let filtered = vecstr.filter(|x| x.is_ascii_digit()).collect::<Vec<_>>();
    
    trace::trace("filtered", &filtered);
    let snew = filtered[0].to_string() + &filtered.last().unwrap().to_string();

    snew.parse::<i32>().unwrap()
}
