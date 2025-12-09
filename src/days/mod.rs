pub mod day01;
pub mod day02;
pub mod day03;
//pub mod day04;
//pub mod day05;
//pub mod day06;
//pub mod day07;

type SolvePair = (Option<fn(&str) -> usize>, Option<fn(&str) -> usize>);

pub const SOLVES: [SolvePair; 3] = [
    (Some(day01::s1), Some(day01::s2)),
    (Some(day02::s1), Some(day02::s2)),
    (Some(day03::s1), Some(day03::s2)),
];
