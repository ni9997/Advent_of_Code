#[allow(dead_code)]
pub fn run() {
    println!("Day 21");
    let enemy = Entity {
        hit_points: 103,
        damage: 9,
        armor: 2,
    };
    println!("The result of part 1 is: {}", part1(&enemy));
    println!("The result of part 2 is: {}", part2(&enemy));
}
#[allow(dead_code)]

pub struct Entity {
    hit_points: i32,
    damage: u32,
    armor: u32,
}
#[allow(dead_code)]
impl Entity {}
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn part1(enemy: &Entity) -> usize {
    panic!("Not implemented")
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn part2(enemy: &Entity) -> usize {
    panic!("Not implemented")
}

#[cfg(feature = "2015_01")]
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {}

    #[test]
    fn part1_test2() {}

    #[test]
    fn part2_test1() {}

    #[test]
    fn part2_test2() {}
}
