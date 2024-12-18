use anyhow::Result;

use aoc_core::{primitives::Rect, text_map::TextMap};

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 4)?;

    let text_map = TextMap::from(puzzle_input.as_str());
    let all_occurences_of_xmas = find_all_xmas(&text_map).unwrap();
    let number_of_occurences = all_occurences_of_xmas.len();

    println!("Number of occurences of XMAS in input is: {}", number_of_occurences);

    assert_eq!(2406, number_of_occurences);

    let all_occurences_of_x_shaped_xmas = find_all_x_shaped_xmas(&text_map).unwrap();
    let number_of_occurences = all_occurences_of_x_shaped_xmas.len();

    println!("Number of occurences of x-shaped XMAS in input is: {}", number_of_occurences);

    assert_eq!(1807, number_of_occurences);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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
                let slice = text_map.slice(x1, y1, x2, y2);

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

fn find_all_x_shaped_xmas(text_map: &TextMap) -> Option<Vec<Rect>> {
    let mut occurences: Vec<Rect> = vec![];

    let pattern_mas_mas = TextMap::from("M.M\n.A.\nS.S");
    let pattern_mas_sam = TextMap::from("M.S\n.A.\nM.S");
    let pattern_sam_mas = TextMap::from("S.M\n.A.\nS.M");
    let pattern_sam_sam = TextMap::from("S.S\n.A.\nM.M");

    for yn in 0 .. text_map.height() - 2 {
        for xn in 0 .. text_map.width() - 2 {
            let r = Rect::new(xn, yn, xn + 2, yn + 2);
            let mut rect_to_match = text_map.rect(r);
            
            rect_to_match.set_char(1, 0, '.');
            rect_to_match.set_char(0, 1, '.');
            rect_to_match.set_char(2, 1, '.');
            rect_to_match.set_char(1, 2, '.');

            if rect_to_match == pattern_mas_mas
                || rect_to_match == pattern_mas_sam
                || rect_to_match == pattern_sam_mas
                || rect_to_match == pattern_sam_sam
            {
                occurences.push(r);
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
    use aoc_core::{primitives::Rect, text_map::TextMap};

    use crate::{find_all_x_shaped_xmas, find_all_xmas};

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

    #[test]
    fn find_all_x_shaped_xmas_should_find_all_occurences_in_example_data() {
        let text_map = TextMap::from(EXAMPLE_DATA);

        assert_eq!(
            9,
            find_all_x_shaped_xmas(&text_map).unwrap().len());
    }
}
