use std::ops::{Add, AddAssign};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn as_pair(self: Self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn build_traversal(self) -> Vec<Position> {
        let i_traversal: Vec<usize> = match self {
            Direction::Down => (0..4).rev().collect(),
            _ => (0..4).collect(),
        };

        let j_traversal: Vec<usize> = match self {
            Direction::Right => (0..4).rev().collect(),
            _ => (0..4).collect(),
        };

        i_traversal
            .iter()
            .flat_map(|i| j_traversal.iter().map(move |j| Position::new(*i, *j)))
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    i: usize,
    j: usize,
}

impl Position {
    pub fn new(i: usize, j: usize) -> Position {
        Position { i, j }
    }

    pub fn from_index(index: usize) -> Position {
        Position {
            i: index / 4,
            j: index % 4,
        }
    }

    pub fn index(self) -> usize {
        self.i * 4 + self.j
    }

    pub fn is_out_of_bounds(self) -> bool {
        self.i >= 4 || self.j >= 4
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Self::Output {
        let (i, j) = direction.as_pair();

        Position {
            i: (self.i as i32 + i) as usize,
            j: (self.j as i32 + j) as usize,
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        *self = *self + direction
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Tile {
    number: i32,
}

impl Tile {
    fn new(number: i32) -> Tile {
        Tile { number }
    }
}

type Cell = Option<Tile>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Grid {
    cells: [Cell; 16],
}

impl Grid {
    pub fn new(cells: [Cell; 16]) -> Grid {
        Grid { cells }
    }

    fn get(&self, position: Position) -> Option<Tile> {
        self.cells.get(position.index()).and_then(|tile| *tile)
    }

    pub fn move_in(&mut self, direction: Direction) {
        let traversal = direction.build_traversal();

        let mut moved = false;

        for start_position in traversal {
            self.traverse_from(start_position, direction);
        }
    }

    fn traverse_from(&mut self, start_position: Position, in_direction: Direction) {
        let start_tile = match self.get(start_position) {
            Some(tile) => tile,
            None => return,
        };

        let mut new_position = start_position;

        loop {
            let next_position = new_position + in_direction;

            if next_position.is_out_of_bounds() {
                break;
            }

            if self.get(next_position).is_some() {
                break;
            }

            new_position = next_position;
        }

        self.cells[start_position.index()] = None;
        self.cells[new_position.index()] = Some(start_tile);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Grid, Tile};
    use std::convert::TryInto;

    #[test]
    fn it_works() {
        struct TestCase {
            state: [i32; 16],
            expected: [i32; 16],
            direction: Direction,
        }

        let test_cases = [
            TestCase {
                #[rustfmt::skip]
                state: [
                    0, 0, 0, 0,
                    0, 2, 2, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                ],
                #[rustfmt::skip]
                expected: [
                    0, 2, 2, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                ],
                direction: Direction::Up,
            },
            TestCase {
                #[rustfmt::skip]
                state: [
                    0, 0, 0, 0,
                    0, 2, 2, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                ],
                #[rustfmt::skip]
                expected: [
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                    0, 0, 0, 0,
                    0, 2, 2, 0,
                ],
                direction: Direction::Down,
            },
            TestCase {
                #[rustfmt::skip]
                state: [
                    0, 0, 0, 0,
                    0, 2, 0, 0,
                    0, 2, 0, 0,
                    0, 0, 0, 0,
                ],
                #[rustfmt::skip]
                expected: [
                    0, 0, 0, 0,
                    2, 0, 0, 0,
                    2, 0, 0, 0,
                    0, 0, 0, 0,
                ],
                direction: Direction::Left,
            },
            TestCase {
                #[rustfmt::skip]
                state: [
                    0, 0, 0, 0,
                    0, 2, 0, 0,
                    0, 2, 0, 0,
                    0, 0, 0, 0,
                ],
                #[rustfmt::skip]
                expected: [
                    0, 0, 0, 0,
                    0, 0, 0, 2,
                    0, 0, 0, 2,
                    0, 0, 0, 0,
                ],
                direction: Direction::Right,
            },
            TestCase {
                #[rustfmt::skip]
                state: [
                    0, 0, 2, 0,
                    0, 2, 4, 0,
                    4, 0, 0, 0,
                    4, 0, 2, 0,
                ],
                #[rustfmt::skip]
                expected: [
                    0, 0, 0, 2,
                    0, 0, 2, 4,
                    0, 0, 0, 4,
                    0, 0, 4, 2,
                ],
                direction: Direction::Right,
            },
        ];

        for case in &test_cases {
            let mut state = make_grid(case.state);
            let expected = make_grid(case.expected);

            state.move_in(case.direction);

            assert_eq!(state, expected);
        }
    }

    fn make_grid(from_numbers: [i32; 16]) -> Grid {
        Grid::new(
            from_numbers
                .iter()
                .map(|number| {
                    if *number > 0 {
                        Some(Tile::new(*number))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<Tile>>>()
                .try_into()
                .unwrap(),
        )
    }
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
