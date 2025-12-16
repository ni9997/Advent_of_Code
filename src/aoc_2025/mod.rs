#[allow(unused_variables)]
#[cfg(feature = "2025_01")]
mod day_01;
#[cfg(feature = "2025_02")]
mod day_02;
#[cfg(feature = "2025_03")]
mod day_03;

pub fn run() {
    println!("Starting Advent of Code 2025");
    #[cfg(feature = "2025_01")]
    day_01::run();
    #[cfg(feature = "2025_02")]
    day_02::run();
    #[cfg(feature = "2025_03")]
    day_03::run();
}
