pub mod aoc_2015;
pub mod aoc_2016;
pub mod aoc_2022;
pub mod aoc_2023;
pub mod aoc_2025;
pub mod utils;

pub fn run() {
    #[cfg(feature = "2015_base")]
    aoc_2015::run();
    #[cfg(feature = "2016_base")]
    aoc_2016::run();
    #[cfg(feature = "2022_base")]
    aoc_2022::run();
    #[cfg(feature = "2023_base")]
    aoc_2023::run();
    #[cfg(feature = "2025_base")]
    aoc_2025::run();
}
