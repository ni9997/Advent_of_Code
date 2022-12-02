#[allow(unused_variables)]
#[cfg(feature = "2022_01")]
mod day_01;
#[cfg(feature = "2022_02")]
mod day_02;

pub fn run() {
    println!("Starting Advent of Code 2022");
    #[cfg(feature = "2022_01")]
    day_01::run();
    #[cfg(feature = "2022_02")]
    day_02::run();
}
