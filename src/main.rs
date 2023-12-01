mod day01;

pub mod myutils;

fn main() {
    println!("Hello, world!");

    myutils::trace::trace("day01 pt1", day01::sol01());
    myutils::trace::trace("day01 pt2", day01::sol02());
}
