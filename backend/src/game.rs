use crate::api::Error;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write},
    ops::{Add, Div, Mul},
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Player {
    White,
    Black,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Player::White => 'W',
            Player::Black => 'B',
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PieceType {
    Man,
    King,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            PieceType::Man => 'M',
            PieceType::King => 'K',
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Piece {
    #[serde(rename = "type")]
    pub type_: PieceType,
    pub player: Player,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.type_, self.player)
    }
}

const BOARD_SIZE: usize = 10;

pub type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

fn default_board() -> Board {
    fn fill_row(board: &mut Board, row: usize, player: Player) {
        for i in 0..BOARD_SIZE {
            if (i + row) % 2 == 0 {
                board[row][i] = Some(Piece {
                    type_: PieceType::Man,
                    player,
                })
            }
        }
    }

    let mut board: Board = Board::default();

    fill_row(&mut board, 0, Player::Black);
    fill_row(&mut board, 1, Player::Black);
    fill_row(&mut board, 2, Player::Black);

    fill_row(&mut board, BOARD_SIZE - 3, Player::White);
    fill_row(&mut board, BOARD_SIZE - 2, Player::White);
    fill_row(&mut board, BOARD_SIZE - 1, Player::White);

    board
}

#[derive(Debug, Serialize, Clone)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: default_board(),
            current_player: Player::White,
        }
    }
}

pub type Position = (usize, usize);
pub type MoveSequence = (Vec<Move>, Vec<Position>);

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

fn is_valid_pos(pos: Pos) -> bool {
    0 <= pos.x && pos.x < BOARD_SIZE as i32 && 0 <= pos.y && pos.y < BOARD_SIZE as i32
}

