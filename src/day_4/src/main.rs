mod text_map;

use anyhow::Result;

use aoc_core::primitives::Rect;
use text_map::TextMap;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 4)?;

    let text_map = TextMap::from(puzzle_input.as_str());
    let all_occurences_of_xmas = find_all_xmas(&text_map).unwrap();
    let number_of_occurences = all_occurences_of_xmas.len();

    println!("Number of occurences of XMAS in input is: {}", number_of_occurences);

    assert_eq!(2406, number_of_occurences);

    Ok(())
}

fn find_all_xmas(text_map: &TextMap) -> Option<Vec<Rect>> {
    let mut occurences: Vec<Rect> = vec![];

    // collect all occurences

    for y in 0 .. text_map.height() {
        for x in 0 .. text_map.width() {
            if text_map.char_at(x, y) != 'X' {
                continue;
            }

            // IMPORTANT: We can not move comparing and pushing occurence outside
            // the if-statements, because there can be multiple findings from the
            // current position in a star-like formation.

            let x1 = x;
            let y1 = y;

            // check NORTH
            if y >= 3 {
                let x2 = x1;
                let y2 = y1 - 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();
                
                for yn in (y2..=y1).rev() {
                    slice.push(text_map.char_at(x2, yn));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // check NORTH-EAST
            if y >= 3 && x < text_map.width() - 3 {
                let x2 = x1 + 3;
                let y2 = y1 - 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                let mut xn = x1;
                for yn in (y2..=y1).rev() {
                    slice.push(text_map.char_at(xn, yn));

                    xn += 1;
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // check EAST
            if x < text_map.width() - 3 {
                let x2 = x1 + 3;
                let y2 = y1;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                for xn in x1..=x2 {
                    slice.push(text_map.char_at(xn, y1));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // check SOUTH-EAST
            if y < text_map.height() - 3 && x < text_map.width() - 3 {
                let x2 = x1 + 3;
                let y2 = y1 + 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                let mut xn = x1;
                for yn in y1..=y2 {
                    slice.push(text_map.char_at(xn, yn));

                    xn += 1;
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // check SOUTH
            if y < text_map.height() - 3 {
                let x2 = x1;
                let y2 = y1 + 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                for yn in y1..=y2 {
                    slice.push(text_map.char_at(x1, yn));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // check SOUTH-WEST
            if y < text_map.height() - 3 && x >= 3 {
                let x2 = x1 - 3;
                let y2 = y1 + 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                let mut xn = x1;
                for yn in y1..=y2 {
                    slice.push(text_map.char_at(xn, yn));

                    if xn > 0 {
                        xn -= 1;
                    }
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // check WEST
            if x >= 3 {
                let x2 = x1 - 3;
                let y2 = y1;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                for xn in (x2..=x1).rev() {
                    slice.push(text_map.char_at(xn, y1));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }

            // chest NORTH-WEST
            if y >= 3 && x >= 3 {
                let x2 = x1 - 3;
                let y2 = y1 - 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                let mut xn = x1;
                for yn in (y2..=y1).rev() {
                    slice.push(text_map.char_at(xn, yn));

                    if xn > 0 {
                        xn -= 1;
                    }
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect::new(x1, y1, x2, y2));
                }
            }
        }
    }

    if occurences.is_empty() {
        None
    } else {
        Some(occurences)
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_all_xmas, text_map::TextMap, Rect};

const TEST_DATA_NORTH: &str = r"..S.......
..A.......
..M.......
..X.......";

    #[test]
    fn find_all_xmas_should_find_northern_xmas() {
        let text_map = TextMap::from(TEST_DATA_NORTH);

        assert_eq!(
            vec![Rect {x1: 2, y1: 3, x2: 2, y2: 0}],
            find_all_xmas(&text_map).unwrap());
    }

    const TEST_DATA_NORTH_EAST: &str = r".....S....
....A.....
...M......
..X.......";
    
    #[test]
    fn find_all_xmas_should_find_north_eastern_xmas() {
        let text_map = TextMap::from(TEST_DATA_NORTH_EAST);

        assert_eq!(
            vec![Rect {x1: 2, y1: 3, x2: 5, y2: 0}],
            find_all_xmas(&text_map).unwrap());
    }

    const TEST_DATA_EAST: &str = r"......XMAS
..........
..........
..........";
    
    #[test]
    fn find_all_xmas_should_find_eastern_xmas() {
        let text_map = TextMap::from(TEST_DATA_EAST);

        assert_eq!(
            vec![Rect {x1: 6, y1: 0, x2: 9, y2: 0}],
            find_all_xmas(&text_map).unwrap());
    }

    const TEST_DATA_SOUTH_EAST: &str = r"X.........
.M........
..A.......
...S......";
    
    #[test]
    fn find_all_xmas_should_find_south_eastern_xmas() {
        let text_map = TextMap::from(TEST_DATA_SOUTH_EAST);

        assert_eq!(
            vec![Rect {x1: 0, y1: 0, x2: 3, y2: 3}],
            find_all_xmas(&text_map).unwrap());
    }

    const TEST_DATA_SOUTH: &str = r".........X
.........M
.........A
.........S";

    #[test]
    fn find_all_xmas_should_find_southern_xmas() {
        let text_map = TextMap::from(TEST_DATA_SOUTH);

        assert_eq!(
            vec![Rect {x1: 9, y1: 0, x2: 9, y2: 3}],
            find_all_xmas(&text_map).unwrap());
    }
   
    const TEST_DATA_SOUTH_WEST: &str = r"...X......
..M.......
.A........
S.........";

    #[test]
    fn find_all_xmas_should_find_south_western_xmas() {
        let text_map = TextMap::from(TEST_DATA_SOUTH_WEST);

        assert_eq!(
            vec![Rect {x1: 3, y1: 0, x2: 0, y2: 3}],
            find_all_xmas(&text_map).unwrap());
    }

    const TEST_DATA_WEST: &str = r"..........
..........
..........
SAMX......";

    #[test]
    fn find_all_xmas_should_find_western_xmas() {
        let text_map = TextMap::from(TEST_DATA_WEST);

        assert_eq!(
            vec![Rect {x1: 3, y1: 3, x2: 0, y2: 3}],
            find_all_xmas(&text_map).unwrap());
    }

    const TEST_DATA_NORTH_WEST: &str = r"S.........
.A........
..M.......
...X......";
    
    #[test]
    fn find_all_xmas_should_find_north_western_xmas() {
        let text_map = TextMap::from(TEST_DATA_NORTH_WEST);

        assert_eq!(
            vec![Rect {x1: 3, y1: 3, x2: 0, y2: 0}],
            find_all_xmas(&text_map).unwrap());
    }

    const EXAMPLE_DATA: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn find_all_xmas_should_find_all_occurences_in_example_data() {
        let text_map = TextMap::from(EXAMPLE_DATA);

        assert_eq!(
            18,
            find_all_xmas(&text_map).unwrap().len());
    }
}
