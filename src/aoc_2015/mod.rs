#[allow(unused_variables)]
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

pub fn run() {
    println!("Starting Advent of Code 2015");
    #[cfg(feature = "2015_01")]
    day_01::run();
    #[cfg(feature = "2015_02")]
    day_02::run();
    #[cfg(feature = "2015_03")]
    day_03::run();
    #[cfg(feature = "2015_04")]
    day_04::run();
    #[cfg(feature = "2015_05")]
    day_05::run();
    #[cfg(feature = "2015_06")]
    day_06::run();
    #[cfg(feature = "2015_07")]
    day_07::run();
    #[cfg(feature = "2015_08")]
    day_08::run();
    #[cfg(feature = "2015_09")]
    day_09::run();
    #[cfg(feature = "2015_10")]
    day_10::run();
    #[cfg(feature = "2015_11")]
    day_11::run();
    #[cfg(feature = "2015_12")]
    day_12::run();
    #[cfg(feature = "2015_13")]
    day_13::run();
    #[cfg(feature = "2015_14")]
    day_14::run();
    #[cfg(feature = "2015_15")]
    day_15::run();
    #[cfg(feature = "2015_16")]
    day_16::run();
    #[cfg(feature = "2015_17")]
    day_17::run();
    #[cfg(feature = "2015_18")]
    day_18::run();
    #[cfg(feature = "2015_19")]
    day_19::run();
    #[cfg(feature = "2015_20")]
    day_20::run();
}
