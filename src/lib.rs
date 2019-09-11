//TODO: follow tha guidelines https://rust-lang-nursery.github.io/api-guidelines/
use std::fmt;

#[test]
fn test_gamestate() {
    assert_eq!(GameState::Draw, GameState::Draw);
    assert_eq!(GameState::Checkmate(Color::Black), GameState::Checkmate(Color::Black));
    assert_ne!(GameState::Checkmate(Color::Black), GameState::Checkmate(Color::White));
}

#[test]
fn test_piece() {
    let piece = Piece {
        position: Position {
            x: 3,
            y: 3,
        },
        color: Color::Black,
        piece_type: PieceType::Rook,
    };

    assert_eq!(piece.color, Color::Black);
    assert_eq!(piece.position, Position::new(3, 3));
}

#[test]
fn test_check() {
    let mut pieces: Vec<Piece> = Vec::new();

    pieces.push(Piece {
        position: Position {
            x: 4,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 8,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 4,
            y: 4,
        },
        color: Color::Black,
        piece_type: PieceType::Rook,
    });

    let board: Board = Board::new(pieces);

    for moves in board.get_available_moves(Color::White) {
        if moves.0.piece_type.is_king() {
            assert_eq!(moves.1.len(), 4);
        }
    }
}

#[test]
fn test_en_passant() {
    let mut pieces: Vec<Piece> = Vec::new();

    pieces.push(Piece {
        position: Position {
            x: 1,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 1,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 4,
        },
        color: Color::Black,
        piece_type: PieceType::Pawn,
    });
    pieces.push(Piece {
        position: Position {
            x: 6,
            y: 2,
        },
        color: Color::White,
        piece_type: PieceType::Pawn,
    });

    let mut board = Board::new(pieces);

    board.take_move("f2-f4".to_string());

    let available_moves = board.get_available_moves(Color::Black);
    let mut black_pawn_moves = Vec::new();
    for available_move in available_moves {
        if available_move.0.position == Position::new(5, 4) {
            black_pawn_moves = available_move.1;
        }
    }

    assert_eq!(black_pawn_moves.len(), 2);

    board.take_move("e4-f3".to_string());

    assert_eq!(board.play_history.len(), 2);
    assert_eq!(board.pieces.len(), 3);
}

#[test]
fn test_promotion() {
    let mut pieces: Vec<Piece> = Vec::new();

    pieces.push(Piece {
        position: Position {
            x: 1,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 1,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 2,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::Knight,
    });
    pieces.push(Piece {
        position: Position {
            x: 6,
            y: 2,
        },
        color: Color::Black,
        piece_type: PieceType::Pawn,
    });
    pieces.push(Piece {
        position: Position {
            x: 6,
            y: 7,
        },
        color: Color::White,
        piece_type: PieceType::Pawn,
    });
    pieces.push(Piece {
        position: Position {
            x: 7,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::Rook,
    });

    let mut board = Board::new(pieces);

    assert_eq!(board.pieces.len(), 6);

    board.take_move("f7-g8=Q".to_string());
    board.take_move("f2-f1=Q".to_string());

    assert_eq!(board.pieces.len(), 5);
    assert!(board.get_piece_at(Position::new(6, 1)).is_some());
}

#[test]
fn test_castling() {
    let mut pieces: Vec<Piece> = Vec::new();

    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 1,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::Rook,
    });
    pieces.push(Piece {
        position: Position {
            x: 8,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::Rook,
    });

    let mut board = Board::new(pieces);

    board.take_move("e1-g1".to_string());
    board.take_move("e8-c8".to_string());
    assert!(board.get_piece_at(Position::new(3, 8)).is_some());
    assert!(board.get_piece_at(Position::new(4, 8)).is_some());
    assert!(board.get_piece_at(Position::new(7, 1)).is_some());
    assert!(board.get_piece_at(Position::new(6, 1)).is_some());
}

#[test]
#[should_panic]
fn test_bad_castling() {
    let mut pieces: Vec<Piece> = Vec::new();

    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 7,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::Rook,
    });
    pieces.push(Piece {
        position: Position {
            x: 8,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::Rook,
    });

    let mut board = Board::new(pieces);

    board.move_piece_at(Position::new(5, 1), Position::new(7, 1), None);
}

