use text_map::TextMap;

mod text_map;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
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

            // check NORTH
            if y >= 3 {
                let x1 = x;
                let y1 = y;
                let x2 = x;
                let y2 = y1 - 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();
                
                for yn in (y2..=y1).rev() {
                    slice.push(text_map.char_at(x, yn));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect {x1, y1, x2, y2});
                }
            }

            // check NORTH-EAST
            if y >= 3 && x < text_map.width() - 3 {
                let x1 = x;
                let y1 = y;
                let x2 = x + 3;
                let y2 = y - 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                let mut xn = x1;
                for yn in (y2..=y1).rev() {
                    slice.push(text_map.char_at(xn, yn));

                    xn += 1;
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect {x1, y1, x2, y2});
                }
            }

            // check EAST
            if x < text_map.width() - 3 {
                let x1 = x;
                let y1 = y;
                let x2 = x + 3;
                let y2 = y;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                for xn in x1..=x2 {
                    slice.push(text_map.char_at(xn, y));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect {x1, y1, x2, y2});
                }
            }

            // check SOUTH-EAST
            if y < text_map.height() -3 && x < text_map.width() - 3 {
                let x1 = x;
                let y1 = y;
                let x2 = x + 3;
                let y2 = y + 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                let mut xn = x1;
                for yn in y1..=y2 {
                    slice.push(text_map.char_at(xn, yn));

                    xn += 1;
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect {x1, y1, x2, y2});
                }
            }

            // check SOUTH
            if y < text_map.height() {
                let x1 = x;
                let y1 = y;
                let x2 = x;
                let y2 = y + 3;

                // get slice from (x1, y1) to (x2, y2)
                let mut slice = String::new();

                for yn in y1..=y2 {
                    slice.push(text_map.char_at(x, yn));
                }

                // compare with 'XMAS'
                if slice == "XMAS" {
                    occurences.push(Rect {x1, y1, x2, y2});
                }
            }

            // check SOUTH-WEST

            // check WEST

            // chest NORTH-WEST
        }
    }

    // remove duplicate findings

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
   
}
