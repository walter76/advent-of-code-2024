use text_map::TextMap;

mod text_map;

fn main() {
    println!("Hello, world!");
}

struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn find_all_xmas(text_map: &TextMap) -> Option<Vec<Rect>> {
    let occurences: Vec<Rect> = vec![];

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
                // compare with 'XMAS'
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