pub mod aoc_2015;
pub mod aoc_2022;

pub fn run() {
    #[cfg(feature = "2015_base")]
    aoc_2015::run();
    #[cfg(feature = "2022_base")]
    aoc_2022::run();
}
