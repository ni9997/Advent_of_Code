pub mod aoc_2015;
pub mod aoc_2016;
pub mod aoc_2022;
pub mod utils;

pub fn run() {
    #[cfg(feature = "2015_base")]
    aoc_2015::run();
    #[cfg(feature = "2016_base")]
    aoc_2016::run();
    #[cfg(feature = "2022_base")]
    aoc_2022::run();
}
