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

            // check EAST

            // check SOUTH-EAST

            // check SOUTH

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
}