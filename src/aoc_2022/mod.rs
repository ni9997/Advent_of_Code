#[allow(unused_variables)]
#[cfg(feature = "2022_01")]
mod day_01;
#[cfg(feature = "2022_02")]
mod day_02;
#[cfg(feature = "2022_03")]
mod day_03;
#[cfg(feature = "2022_04")]
mod day_04;
#[cfg(feature = "2022_05")]
mod day_05;
#[cfg(feature = "2022_06")]
mod day_06;
#[cfg(feature = "2022_07")]
mod day_07;
#[cfg(feature = "2022_08")]
mod day_08;
#[cfg(feature = "2022_09")]
mod day_09;
#[cfg(feature = "2022_10")]
mod day_10;
#[cfg(feature = "2022_11")]
mod day_11;

pub fn run() {
    println!("Starting Advent of Code 2022");
    #[cfg(feature = "2022_01")]
    day_01::run();
    #[cfg(feature = "2022_02")]
    day_02::run();
    #[cfg(feature = "2022_03")]
    day_03::run();
    #[cfg(feature = "2022_04")]
    day_04::run();
    #[cfg(feature = "2022_05")]
    day_05::run();
    #[cfg(feature = "2022_06")]
    day_06::run();
    #[cfg(feature = "2022_07")]
    day_07::run();
    #[cfg(feature = "2022_08")]
    day_08::run();
    #[cfg(feature = "2022_09")]
    day_09::run();
    #[cfg(feature = "2022_10")]
    day_10::run();
    #[cfg(feature = "2022_11")]
    day_11::run();
}