#[test]
fn test_chess_notation() {
    let mut pieces: Vec<Piece> = Vec::new();

    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 1,
        },
        color: Color::White,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 5,
            y: 8,
        },
        color: Color::Black,
        piece_type: PieceType::King,
    });
    pieces.push(Piece {
        position: Position {
            x: 2,
            y: 2,
        },
        color: Color::White,
        piece_type: PieceType::Bishop,
    });

    let mut board = Board::new(pieces);

    board.take_move("b2-d4".to_string());

    assert!(board.get_piece_at(Position::new(4, 4)).is_some());
}

#[test]
fn test_game_states() {
    let mut pieces1: Vec<Piece> = Vec::new();

    pieces1.push(Piece { position: Position::new(5, 1), color: Color::White, piece_type: PieceType::King });
    pieces1.push(Piece { position: Position::new(5, 8), color: Color::Black, piece_type: PieceType::King });
    pieces1.push(Piece { position: Position::new(5, 7), color: Color::White, piece_type: PieceType::Queen });
    pieces1.push(Piece { position: Position::new(5, 6), color: Color::White, piece_type: PieceType::Rook });
    let mut board1 = Board::new(pieces1);
    board1.current_turn = Color::Black;

    assert_eq!(board1.get_game_state(), GameState::Checkmate(Color::White));

    let mut pieces2: Vec<Piece> = Vec::new();

    pieces2.push(Piece { position: Position::new(5, 1), color: Color::White, piece_type: PieceType::King });
    pieces2.push(Piece { position: Position::new(5, 8), color: Color::Black, piece_type: PieceType::King });
    pieces2.push(Piece { position: Position::new(6, 6), color: Color::White, piece_type: PieceType::Queen });
    pieces2.push(Piece { position: Position::new(6, 5), color: Color::White, piece_type: PieceType::Bishop });
    let mut board2 = Board::new(pieces2);
    board2.current_turn = Color::Black;

    assert_eq!(board2.get_game_state(), GameState::Draw);

    let mut pieces3: Vec<Piece> = Vec::new();

    pieces3.push(Piece { position: Position::new(5, 1), color: Color::White, piece_type: PieceType::King });
    pieces3.push(Piece { position: Position::new(5, 8), color: Color::Black, piece_type: PieceType::King });
    pieces3.push(Piece { position: Position::new(7, 1), color: Color::Black, piece_type: PieceType::Queen });
    let mut board3 = Board::new(pieces3);
    board3.current_turn = Color::White;

    assert_eq!(board3.get_game_state(), GameState::Check(Color::Black));
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::Pawn => { write!(f, "p") }
            PieceType::King => { write!(f, "k") }
            PieceType::Queen => { write!(f, "q") }
            PieceType::Rook => { write!(f, "r") }
            PieceType::Bishop => { write!(f, "b") }
            PieceType::Knight => { write!(f, "n") }
        }
    }
}

impl PieceType {
    fn is_king(&self) -> bool {
        match self {
            PieceType::King => { true }
            _ => { false }
        }
    }

    fn is_pawn(&self) -> bool {
        match self {
            PieceType::Pawn => { true }
            _ => { false }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => { write!(f, "White") }
            Color::Black => { write!(f, "Black") }
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match other {
            Color::White => {
                match self {
                    Color::White => { true }
                    Color::Black => { false }
                }
            }
            Color::Black => {
                match self {
                    Color::White => { false }
                    Color::Black => { true }
                }
            }
        }
    }
}

impl Color {
    pub fn invert(&self) -> Color {
        match self {
            Color::Black => { Color::White }
            Color::White => { Color::Black }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: u8,
    y: u8,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (self.x + 96) as char, self.y)
    }
}

impl Position {
    pub fn new(x: u8, y: u8) -> Position {
        Position {
            x,
            y,
        }
    }

    fn exist_in_vec(&self, list_to_check: &Vec<Position>, index: &mut usize) -> bool {
        for position in list_to_check {
            if self == position {
                *index = list_to_check.iter().enumerate().find(|r| self == r.1).unwrap().0;
                return true;
            }
        }
        return false;
    }

    fn on_chess_board(&self) -> bool {
        (1 <= self.y && self.y <= 8 && 1 <= self.x && self.x <= 8)
    }

