mod day01;

pub mod myutils;

fn main() {
    println!("Hello, world!");

    myutils::trace::trace("day01", day01::sol01());
}
