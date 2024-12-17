use aoc_core::text_map::TextMap;

use anyhow::Result;

fn main() -> Result<()> {
    let puzzle_input = aoc_core::get_input(2024, 6)?;

    let mut map = TextMap::from(puzzle_input.as_str());

    move_guard_till_leaves_map(&mut map);

    println!(
        "The guard visits {} distinct positions before leaving the mapped area.",
        map.count_chars(VISITED));

    let map = TextMap::from(puzzle_input.as_str());
    let possible_loops = detect_and_count_possible_loops(&map);

    println!(
        "There are {} possible positions to trap the guard in a loop.",
        possible_loops);

    Ok(())
}

const OBSTACLE: char = '#';
const VISITED: char = 'X';
const WALKABLE: char = '.';

const GUARD_FACING_UP: char = '^';
const GUARD_FACING_RIGHT: char = '>';
const GUARD_FACING_DOWN: char = 'v';
const GUARD_FACING_LEFT: char = '<';

#[derive(Eq, PartialEq, Debug, Clone)]
enum GuardState {
    LeftMap,
    Moved,
    Turned,
}

fn detect_and_count_possible_loops(map: &TextMap) -> usize {
    let possibilities_to_check = map.count_chars(WALKABLE);
    let mut possibilties_checked = 0;

    let mut possible_loops = 0;

    for y in 0 .. map.height() {
        for x in 0 .. map.height() {
            if map.char_at(x, y) == WALKABLE {
                let mut changed_map = map.clone();
                changed_map.set_char(x, y, OBSTACLE);

                if move_guard_and_check_for_loop(&mut changed_map) {
                    possible_loops += 1;
                }

                possibilties_checked += 1;
                
                println!("checked ({}/{})", possibilties_checked, possibilities_to_check);
            }
        }
    }

    possible_loops
}

fn move_guard_and_check_for_loop(map: &mut TextMap) -> bool {
    let (guard_x, guard_y) = map.find_char_pos(GUARD_FACING_UP).unwrap();
    let mut guard = Guard::new(guard_x, guard_y);
    let mut guard_path: Vec<(usize, usize, char)> = Vec::new();

    guard_path.push((guard.x(), guard.y(), GUARD_FACING_UP));

    loop {
        let guard_state = move_guard(&mut guard, map, &mut guard_path);

        if guard_state == Some(GuardState::LeftMap) {
            return false;
        }

        let visited =
            (guard.x(), guard.y(), map.char_at(guard.x(), guard.y()));

        if guard_path.contains(&visited) {
            return true;
        } else {
            guard_path.push(visited);
        }
    }
}

fn move_guard_till_leaves_map(map: &mut TextMap) {
    let (guard_x, guard_y) = map.find_char_pos(GUARD_FACING_UP).unwrap();
    let mut guard = Guard::new(guard_x, guard_y);
    let mut guard_path: Vec<(usize, usize, char)> = Vec::new();

    guard_path.push((guard.x(), guard.y(), GUARD_FACING_UP));

    while move_guard(&mut guard, map, &mut guard_path) != Some(GuardState::LeftMap) {}
}

fn move_guard(guard: &mut Guard, map: &mut TextMap, guard_path: &mut Vec<(usize, usize, char)>) -> Option<GuardState> {
    let guard_x = guard.x();
    let guard_y = guard.y();

    match map.char_at(guard_x, guard_y) {
        GUARD_FACING_UP => Some(up(guard, map)),
        GUARD_FACING_RIGHT => Some(right(guard, map)),
        GUARD_FACING_DOWN => Some(down(guard, map)),
        GUARD_FACING_LEFT => Some(left(guard, map)),
        _ => None,
    }
}

fn up(guard: &mut Guard, map: &mut TextMap) -> GuardState {
    let guard_x = guard.x();
    let guard_y = guard.y();

    if guard_y == 0 {
        map.set_char(guard_x, guard_y, VISITED);

        GuardState::LeftMap
    } else if map.char_at(guard_x, guard_y - 1) == OBSTACLE {
        map.set_char(guard_x, guard_y, GUARD_FACING_RIGHT);

        GuardState::Turned
    } else {
        map.set_char(guard_x, guard_y, VISITED);
        map.set_char(guard_x, guard_y - 1, GUARD_FACING_UP);

        guard.set_pos(guard_x, guard_y - 1);

        GuardState::Moved
    }
}

fn right(guard: &mut Guard, map: &mut TextMap) -> GuardState {
    let guard_x = guard.x();
    let guard_y = guard.y();

    if guard_x == map.width() - 1 {
        map.set_char(guard_x, guard_y, VISITED);

        GuardState::LeftMap
    } else if map.char_at(guard_x + 1, guard_y) == OBSTACLE {
        map.set_char(guard_x, guard_y, GUARD_FACING_DOWN);

        GuardState::Turned
    } else {
        map.set_char(guard_x, guard_y, VISITED);
        map.set_char(guard_x + 1, guard_y, GUARD_FACING_RIGHT);

        guard.set_pos(guard_x + 1, guard_y);

        GuardState::Moved
    }
}

fn down(guard: &mut Guard, map: &mut TextMap) -> GuardState {
    let guard_x = guard.x();
    let guard_y = guard.y();

    if guard_y == map.height() - 1 {
        map.set_char(guard_x, guard_y, VISITED);

        GuardState::LeftMap
    } else if map.char_at(guard_x, guard_y + 1) == OBSTACLE {
        map.set_char(guard_x, guard_y, GUARD_FACING_LEFT);

        GuardState::Turned
    } else {
        map.set_char(guard_x, guard_y, VISITED);
        map.set_char(guard_x, guard_y + 1, GUARD_FACING_DOWN);

        guard.set_pos(guard_x, guard_y + 1);

        GuardState::Moved
    }
}