    fn position_at_offset(&self, offset: &(i8, i8)) -> Option<Position> {
        let new_position = Position::new(((self.x as i8) + offset.0) as u8, (self.y as i8 + offset.1) as u8);
        if new_position.on_chess_board() {
            return Some(new_position);
        }
        return None;
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    moved_piece_type: PieceType,
    moved_piece_color: Color,
    start_position: Position,
    end_position: Position,
    captures_piece: bool,
    move_type: MoveType,
}

impl Move {
    fn new(moved_piece_type: PieceType, moved_piece_color: Color, start_position: Position, end_position: Position, captures_piece: bool, move_type: MoveType) -> Move {
        Move {
            moved_piece_type,
            moved_piece_color,
            start_position,
            end_position,
            captures_piece,
            move_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum MoveType {
    Normal,
    Passant(Position),
    Castling(bool),
    Promotion(),
}

impl MoveType {
    fn same_as(&self, move_type: &MoveType) -> bool {
        match &self {
            MoveType::Normal => {
                match move_type {
                    MoveType::Normal => { true }
                    MoveType::Passant(_) => { false }
                    MoveType::Castling(_) => { false }
                    MoveType::Promotion() => { false }
                }
            }
            MoveType::Passant(_) => {
                match move_type {
                    MoveType::Normal => { false }
                    MoveType::Passant(_) => { true }
                    MoveType::Castling(_) => { false }
                    MoveType::Promotion() => { false }
                }
            }
            MoveType::Castling(_) => {
                match move_type {
                    MoveType::Normal => { false }
                    MoveType::Passant(_) => { false }
                    MoveType::Castling(_) => { true }
                    MoveType::Promotion() => { false }
                }
            }
            MoveType::Promotion() => {
                match move_type {
                    MoveType::Normal => { false }
                    MoveType::Passant(_) => { false }
                    MoveType::Castling(_) => { false }
                    MoveType::Promotion() => { true }
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    position: Position,
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    fn get_possible_moves(&self, board: &Board, moving: bool) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        match &self.piece_type {
            PieceType::King => {
                //TODO: this could use a refactor
                let mut possible_directions: Vec<((i8, i8), MoveType)> = Vec::new();

                //Check if king has moved
                if board.play_history.iter().find(|mov|
                    mov.start_position == Position::new(5, if self.color == Color::White { 1 } else { 8 })).is_none()
                    && board.get_piece_at(Position::new(5, if self.color == Color::White { 1 } else { 8 })).is_some() {
                    //Check if rook for kingside castling has moved
                    if board.play_history.iter().find(|mov|
                        mov.start_position == Position::new(8, if self.color == Color::White { 1 } else { 8 })).is_none()
                        && board.get_piece_at(Position::new(8, if self.color == Color::White { 1 } else { 8 })).is_some() {
                        //Check if any pieces are in the way
                        if board.get_piece_at(Position::new(6, 1)).is_none() && board.get_piece_at(Position::new(7, 1)).is_none() {
                            possible_directions.push(((2i8, 0), MoveType::Castling(false)));
                        }
                    }

                    //Check if rook for queenside castling has moved
                    if board.play_history.iter().find(|mov|
                        mov.start_position == Position::new(1, if self.color == Color::White { 1 } else { 8 })).is_none()
                        && board.get_piece_at(Position::new(1, if self.color == Color::White { 1 } else { 8 })).is_some() {
                        //Check if any pieces are in the way
                        if board.get_piece_at(Position::new(4, 1)).is_none()
                            && board.get_piece_at(Position::new(3, 1)).is_none()
                            && board.get_piece_at(Position::new(2, 1)).is_none() {
                            possible_directions.push(((-2i8, 0), MoveType::Castling(true)));
                        }
                    }
                }

                possible_moves = self.offset_and_check_availability(
                    &[(1i8, 1i8), (0i8, 1i8), (-1i8, 1i8), (-1i8, 0i8), (-1i8, -1i8), (0i8, -1i8), (1i8, -1i8), (1i8, 0i8)],
                    1i8, board, moving, MoveType::Normal);


                for possible_direction in possible_directions {
                    if let Some(val) = self.offset_and_check_availability(
                        &[possible_direction.0], 1i8, board, moving, possible_direction.1)
                        .get(0) {
                        possible_moves.push(*val);
                    }
                }
            }
            PieceType::Bishop => {
                possible_moves = self.offset_and_check_availability(
                    &[(1i8, 1i8), (-1i8, 1i8), (1i8, -1i8), (-1i8, -1i8)],
                    8i8, board, moving, MoveType::Normal);
            }
            PieceType::Knight => {
                possible_moves = self.offset_and_check_availability(
                    &[(2i8, 1i8), (2i8, -1i8), (-2i8, 1i8), (-2i8, -1i8), (1i8, 2i8), (-1i8, 2i8), (1i8, -2i8), (-1i8, -2i8)],
                    1i8, board, moving, MoveType::Normal);
            }
            PieceType::Pawn => {
                //TODO: refactor this hot mess
                let mut possible_directions: Vec<((i8, i8), MoveType)> = Vec::new();
                let direction_from_color;
                let mut promotes = false;
                match &self.color {
                    Color::White => {
                        direction_from_color = 1i8;
                        if self.position.y == 7 {
                            promotes = true;
                        }
                    }
                    Color::Black => {
                        direction_from_color = -1i8;
                        if self.position.y == 2 {
                            promotes = true;
                        }
                    }
                }

                let move_type: MoveType = if promotes { MoveType::Promotion() } else { MoveType::Normal };

                possible_directions.push(((0i8, direction_from_color), move_type));
                match &self.position.position_at_offset(&(0, direction_from_color * 2)) {
                    Some(_) => {
                        if (self.position.y == 2 && self.color == Color::White) || (self.position.y == 7 && self.color == Color::Black) {
                            possible_directions.push(((0, direction_from_color * 2), move_type));
                        }
                    }
                    None => {}
                }

                for side_direction in &[-1i8, 1i8] {
                    match &self.position.position_at_offset(&(*side_direction, direction_from_color)) {
                        Some(new_position) => {
                            let mut new_position_occupied: bool = false;

                            //Checks for a piece in the diagonal direction to capture
                            for piece in &board.pieces {
                                if piece.position == *new_position {
                                    if piece.color == self.color {} else {
                                        possible_directions.push(((*side_direction, direction_from_color), move_type));
                                        new_position_occupied = true;
                                    }
                                }
                            }

                            //Checks if the piece can capture a pawn en passant
                            if new_position_occupied {} else {
                                match &self.position.position_at_offset(&(*side_direction, 0i8)) {
                                    Some(side_position) => {
                                        for piece in &board.pieces {
                                            if piece.position == side_position.clone() {
                                                if piece.color == self.color {} else {
                                                    if board.play_history.len() == 0 {
                                                        continue;
                                                    }
                                                    if board.play_history.last().unwrap().end_position == side_position.clone()
                                                        && board.play_history.last().unwrap().start_position == side_position.position_at_offset(
                                                        &(0i8, direction_from_color * 2)).unwrap()
                                                        && board.play_history.last().unwrap().moved_piece_type.is_pawn() {
                                                        possible_directions.push(((*side_direction, direction_from_color), MoveType::Passant(side_position.clone())));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                        }
                        None => {}
                    }
                }

                for possible_direction in possible_directions {
                    if let Some(val) = self.offset_and_check_availability(
                        &[possible_direction.0], 1i8, board, moving, possible_direction.1)
                        .get(0) {
                        possible_moves.push(*val);
                    }
                }
            }
            PieceType::Queen => {
                possible_moves = self.offset_and_check_availability(
                    &[(1i8, 0i8), (-1i8, 0i8), (0i8, 1i8), (0i8, -1i8), (1i8, 1i8), (-1i8, 1i8), (1i8, -1i8), (-1i8, -1i8)],
                    8i8, board, moving, MoveType::Normal);
            }
            PieceType::Rook => {
                possible_moves = self.offset_and_check_availability(
                    &[(1i8, 0i8), (-1i8, 0i8), (0i8, 1i8), (0i8, -1i8)],
                    8i8, board, moving, MoveType::Normal);
            }
        }
        return possible_moves;
    }

    //TODO: this also has to be refactored
    fn offset_and_check_availability(&self, offset_direction: &[(i8, i8)], range: i8, board: &Board, moving: bool, move_type: MoveType) -> Vec<Move> {
        let mut possible_moves = Vec::new();

        for dir in offset_direction.iter() {
            for distance in 1..(range + 1) {
                match self.position.position_at_offset(&(dir.0 * distance, dir.1 * distance)) {
                    Some(position_moving_to) => {
                        let mut king_gets_threatened = false;
                        if moving {
                            let mut king_position: Position = Position::new(1, 1);
                            for piece in &board.pieces {
                                if piece.color == self.color && piece.piece_type.is_king() {
                                    king_position = piece.position.clone();
                                }
                            }

                            let mut p: Piece = self.clone();
                            p.set_position(position_moving_to.clone());

                            let mut new_board_state: Vec<&Piece> = Vec::new();
                            for piece_on_board in &board.pieces {
                                if piece_on_board.position == self.position.clone() {
                                    new_board_state.push(&p);
                                } else if piece_on_board.position == position_moving_to {} else {
                                    new_board_state.push(&piece_on_board);
                                }
                            }

                            if self.piece_type.is_king() {
                                king_position = position_moving_to.clone();
                            }

                            for piece in &new_board_state {
                                if piece.color == self.color {} else {
                                    let board2 = Board {
                                        pieces: new_board_state.iter().map(|piece| *piece.clone()).collect(),
                                        current_turn: board.current_turn.clone(),
                                        play_history: board.play_history.iter().map(|mov| mov.clone()).collect(),
                                    };
                                    if piece.can_check(king_position.clone(), &board2) {
                                        king_gets_threatened = true;
                                    }
                                }
                            }

                            if king_gets_threatened {
                                continue;
                            }
                        }

                        let mut index: usize = 0;
                        if (&position_moving_to).exist_in_vec(&board.get_occupied_squares(), &mut index) {
                            if board.pieces.get(index).unwrap().color == self.color {} else {
                                &possible_moves.push(
                                    Move::new(self.piece_type.clone(), self.color.clone(),
                                              self.position.clone(), position_moving_to.clone(),
                                              true, move_type));
                            }
                            break;
                        }
                        &possible_moves.push(Move::new(self.piece_type.clone(), self.color.clone(),
                                                       self.position.clone(), position_moving_to.clone(),
                                                       false || move_type.same_as(&MoveType::Passant(Position::new(1, 1))), move_type));
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        return possible_moves;
    }

    fn can_check(&self, position: Position, board: &Board) -> bool {
        let available_moves = self.get_possible_moves(board, false);
        for available_move in &available_moves {
            if available_move.end_position == position {
                return true;
            }
        }
        return false;
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_position(&self) -> Position { self.position.clone() }

    pub fn get_type(&self) -> PieceType { self.piece_type.clone() }

    pub fn get_color(&self) -> Color { self.color.clone() }
}

#[derive(Debug)]
pub struct Board {
    pieces: Vec<Piece>,
    current_turn: Color,
    play_history: Vec<Move>,
}

impl Board {
    pub fn new(pieces: Vec<Piece>) -> Board {
        Board {
            pieces,
            current_turn: Color::White,
            play_history: Vec::new(),
        }
    }

    fn get_available_moves(&self, player_color: Color) -> Vec<(&Piece, Vec<Move>)> {
        let mut out: Vec<(&Piece, Vec<Move>)> = Vec::new();
        for piece in &self.pieces {
            if piece.color == player_color {
                let possible_moves: Vec<Move> = piece.get_possible_moves(&self, true);
                &out.push((&piece, possible_moves));
            }
        }
        return out;
    }

    pub fn get_current_player_moves(&self) -> Vec<(&Piece, Vec<Position>)> {
        return self.get_available_moves(self.current_turn).iter()
            .map(|t| (t.0, t.1.iter().map(|mov| mov.end_position.clone()).collect())).collect();
    }

    pub fn get_standard_layout() -> Vec<Piece> {
        const STANDARD_BOARD: [(Color, char, u8, u8); 32] = [(Color::Black, 'r', 1, 8), (Color::Black, 'n', 2, 8), (Color::Black, 'b', 3, 8), (Color::Black, 'q', 4, 8),
            (Color::Black, 'k', 5, 8), (Color::Black, 'b', 6, 8), (Color::Black, 'n', 7, 8), (Color::Black, 'r', 8, 8),
            (Color::Black, 'p', 1, 7), (Color::Black, 'p', 2, 7), (Color::Black, 'p', 3, 7), (Color::Black, 'p', 4, 7),
            (Color::Black, 'p', 5, 7), (Color::Black, 'p', 6, 7), (Color::Black, 'p', 7, 7), (Color::Black, 'p', 8, 7),
            (Color::White, 'r', 1, 1), (Color::White, 'n', 2, 1), (Color::White, 'b', 3, 1), (Color::White, 'q', 4, 1),
            (Color::White, 'k', 5, 1), (Color::White, 'b', 6, 1), (Color::White, 'n', 7, 1), (Color::White, 'r', 8, 1),
            (Color::White, 'p', 1, 2), (Color::White, 'p', 2, 2), (Color::White, 'p', 3, 2), (Color::White, 'p', 4, 2),
            (Color::White, 'p', 5, 2), (Color::White, 'p', 6, 2), (Color::White, 'p', 7, 2), (Color::White, 'p', 8, 2)];
        let mut standard_layout: Vec<Piece> = Vec::new();
        for piece_prototype in STANDARD_BOARD.iter() {
            let piece_type: PieceType;

            match piece_prototype.1 {
                'r' => { piece_type = PieceType::Rook }
                'n' => { piece_type = PieceType::Knight }
                'b' => { piece_type = PieceType::Bishop }
                'q' => { piece_type = PieceType::Queen }
                'k' => { piece_type = PieceType::King }
                'p' => { piece_type = PieceType::Pawn }
                _ => { panic!("Char {} does not match a piece type", piece_prototype.1) }
            }

            &standard_layout.push(Piece {
                position: Position {
                    x: piece_prototype.2,
                    y: piece_prototype.3,
                },
                color: piece_prototype.0,
                piece_type: piece_type.clone(),
            });
        }
        return standard_layout;
    }

    fn get_piece_at(&self, position: Position) -> Option<&Piece> {
        for piece in &self.pieces {
            if piece.position == position {
                return Some(&piece);
            }
        }
        return None;
    }

    fn get_piece_at_mut(&mut self, position: Position) -> Option<&mut Piece> {
        for piece in &mut self.pieces {
            if piece.position == position {
                return Some(piece);
            }
        }
        return None;
    }

    fn move_piece_at(&mut self, piece_position: Position, target_position: Position, promote_to: Option<PieceType>) {
        let piece_color = self.get_piece_at(piece_position).unwrap().color.clone();

        if !(piece_color == self.current_turn) {
            panic!(format!("Tried to move a {} piece even though it's {}'s turn", piece_color, self.current_turn));
        }

        let mut available_moves_clone: Vec<(Piece, Vec<Move>)> = Vec::new();
        for available_move in self.get_available_moves(piece_color.clone()) {
            let mut move_vec_clone: Vec<Move> = Vec::new();
            for mov in available_move.1 {
                move_vec_clone.push(mov.clone());
            }
            available_moves_clone.push((available_move.0.clone(), move_vec_clone));
        }

        let mut found_piece = false;
        let mut target_available = false;
        for piece_move in available_moves_clone {
            if piece_move.0.position == piece_position {
                found_piece = true;
                for possible_move in &piece_move.1 {
                    if possible_move.end_position == target_position {
                        target_available = true;
                        match possible_move.move_type {
                            MoveType::Normal => {
                                if possible_move.captures_piece {
                                    self.remove_piece_at_position(possible_move.end_position);
                                }
                                self.get_piece_at_mut(piece_move.0.position.clone()).unwrap().set_position(possible_move.end_position.clone());
                            }
                            MoveType::Passant(position) => {
                                self.remove_piece_at_position(position);
                                self.get_piece_at_mut(piece_move.0.position.clone()).unwrap().set_position(possible_move.end_position.clone());
                            }
                            MoveType::Castling(queenside) => {
                                self.get_piece_at_mut(piece_move.0.position.clone()).unwrap().set_position(possible_move.end_position.clone());
                                self.get_piece_at_mut(Position::new(if queenside { 1 } else { 8 }, piece_move.0.position.y)).unwrap()
                                    .set_position(Position::new(if queenside { 4 } else { 6 }, piece_move.0.position.y));
                            }
                            MoveType::Promotion() => {
                                if possible_move.captures_piece {
                                    self.remove_piece_at_position(possible_move.end_position);
                                }
                                if promote_to.is_none() {
                                    panic!("Piece is trying to promote but cant because it didn't receive a type to promote to")
                                }
                                if promote_to.unwrap().is_king() {
                                    panic!("Can't promote to king")
                                }
                                self.get_piece_at_mut(piece_move.0.position.clone()).unwrap().piece_type = promote_to.unwrap().clone();
                                self.get_piece_at_mut(piece_move.0.position.clone()).unwrap().set_position(possible_move.end_position.clone());
                            }
                        }
                        self.play_history.push(possible_move.clone());
                    }
                }
                if !target_available {
                    panic!(format!("Can't move piece at {} to {}", piece_move.0.position, target_position));
                }
            }
        }

        if !found_piece {
            panic!(format!("Didn't find a piece at {}", piece_position));
        }
    }

    fn remove_piece_at_position(&mut self, position: Position) {
        self.pieces = self.pieces.iter().filter(|piece| !(piece.position == position)).map(|piece| *piece).collect();
    }

    fn get_occupied_squares(&self) -> Vec<Position> {
        let mut out: Vec<Position> = Vec::new();
        for piece in &self.pieces {
            out.push(piece.position.clone());
        }
        out
    }

    pub fn take_move(&mut self, notated_move: String) {
        let strings: Vec<char> = notated_move.chars().filter(|c| *c != '=' && *c != '-').collect();

        self.move_piece_at(Position::new((*strings.get(0).unwrap() as u32 - 96) as u8, (*strings.get(1).unwrap() as u32 - 48) as u8),
                           Position::new((*strings.get(2).unwrap() as u32 - 96) as u8, (*strings.get(3).unwrap() as u32 - 48) as u8),
                           match strings.get(4) {
                               Some(letter) => {
                                   match *letter {
                                       'P' => { Some(PieceType::Pawn) }
                                       'N' => { Some(PieceType::Knight) }
                                       'R' => { Some(PieceType::Rook) }
                                       'B' => { Some(PieceType::Bishop) }
                                       'Q' => { Some(PieceType::Queen) }
                                       'K' => { Some(PieceType::King) }
                                       _ => { None }
                                   }
                               }
                               None => { None }
                           });

        match self.current_turn {
            Color::Black => { self.current_turn = Color::White; }
            Color::White => { self.current_turn = Color::Black; }
        }
    }

    pub fn get_game_state(&self) -> GameState {
        let available_moves_opponent = self.get_available_moves(self.current_turn.invert().clone());
        let available_moves = self.get_available_moves(self.current_turn.clone());
        let mut king_threatened = false;
        let mut king_piece: &Piece = &Piece {
            position: Position::new(1, 1),
            piece_type: PieceType::King,
            color: self.current_turn.clone(),
        };

        for piece in &self.pieces {
            if piece.color == self.current_turn && piece.piece_type.is_king() {
                king_piece = piece;
            }
        }

        for piece_move in available_moves_opponent {
            for mov in piece_move.1 {
                if mov.end_position == king_piece.position {
                    king_threatened = true;
                }
            }
        }

        if self.play_history.len() >= 100 {
            return GameState::Draw;
            //This is incorrect, because the game is supposed to end after 50 full moves of no pawn moves or piece captures
            //but games rarely go above 100 moves, making this a good approximation
        }

        if king_threatened {
            if available_moves.iter().map(|mov| mov.1.len()).fold(0, |acc, val| acc + val) == 0 {
                return GameState::Checkmate(self.current_turn.invert());
            } else {
                return GameState::Check(self.current_turn.invert());
            }
        } else {
            if available_moves.iter().map(|mov| mov.1.len()).fold(0, |acc, val| acc + val) == 0 {
                return GameState::Draw;
            } else {
                return GameState::Normal;
            }
        }
    }

    pub fn get_pieces(&self) -> Vec<&Piece> {
        self.pieces.iter().map(|piece| piece).collect()
    }
}

#[derive(Debug)]
pub enum GameState {
    Normal,
    Check(Color),
    Checkmate(Color),
    Draw,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        match other {
            GameState::Draw => {
                match self {
                    GameState::Draw => { true }
                    GameState::Normal => { false }
                    GameState::Check(_) => { false }
                    GameState::Checkmate(_) => { false }
                }
            }
            GameState::Normal => {
                match self {
                    GameState::Draw => { false }
                    GameState::Normal => { true }
                    GameState::Check(_) => { false }
                    GameState::Checkmate(_) => { false }
                }
            }
            GameState::Check(a) => {
                match self {
                    GameState::Draw => { false }
                    GameState::Normal => { false }
                    GameState::Check(b) => { a == b }
                    GameState::Checkmate(_) => { false }
                }
            }
            GameState::Checkmate(a) => {
                match self {
                    GameState::Draw => { false }
                    GameState::Normal => { false }
                    GameState::Check(_) => { false }
                    GameState::Checkmate(b) => { a == b }
                }
            }
        }
    }
}