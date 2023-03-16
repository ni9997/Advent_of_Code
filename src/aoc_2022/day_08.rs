use std::{fs, vec};

const DAY: usize = 8;

#[allow(dead_code)]
pub fn run() {
    println!("Day {:02}", DAY);
    let path = format!("input/2022/day_{:02}.txt", DAY);
    let input = fs::read_to_string(path).expect("Wo Datei?");
    println!("The result of part 1 is: {}", part1(&input));
    println!("The result of part 2 is: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {

    let mut grid = vec![];

    for line in input.split('\n') {
        let mut horizontal = vec![];
        for c in line.chars() {
            horizontal.push(c.to_digit(10).unwrap());
        }
        grid.push(horizontal);
    }

    let mut total_visible: usize = 2*grid.len()+2*(grid[0].len()-2);

    for i in 1..grid.len()-1 {
        for j in 1..grid[0].len()-1 {
            // println!("shape: {},{}, x:{} y:{}", grid.len(), grid[0].len(), j, i);
            let height = grid[i][j];
            let mut visible: bool;
            // Check left
            let mut one_bigger = false;
            for x in 0..j {
                if grid[i][x] >= height {
                    one_bigger = true;
                    break;
                }
            }
            visible = !one_bigger;

            // Check right
            let mut one_bigger = false;
            for x in j+1..grid[i].len() {
                if grid[i][x] >= height {
                    one_bigger = true;
                    break;
                }
            }
            visible = visible || !one_bigger;

            // Check top
            let mut one_bigger = false;
            for y in 0..i {
                if grid[y][j] >= height {
                    one_bigger = true;
                    break;
                }
            }
            visible = visible || !one_bigger;

            //Check bottom
            let mut one_bigger = false;
            for y in i+1..grid.len() {
                if grid[y][j] >= height {
                    one_bigger = true;
                    break;
                }
            }
            visible = visible || !one_bigger;

            if visible {
                total_visible += 1;
            }
        }
    }

    total_visible
}

pub fn part2(input: &str) -> u32 {

    let mut grid = vec![];

    for line in input.split('\n') {
        let mut horizontal = vec![];
        for c in line.chars() {
            horizontal.push(c.to_digit(10).unwrap());
        }
        grid.push(horizontal);
    }

    let mut heighest_scenic_score: u32 = 0;

    for i in 1..grid.len()-1 {
        for j in 1..grid[0].len()-1 {
            // println!("shape: {},{}, x:{} y:{}", grid.len(), grid[0].len(), j, i);
            let height = grid[i][j];
            let mut scenic_score: u32 = 1;

            let mut distance = 0;
            
            // Check left
            for x in (0..j).rev() {
                distance += 1;
                if grid[i][x] >= height {
                    break;
                }
            }
            scenic_score *= distance;
            // println!("distance:{}", distance);

            distance = 0;
            // Check right
            for x in j+1..grid[i].len() {
                distance += 1;
                if grid[i][x] >= height {
                    break;
                }
            }
            scenic_score *= distance;
            // println!("distance:{}", distance);

            distance = 0;
            // Check top
            for y in (0..i).rev() {
                distance += 1;
                if grid[y][j] >= height {
                    break;
                }
            }
            scenic_score *= distance;
            // println!("distance:{}", distance);

            distance = 0;
            //Check bottom
            for y in i+1..grid.len() {
                distance += 1;
                if grid[y][j] >= height {
                    break;
                }
            }
            scenic_score *= distance;
            // println!("distance:{}", distance);

            // println!("x:{}, y:{}, scenic:{}, height:{}", j, i, scenic_score, height);

            if scenic_score > heighest_scenic_score {
                heighest_scenic_score = scenic_score;
            }
        }
    }

    heighest_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part1(&input);
        assert_eq!(t, 21);
    }

    #[test]
    fn part2_test1() {
        let path = format!("input/2022/day_{:02}_test_01.txt", DAY);
        let input = fs::read_to_string(path).expect("Wo Datei?");
        let t = part2(&input);
        assert_eq!(t, 8);
    }
}
