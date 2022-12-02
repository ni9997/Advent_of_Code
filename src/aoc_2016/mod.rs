#[allow(unused_variables)]
#[cfg(feature = "2016_01")]
mod day_01;
// #[cfg(feature = "2016_02")]
// mod day_02;

pub fn run() {
    println!("Starting Advent of Code 2016");
    #[cfg(feature = "2016_01")]
    day_01::run();
    // #[cfg(feature = "2016_02")]
    // day_02::run();
}
