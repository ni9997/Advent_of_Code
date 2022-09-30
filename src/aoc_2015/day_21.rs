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

pub struct Entity {
    hit_points: i32,
    damage: u32,
    armor: u32,
}

impl Entity {
    
}

pub fn part1(enemy: &Entity) -> usize {
    panic!("Not implemented")
}

pub fn part2(enemy: &Entity) -> usize {
    panic!("Not implemented")
}

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