fn at(board: &Board, pos: Pos) -> Option<&Piece> {
    board[pos.x as usize][pos.y as usize].as_ref()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Pos {
    type Output = Pos;

    fn mul(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<i32> for Pos {
    type Output = Pos;

    fn div(self, rhs: i32) -> Self::Output {
        Pos {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

fn p(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

fn mov(from: Pos, to: Pos) -> Move {
    Move {
        from: (from.x as usize, from.y as usize),
        to: (to.x as usize, to.y as usize),
    }
}

impl GameState {
    pub fn to_csv_string(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|piece| {
                        piece
                            .as_ref()
                            .map_or_else(|| "".to_owned(), Piece::to_string)
                    })
                    .collect::<Vec<_>>()
                    .join(";")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn apply_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), Error> {
        self.board[to.0][to.1] = self.board[from.0][from.1].take();
        Ok(())
    }

    pub fn list_valid_moves(&self) -> Vec<MoveSequence> {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Intermediate {
            pos: Pos,
            captures: Vec<Pos>,
            moves: Vec<Move>,
        }

        fn update_intermediate(
            new_pos: Pos,
            captured_pos: Option<Pos>,
            mut i: Intermediate,
        ) -> Option<Intermediate> {
            if captured_pos.is_some_and(|c| i.captures.contains(&c)) {
                None
            } else {
                i.moves.push(mov(i.pos, new_pos));
                i.pos = new_pos;

                if let Some(captured_pos) = captured_pos {
                    i.captures.push(captured_pos);
                }

                Some(i)
            }
        }

        fn list_valid_moves_for_man(state: &GameState, i: Intermediate) -> Vec<Intermediate> {
            let is_valid_capture_move = |d: Pos| {
                let new_pos = i.pos + d;
                let captured_pos = i.pos + d / 2;

                is_valid_pos(new_pos)
                    && at(&state.board, new_pos).is_none()
                    && at(&state.board, captured_pos)
                        .is_some_and(|p| p.player != state.current_player)
            };

            if i.captures.is_empty() {
                let dv = match state.current_player {
                    Player::White => -1,
                    Player::Black => 1,
                };

                let d = vec![p(dv, 1), p(dv, -1)];

                d.into_iter()
                    .filter(|d| is_valid_pos(i.pos + *d))
                    .flat_map(|d| {
                        if at(&state.board, i.pos + d).is_none() {
                            vec![Intermediate {
                                pos: i.pos + d,
                                captures: vec![],
                                moves: vec![mov(i.pos, i.pos + d)],
                            }]
                        } else if is_valid_capture_move(d * 2) {
                            list_valid_moves_for_man(
                                state,
                                Intermediate {
                                    pos: i.pos + d * 2,
                                    captures: vec![i.pos + d],
                                    moves: vec![mov(i.pos, i.pos + d * 2)],
                                },
                            )
                        } else {
                            vec![]
                        }
                    })
                    .collect()
            } else {
                let d = vec![p(2, 2), p(2, -2), p(2, 2), p(2, -2)];

                d.into_iter()
                    .filter(|d| is_valid_capture_move(*d))
                    .filter_map(|d| {
                        let new_pos = i.pos + d;
                        let captured_pos = i.pos + d / 2;
                        update_intermediate(new_pos, Some(captured_pos), i.clone())
                    })
                    .flat_map(|i| list_valid_moves_for_man(state, i))
                    .collect()
            }
        }

        fn list_valid_moves_for_king(state: &GameState, i: Intermediate) -> Vec<Intermediate> {
            let d = vec![p(1, 1), p(1, -1), p(-1, 1), p(-1, -1)];
            let must_capture = !i.captures.is_empty();

            d.into_iter()
                .flat_map(|delta| {
                    let next = (1..BOARD_SIZE).fold(
                        (None, vec![], false),
                        |(capturable, moves, done), distance| {
                            let pos = i.pos + delta * distance as i32;

                            if done || !is_valid_pos(pos) {
                                return (capturable, moves, true);
                            }

                            match at(&state.board, pos) {
                                Some(p) if p.player == state.current_player => {
                                    (capturable, moves, true)
                                }
                                Some(_) => (Some(pos), moves, capturable.is_some()),
                                None => {
                                    if !must_capture || capturable.is_some() {
                                        let mut new_moves = moves.clone();
                                        new_moves.push((pos, capturable));
                                        (capturable, new_moves, done)
                                    } else {
                                        (capturable, moves, done)
                                    }
                                }
                            }
                        },
                    );

                    next.1
                })
                .flat_map(|(new_pos, captures)| {
                    if let Some(i) = update_intermediate(new_pos, captures, i.clone()) {
                        if captures.is_some() {
                            list_valid_moves_for_king(state, i)
                        } else {
                            vec![i]
                        }
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<_>>()
        }

        let mut available_moves = vec![];

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let pos = p(col as i32, row as i32);
                let piece = at(&self.board, pos);

                let moves = match piece {
                    Some(Piece {
                        type_: PieceType::Man,
                        player,
                    }) if *player == self.current_player => list_valid_moves_for_man(
                        self,
                        Intermediate {
                            pos,
                            captures: vec![],
                            moves: vec![],
                        },
                    ),
                    Some(Piece {
                        type_: PieceType::King,
                        player,
                    }) if *player == self.current_player => dbg!(list_valid_moves_for_king(
                        self,
                        Intermediate {
                            pos,
                            captures: vec![],
                            moves: vec![],
                        },
                    )),
                    _ => continue,
                };

                available_moves.append(
                    &mut moves
                        .into_iter()
                        .map(|i| {
                            (
                                i.moves,
                                i.captures
                                    .into_iter()
                                    .map(|c| (c.y as usize, c.x as usize))
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect(),
                );
            }
        }

        dbg!(&available_moves);

        let max = available_moves.iter().map(|m| m.1.len()).max();

        if let Some(max) = max {
            available_moves
                .into_iter()
                .filter(|m| m.1.len() == max)
                .collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Board, GameState, Move, MoveSequence, Piece, Position};
    use core::hash::Hash;
    use std::collections::HashSet;

    fn p(type_: char, player: char) -> Option<Piece> {
        Some(Piece {
            type_: match type_ {
                'M' => super::PieceType::Man,
                'K' => super::PieceType::King,
                _ => panic!(),
            },
            player: match player {
                'W' => super::Player::White,
                'B' => super::Player::Black,
                _ => panic!(),
            },
        })
    }

    fn m(x1: usize, y1: usize, x2: usize, y2: usize) -> Move {
        Move {
            from: (x1, y1),
            to: (x2, y2),
        }
    }

    fn list(board: Board) -> (Vec<MoveSequence>, Vec<MoveSequence>) {
        (
            GameState {
                board: board.clone(),
                current_player: crate::game::Player::White,
            }
            .list_valid_moves(),
            GameState {
                board,
                current_player: crate::game::Player::Black,
            }
            .list_valid_moves(),
        )
    }

    fn iters_equal_anyorder<T: Eq + Hash>(
        mut i1: impl Iterator<Item = T>,
        i2: impl Iterator<Item = T>,
    ) -> bool {
        let set: HashSet<T> = i2.collect();
        i1.all(|x| set.contains(&x))
    }

    #[test]
    fn empty() {
        let state = GameState {
            board: [
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None],
            ],
            current_player: super::Player::White,
        };
        assert!(state.list_valid_moves().is_empty());
    }

    #[test]
    fn trivial_man() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                None,
                p('M', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 2);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [(vec![m(5, 2, 4, 1)], vec![]), (vec![m(5, 2, 4, 3)], vec![])].iter()
        ));

        assert!(black_moves.is_empty());
    }

    #[test]
    fn trivial_king() {
        let board = [
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [
                None,
                None,
                p('K', 'W'),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None, None],
        ];

        let (white_moves, black_moves) = list(board);

        assert_eq!(white_moves.len(), 13);
        assert!(iters_equal_anyorder(
            white_moves.iter(),
            [
                (vec![m(5, 2, 3, 0)], vec![]),
                (vec![m(5, 2, 4, 1)], vec![]),
                (vec![m(5, 2, 6, 3)], vec![]),
                (vec![m(5, 2, 7, 4)], vec![]),
                (vec![m(5, 2, 8, 5)], vec![]),
                (vec![m(5, 2, 9, 6)], vec![]),
                (vec![m(5, 2, 7, 0)], vec![]),
                (vec![m(5, 2, 6, 1)], vec![]),
                (vec![m(5, 2, 4, 3)], vec![]),
                (vec![m(5, 2, 3, 4)], vec![]),
                (vec![m(5, 2, 2, 5)], vec![]),
                (vec![m(5, 2, 1, 6)], vec![]),
                (vec![m(5, 2, 0, 7)], vec![]),
            ]
            .iter()
        ));

        assert!(black_moves.is_empty());
    }
}
