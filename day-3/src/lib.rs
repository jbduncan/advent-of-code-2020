use itertools::{iterate, Itertools};
use ndarray::Array2;
use thiserror::Error;

pub fn trees_encountered(map: &str) -> Result<u32, Day3Error> {
    trees_encountered_impl(map, Direction { right: 3, down: 1 })
}

pub fn trees_encountered_in_direction(map: &str, direction: Direction) -> Result<u32, Day3Error> {
    trees_encountered_impl(map, direction)
}

fn trees_encountered_impl(map: &str, direction: Direction) -> Result<u32, Day3Error> {
    if map.contains(|c: char| c != '.' && c != '#' && c != '\n') {
        todo!()
    }

    if !is_rectangular(map) {
        todo!()
    }

    let map = to_2d_array(map)?;

    struct Point {
        row: usize,
        column: usize,
    }

    let width = map.ncols();
    let height = map.nrows();
    let next_point = |point: &Point| Point {
        row: point.row + (direction.down as usize),
        column: (point.column + (direction.right as usize)) % width,
    };
    let points =
        iterate(Point { row: 0, column: 0 }, next_point).take_while(|point| point.row < height);
    let result = points
        .map(|point| map[(point.row, point.column)])
        .filter(|part| *part == '#')
        .map(|_| 1u32)
        .fold(0u32, |acc, i| acc.saturating_add(i));
    Ok(result)
}

fn is_rectangular(map: &str) -> bool {
    map.lines().map(|row| row.len()).all_equal()
}

fn to_2d_array(map: &str) -> Result<Array2<char>, Day3Error> {
    let width = map
        .lines()
        .map(|row| row.len())
        .next()
        .unwrap_or_else(|| todo!());
    let height = map.lines().count();

    let mut result = Array2::<char>::default((height, width));
    for (r, line) in map.lines().enumerate() {
        for (c, character) in line.char_indices() {
            result[[r, c]] = character;
        }
    }
    Ok(result)
}

pub struct Direction {
    pub right: u32,
    pub down: u32,
}

#[derive(Error, Debug)]
pub enum Day3Error {
    #[error("map was expected to contain just '.' and '#' characters, but it also contained other characters")]
    InvalidMap,
}

#[cfg(test)]
mod given_single_tile_map_with_no_trees {
    use super::*;

    mod when_sliding_down_map {
        use super::*;

        #[test]
        fn then_we_encounter_no_trees() -> Result<(), Box<dyn std::error::Error>> {
            let map = ".";

            let result = trees_encountered(&map)?;

            assert_eq!(result, 0);

            Ok(())
        }
    }
}

#[cfg(test)]
mod given_large_map_with_no_trees {
    use super::*;
    use indoc::indoc;

    mod when_sliding_down_map {
        use super::*;

        #[test]
        fn then_we_encounter_no_trees() -> Result<(), Box<dyn std::error::Error>> {
            let map = indoc! { "
                ...........
                ...........
                ...........
                ...........
                ...........
                ...........
                ...........
                ...........
                ...........
                ...........
                ..........."
            };

            let result = trees_encountered(&map)?;

            assert_eq!(result, 0);

            Ok(())
        }
    }
}

#[cfg(test)]
mod given_large_map_with_trees {
    use super::*;
    use indoc::indoc;

    mod when_sliding_down_map {
        use super::*;

        #[test]
        fn then_we_encounter_some_trees() -> Result<(), Box<dyn std::error::Error>> {
            let map = indoc! { "
                ..##.......
                #...#...#..
                .#....#..#.
                ..#.#...#.#
                .#...##..#.
                ..#.##.....
                .#.#.#....#
                .#........#
                #.##...#...
                #...##....#
                .#..#...#.#"
            };

            let result = trees_encountered(&map)?;

            assert_eq!(result, 7);

            Ok(())
        }
    }

    mod when_sliding_down_map_in_many_directions {
        use super::*;
        use test_case::test_case;

        #[test_case(Direction { right: 1, down: 1 }, 2 ; "sliding right 1, down 1")]
        #[test_case(Direction { right: 3, down: 1 }, 7 ; "sliding right 3, down 1")]
        #[test_case(Direction { right: 5, down: 1 }, 3 ; "sliding right 5, down 1")]
        #[test_case(Direction { right: 7, down: 1 }, 4 ; "sliding right 7, down 1")]
        #[test_case(Direction { right: 1, down: 2 }, 2 ; "sliding right 1, down 2")]
        fn then_we_encounter_some_trees(
            direction: Direction,
            expected_result: u32,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let map = indoc! { "
                ..##.......
                #...#...#..
                .#....#..#.
                ..#.#...#.#
                .#...##..#.
                ..#.##.....
                .#.#.#....#
                .#........#
                #.##...#...
                #...##....#
                .#..#...#.#"
            };

            let result = trees_encountered_in_direction(&map, direction)?;

            assert_eq!(result, expected_result);

            Ok(())
        }
    }
}

#[cfg(test)]
mod given_puzzle_input_map_from_advent_of_code_2020 {
    use super::*;

    mod when_sliding_down_map {
        use super::*;

        #[test]
        fn then_we_should_encounter_207_trees() -> Result<(), Box<dyn std::error::Error>> {
            let map = std::fs::read_to_string("tests/input.txt")?;

            let result = trees_encountered(&map)?;

            assert_eq!(result, 207);

            Ok(())
        }
    }
}
