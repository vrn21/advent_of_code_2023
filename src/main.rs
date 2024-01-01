mod bin{
    pub mod day1;
    pub mod day2;
}

fn main() {
    println!("Answers for Advent of code 2023 are" );
    println!("Day 1 part one : {}",bin::day1::run());

    println!("Day 2 part one : {}",bin::day2::run());
}