fn left(guard: &mut Guard, map: &mut TextMap) -> GuardState {
    let guard_x = guard.x();
    let guard_y = guard.y();

    if guard_x == 0 {
        map.set_char(guard_x, guard_y, VISITED);

        GuardState::LeftMap
    } else if map.char_at(guard_x - 1, guard_y) == OBSTACLE {
        map.set_char(guard_x, guard_y, GUARD_FACING_UP);

        GuardState::Turned
    } else {
        map.set_char(guard_x, guard_y, VISITED);
        map.set_char(guard_x - 1, guard_y, GUARD_FACING_LEFT);

        guard.set_pos(guard_x - 1, guard_y);

        GuardState::Moved
    }
}

struct Guard {
    x: usize,
    y: usize,
}

impl Guard {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use aoc_core::text_map::TextMap;

    use crate::{detect_and_count_possible_loops, down, left, move_guard_and_check_for_loop, move_guard_till_leaves_map, right, up, Guard, GuardState, GUARD_FACING_DOWN, GUARD_FACING_LEFT, GUARD_FACING_RIGHT, GUARD_FACING_UP, VISITED};

    const EXAMPLE_DATA: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn up_is_moving_guard_up() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let (guard_x, guard_y) = map.find_char_pos(GUARD_FACING_UP).unwrap();
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Moved, up(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y - 1, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
        assert_eq!(GUARD_FACING_UP, map.char_at(guard.x(), guard.y()));

    }

    #[test]
    fn up_is_turning_right_on_obstacle() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 4;
        let guard_y = 1;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Turned, up(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(GUARD_FACING_RIGHT, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn up_is_leaving_map() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 3;
        let guard_y = 0;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::LeftMap, up(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn right_is_moving_guard_right() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let (guard_x, guard_y) = map.find_char_pos(GUARD_FACING_UP).unwrap();
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Moved, right(&mut guard, &mut map));

        assert_eq!(guard_x + 1, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
        assert_eq!(GUARD_FACING_RIGHT, map.char_at(guard.x(), guard.y()));
    }

    #[test]
    fn right_is_turning_down_on_obstacle() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 8;
        let guard_y = 1;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Turned, right(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(GUARD_FACING_DOWN, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn right_is_leaving_map() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 9;
        let guard_y = 2;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::LeftMap, right(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn down_is_moving_guard_down() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let (guard_x, guard_y) = map.find_char_pos(GUARD_FACING_UP).unwrap();
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Moved, down(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y + 1, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
        assert_eq!(GUARD_FACING_DOWN, map.char_at(guard.x(), guard.y()));
    }

    #[test]
    fn down_is_turning_left_on_obstacle() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 6;
        let guard_y = 8;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Turned, down(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(GUARD_FACING_LEFT, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn down_is_leaving_map() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 4;
        let guard_y = 9;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::LeftMap, down(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn left_is_moving_guard_left() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let (guard_x, guard_y) = map.find_char_pos(GUARD_FACING_UP).unwrap();
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Moved, left(&mut guard, &mut map));

        assert_eq!(guard_x - 1, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
        assert_eq!(GUARD_FACING_LEFT, map.char_at(guard.x(), guard.y()));
    }

    #[test]
    fn left_is_turning_up_on_obstacle() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 3;
        let guard_y = 3;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::Turned, left(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(GUARD_FACING_UP, map.char_at(guard_x, guard_y));
    }

    #[test]
    fn left_is_leaving_map() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let guard_x = 0;
        let guard_y = 2;
        let mut guard = Guard::new(guard_x, guard_y);

        assert_eq!(GuardState::LeftMap, left(&mut guard, &mut map));

        assert_eq!(guard_x, guard.x());
        assert_eq!(guard_y, guard.y());

        assert_eq!(VISITED, map.char_at(guard_x, guard_y));
    }

const VISITED_LOCATIONS: &str = r"....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..";

    #[test]
    fn move_guard_till_leaves_map_should_mark_all_visited_locations_for_example_data() {
        let mut map = TextMap::from(EXAMPLE_DATA);

        move_guard_till_leaves_map(&mut map);

        assert_eq!(TextMap::from(VISITED_LOCATIONS), map);
    }

    #[test]
    fn count_chars_should_return_41_for_example_data() {
        let mut map = TextMap::from(EXAMPLE_DATA);

        move_guard_till_leaves_map(&mut map);

        assert_eq!(41, map.count_chars(VISITED));
    }

const EXAMPLE_DATA_LOOP_1: &str = r"....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.#^---+.
........#.
#.........
......#...";

    #[test]
    fn move_guard_and_check_for_loop_should_return_true_for_loop_1() {
        let mut map = TextMap::from(EXAMPLE_DATA_LOOP_1);

        assert!(move_guard_and_check_for_loop(&mut map));
    }

    #[test]
    fn move_guard_and_check_for_loop_should_return_false_for_example_data() {
        let mut map = TextMap::from(EXAMPLE_DATA);

        assert!(!move_guard_and_check_for_loop(&mut map));
    }

const EXAMPLE_DATA_LOOP_2: &str = r"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
......#.#.
#.........
......#...";

    #[test]
    fn move_guard_and_check_for_loop_should_return_true_for_loop_2() {
        let mut map = TextMap::from(EXAMPLE_DATA_LOOP_2);

        assert!(move_guard_and_check_for_loop(&mut map));
    }

    #[test]
    fn detect_and_count_possible_loops_should_return_6_for_example_data() {
        let map = TextMap::from(EXAMPLE_DATA);

        assert_eq!(6, detect_and_count_possible_loops(&map))
    }
}
