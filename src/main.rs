mod day1;
mod day2;
mod day3;
use day1::solution::solution as day1_solution;
use day2::solution::solution as day2_solution;
use day3::solution::solution as day3_solution;

fn main() {
    println!("Day1 ans: {}", day1_solution());
    println!("Day2 ans: {}", day2_solution());
    println!("Day3 ans: {}", day3_solution());
}
