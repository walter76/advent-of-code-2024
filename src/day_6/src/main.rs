use aoc_core::text_map::TextMap;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

const OBSTACLE: char = '#';
const VISITED: char = 'X';

const GUARD_FACING_UP: char = '^';
const GUARD_FACING_RIGHT: char = '>';
const GUARD_FACING_DOWN: char = 'v';

fn find_start_pos(map: &TextMap) -> Option<(usize, usize)> {
    for y in 0 .. map.height() {
        for x in 0 .. map.width() {
            if map.char_at(x, y) == GUARD_FACING_UP {
                return Some((x, y));
            }
        }
    }

    None
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum GuardState {
    LeftMap,
    Moved,
    Turned,
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

    use crate::{find_start_pos, right, up, Guard, GuardState, GUARD_FACING_DOWN, GUARD_FACING_RIGHT, GUARD_FACING_UP, VISITED};

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
    fn find_start_pos_should_return_x4_y6_for_example_data() {
        let map = TextMap::from(EXAMPLE_DATA);

        assert_eq!(Some((4, 6)), find_start_pos(&map));
    }

    #[test]
    fn find_start_pos_should_return_none_if_no_start_pos() {
        let mut map = TextMap::from(EXAMPLE_DATA);

        map.set_char(4, 6, '.');

        assert_eq!(None, find_start_pos(&map));
    }

    #[test]
    fn find_start_pos_should_return_x9_y9_for_last_map_position() {
        let mut map = TextMap::from(EXAMPLE_DATA);

        map.set_char(4, 6, '.');
        map.set_char(9, 9, '^');

        assert_eq!(Some((9, 9)), find_start_pos(&map));
    }

    #[test]
    fn up_is_moving_guard_up() {
        let mut map = TextMap::from(EXAMPLE_DATA);
        let (guard_x, guard_y) = find_start_pos(&map).unwrap();
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
        let (guard_x, guard_y) = find_start_pos(&map).unwrap();
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
}