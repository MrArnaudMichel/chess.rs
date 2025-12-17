//! Bitboard-based board representation.
//!
//! This module provides a high-performance board representation using bitboards.
//! Each piece type and color combination is stored in a separate 64-bit integer,
//! allowing for fast bit operations to compute moves, attacks, and other board state.

use super::bitboard::BitBoard;
use super::piece_type::{Color, ColoredPiece, PieceType};
use super::attacks::{
    bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
};
use crate::core::types::position::Position;

/// Castling rights for both sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRights {
    /// Full castling rights (initial game state).
    pub const ALL: CastlingRights = CastlingRights {
        white_kingside: true,
        white_queenside: true,
        black_kingside: true,
        black_queenside: true,
    };

    /// No castling rights.
    pub const NONE: CastlingRights = CastlingRights {
        white_kingside: false,
        white_queenside: false,
        black_kingside: false,
        black_queenside: false,
    };

    /// Returns true if the given color can castle kingside.
    #[inline]
    pub fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_kingside,
            Color::Black => self.black_kingside,
        }
    }

    /// Returns true if the given color can castle queenside.
    #[inline]
    pub fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_queenside,
            Color::Black => self.black_queenside,
        }
    }

    /// Removes kingside castling right for the given color.
    #[inline]
    pub fn remove_kingside(&mut self, color: Color) {
        match color {
            Color::White => self.white_kingside = false,
            Color::Black => self.black_kingside = false,
        }
    }

    /// Removes queenside castling right for the given color.
    #[inline]
    pub fn remove_queenside(&mut self, color: Color) {
        match color {
            Color::White => self.white_queenside = false,
            Color::Black => self.black_queenside = false,
        }
    }

    /// Removes all castling rights for the given color.
    #[inline]
    pub fn remove_all(&mut self, color: Color) {
        self.remove_kingside(color);
        self.remove_queenside(color);
    }

    /// Returns the FEN representation of castling rights.
    pub fn to_fen(&self) -> String {
        let mut result = String::new();
        if self.white_kingside {
            result.push('K');
        }
        if self.white_queenside {
            result.push('Q');
        }
        if self.black_kingside {
            result.push('k');
        }
        if self.black_queenside {
            result.push('q');
        }
        if result.is_empty() {
            result.push('-');
        }
        result
    }

    /// Parses castling rights from FEN string.
    pub fn from_fen(s: &str) -> Result<Self, &'static str> {
        if s == "-" {
            return Ok(CastlingRights::NONE);
        }

        let mut rights = CastlingRights::NONE;
        for c in s.chars() {
            match c {
                'K' => rights.white_kingside = true,
                'Q' => rights.white_queenside = true,
                'k' => rights.black_kingside = true,
                'q' => rights.black_queenside = true,
                _ => return Err("Invalid castling rights character"),
            }
        }
        Ok(rights)
    }
}

/// Bitboard-based board representation.
///
/// This structure stores the position of all pieces using 12 bitboards
/// (6 piece types × 2 colors), plus aggregate bitboards for efficient queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitBoardState {
    /// Bitboards for white pieces (indexed by PieceType).
    pub white: [BitBoard; PieceType::COUNT],
    /// Bitboards for black pieces (indexed by PieceType).
    pub black: [BitBoard; PieceType::COUNT],

    /// All white pieces combined.
    pub white_pieces: BitBoard,
    /// All black pieces combined.
    pub black_pieces: BitBoard,
    /// All pieces combined.
    pub all_pieces: BitBoard,

    /// Side to move.
    pub side_to_move: Color,
    /// Castling rights.
    pub castling: CastlingRights,
    /// En passant target square (if any).
    pub en_passant: Option<u8>,
    /// Halfmove clock (for 50-move rule).
    pub halfmove_clock: u32,
    /// Fullmove number.
    pub fullmove_number: u32,
}

impl Default for BitBoardState {
    fn default() -> Self {
        Self::new()
    }
}

impl BitBoardState {
    /// Creates an empty board.
    pub fn new() -> Self {
        Self {
            white: [BitBoard::EMPTY; PieceType::COUNT],
            black: [BitBoard::EMPTY; PieceType::COUNT],
            white_pieces: BitBoard::EMPTY,
            black_pieces: BitBoard::EMPTY,
            all_pieces: BitBoard::EMPTY,
            side_to_move: Color::White,
            castling: CastlingRights::NONE,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Creates a board with the standard initial position.
    pub fn startpos() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Invalid starting position FEN")
    }

    /// Returns the bitboard for a specific piece type and color.
    #[inline]
    pub fn pieces(&self, piece_type: PieceType, color: Color) -> BitBoard {
        match color {
            Color::White => self.white[piece_type.index()],
            Color::Black => self.black[piece_type.index()],
        }
    }

    /// Returns the bitboard for all pieces of a given color.
    #[inline]
    pub fn pieces_by_color(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        }
    }

    /// Returns the piece at a given square, if any.
    pub fn piece_at(&self, square: u8) -> Option<ColoredPiece> {
        let bb = BitBoard::from_square(square);

        // Check if square is occupied
        if (self.all_pieces & bb).is_empty() {
            return None;
        }

        // Determine color
        let color = if (self.white_pieces & bb).is_not_empty() {
            Color::White
        } else {
            Color::Black
        };

        // Find piece type
        let pieces = match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        };

        for (idx, piece_bb) in pieces.iter().enumerate() {
            if (*piece_bb & bb).is_not_empty() {
                return Some(ColoredPiece::new(
                    PieceType::from_index(idx).unwrap(),
                    color,
                ));
            }
        }

        None
    }

    /// Returns the piece at a given position, if any.
    pub fn piece_at_position(&self, pos: &Position) -> Option<ColoredPiece> {
        let square = BitBoard::position_to_square(pos);
        self.piece_at(square)
    }

    /// Places a piece on the board.
    pub fn place_piece(&mut self, square: u8, piece: ColoredPiece) {
        let bb = BitBoard::from_square(square);

        match piece.color {
            Color::White => {
                self.white[piece.piece_type.index()] |= bb;
                self.white_pieces |= bb;
            }
            Color::Black => {
                self.black[piece.piece_type.index()] |= bb;
                self.black_pieces |= bb;
            }
        }
        self.all_pieces |= bb;
    }

    /// Removes a piece from the board.
    pub fn remove_piece(&mut self, square: u8) -> Option<ColoredPiece> {
        let piece = self.piece_at(square)?;
        let bb = BitBoard::from_square(square);

        match piece.color {
            Color::White => {
                self.white[piece.piece_type.index()] &= !bb;
                self.white_pieces &= !bb;
            }
            Color::Black => {
                self.black[piece.piece_type.index()] &= !bb;
                self.black_pieces &= !bb;
            }
        }
        self.all_pieces &= !bb;

        Some(piece)
    }

    /// Moves a piece from one square to another.
    /// Returns the captured piece, if any.
    pub fn move_piece(&mut self, from: u8, to: u8) -> Option<ColoredPiece> {
        let piece = self.remove_piece(from)?;
        let captured = self.remove_piece(to);
        self.place_piece(to, piece);
        captured
    }

    /// Updates the aggregate bitboards from the individual piece bitboards.
    pub fn update_aggregates(&mut self) {
        self.white_pieces = BitBoard::EMPTY;
        self.black_pieces = BitBoard::EMPTY;

        for bb in &self.white {
            self.white_pieces |= *bb;
        }
        for bb in &self.black {
            self.black_pieces |= *bb;
        }

        self.all_pieces = self.white_pieces | self.black_pieces;
    }

    /// Returns the square of the king for the given color.
    pub fn king_square(&self, color: Color) -> Option<u8> {
        self.pieces(PieceType::King, color).lsb()
    }

    /// Returns all squares attacked by a given color.
    pub fn attacks_by(&self, color: Color) -> BitBoard {
        let mut attacks = BitBoard::EMPTY;

        let pieces = match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        };

        // Pawn attacks
        let pawns = pieces[PieceType::Pawn.index()];
        for sq in pawns.iter() {
            attacks |= pawn_attacks(sq, color);
        }

        // Knight attacks
        let knights = pieces[PieceType::Knight.index()];
        for sq in knights.iter() {
            attacks |= knight_attacks(sq);
        }

        // Bishop attacks
        let bishops = pieces[PieceType::Bishop.index()];
        for sq in bishops.iter() {
            attacks |= bishop_attacks(sq, self.all_pieces);
        }

        // Rook attacks
        let rooks = pieces[PieceType::Rook.index()];
        for sq in rooks.iter() {
            attacks |= rook_attacks(sq, self.all_pieces);
        }

        // Queen attacks
        let queens = pieces[PieceType::Queen.index()];
        for sq in queens.iter() {
            attacks |= queen_attacks(sq, self.all_pieces);
        }

        // King attacks
        let kings = pieces[PieceType::King.index()];
        for sq in kings.iter() {
            attacks |= king_attacks(sq);
        }

        attacks
    }

    /// Returns true if the given color's king is in check.
    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_sq) = self.king_square(color) {
            let enemy_attacks = self.attacks_by(color.opposite());
            (enemy_attacks & BitBoard::from_square(king_sq)).is_not_empty()
        } else {
            false
        }
    }

    /// Returns true if a square is attacked by a given color.
    pub fn is_attacked_by(&self, square: u8, color: Color) -> bool {
        let attacks = self.attacks_by(color);
        attacks.is_set(square)
    }

    /// Parses a board from FEN notation.
    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("Invalid FEN: not enough parts");
        }

        let mut board = BitBoardState::new();

        // Parse piece placement
        let mut rank = 7i8;
        let mut file = 0i8;

        for c in parts[0].chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
                if rank < 0 {
                    return Err("Invalid FEN: too many ranks");
                }
            } else if c.is_ascii_digit() {
                let skip = c.to_digit(10).unwrap() as i8;
                file += skip;
            } else {
                if file >= 8 {
                    return Err("Invalid FEN: too many files");
                }

                let piece =
                    ColoredPiece::from_char(c).ok_or("Invalid FEN: invalid piece character")?;
                let square = (rank * 8 + file) as u8;
                board.place_piece(square, piece);
                file += 1;
            }
        }

        // Parse side to move
        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid FEN: invalid side to move"),
        };

        // Parse castling rights
        board.castling = CastlingRights::from_fen(parts[2])?;

        // Parse en passant square
        if parts[3] != "-" {
            let pos =
                Position::from_algebraic(parts[3]).ok_or("Invalid FEN: invalid en passant square")?;
            board.en_passant = Some(BitBoard::position_to_square(&pos));
        }

        // Parse halfmove clock (optional)
        if parts.len() > 4 {
            board.halfmove_clock = parts[4]
                .parse()
                .map_err(|_| "Invalid FEN: invalid halfmove clock")?;
        }

        // Parse fullmove number (optional)
        if parts.len() > 5 {
            board.fullmove_number = parts[5]
                .parse()
                .map_err(|_| "Invalid FEN: invalid fullmove number")?;
        }

        Ok(board)
    }

    /// Generates FEN notation for the current board state.
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Piece placement
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let square = rank * 8 + file;
                if let Some(piece) = self.piece_at(square) {
                    if empty_count > 0 {
                        fen.push(char::from_digit(empty_count, 10).unwrap());
                        empty_count = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push(char::from_digit(empty_count, 10).unwrap());
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        // Side to move
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling rights
        fen.push(' ');
        fen.push_str(&self.castling.to_fen());

        // En passant square
        fen.push(' ');
        if let Some(ep_sq) = self.en_passant {
            let pos = BitBoard::square_to_position(ep_sq);
            let file = (b'a' + pos.x as u8) as char;
            let rank = (b'1' + pos.y as u8) as char;
            fen.push(file);
            fen.push(rank);
        } else {
            fen.push('-');
        }

        // Halfmove clock
        fen.push(' ');
        fen.push_str(&self.halfmove_clock.to_string());

        // Fullmove number
        fen.push(' ');
        fen.push_str(&self.fullmove_number.to_string());

        fen
    }

    /// Calculates the material balance (positive for white advantage).
    pub fn material_balance(&self) -> i32 {
        let mut balance = 0;

        for piece_type in PieceType::ALL.iter() {
            let white_count = self.pieces(*piece_type, Color::White).count() as i32;
            let black_count = self.pieces(*piece_type, Color::Black).count() as i32;
            balance += (white_count - black_count) * piece_type.value();
        }

        balance
    }
}

impl std::fmt::Display for BitBoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = rank * 8 + file;
                match self.piece_at(square) {
                    Some(piece) => write!(f, "{} ", piece.to_char())?,
                    None => write!(f, ". ")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f)?;
        writeln!(f, "FEN: {}", self.to_fen())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::position::{A1, E1, E4};

    #[test]
    fn test_empty_board() {
        let board = BitBoardState::new();
        assert!(board.all_pieces.is_empty());
        assert!(board.piece_at(0).is_none());
    }

    #[test]
    fn test_startpos() {
        let board = BitBoardState::startpos();

        // Check white pawns on rank 2
        assert_eq!(board.pieces(PieceType::Pawn, Color::White).count(), 8);

        // Check black pawns on rank 7
        assert_eq!(board.pieces(PieceType::Pawn, Color::Black).count(), 8);

        // Check total pieces
        assert_eq!(board.all_pieces.count(), 32);

        // Check specific pieces
        assert_eq!(
            board.piece_at(0),
            Some(ColoredPiece::new(PieceType::Rook, Color::White))
        ); // a1
        assert_eq!(
            board.piece_at(4),
            Some(ColoredPiece::new(PieceType::King, Color::White))
        ); // e1
        assert_eq!(
            board.piece_at(60),
            Some(ColoredPiece::new(PieceType::King, Color::Black))
        ); // e8

        // Check castling rights
        assert!(board.castling.white_kingside);
        assert!(board.castling.white_queenside);
        assert!(board.castling.black_kingside);
        assert!(board.castling.black_queenside);
    }

    #[test]
    fn test_place_and_remove_piece() {
        let mut board = BitBoardState::new();

        // Place a white knight on e4
        let knight = ColoredPiece::new(PieceType::Knight, Color::White);
        board.place_piece(28, knight);

        assert_eq!(board.piece_at(28), Some(knight));
        assert_eq!(board.all_pieces.count(), 1);

        // Remove the knight
        let removed = board.remove_piece(28);
        assert_eq!(removed, Some(knight));
        assert!(board.piece_at(28).is_none());
        assert!(board.all_pieces.is_empty());
    }

    #[test]
    fn test_move_piece() {
        let mut board = BitBoardState::startpos();

        // Move e2-e4
        let captured = board.move_piece(12, 28); // e2 to e4
        assert!(captured.is_none());
        assert!(board.piece_at(12).is_none());
        assert_eq!(
            board.piece_at(28),
            Some(ColoredPiece::new(PieceType::Pawn, Color::White))
        );
    }

    #[test]
    fn test_fen_roundtrip() {
        let original = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = BitBoardState::from_fen(original).unwrap();
        assert_eq!(board.to_fen(), original);

        // Test a more complex position
        let complex = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
        let board2 = BitBoardState::from_fen(complex).unwrap();
        assert_eq!(board2.to_fen(), complex);
    }

    #[test]
    fn test_is_in_check() {
        // Position where white king is in check by black rook on e8
        // 4r3/8/8/8/8/8/8/4K3 w - - 0 1
        let board = BitBoardState::from_fen("4r3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        assert!(board.is_in_check(Color::White));
        assert!(!board.is_in_check(Color::Black));

        // Starting position - no one in check
        let board2 = BitBoardState::startpos();
        assert!(!board2.is_in_check(Color::White));
        assert!(!board2.is_in_check(Color::Black));
    }

    #[test]
    fn test_king_square() {
        let board = BitBoardState::startpos();
        assert_eq!(board.king_square(Color::White), Some(4)); // e1
        assert_eq!(board.king_square(Color::Black), Some(60)); // e8
    }

    #[test]
    fn test_material_balance() {
        let board = BitBoardState::startpos();
        assert_eq!(board.material_balance(), 0);

        // Remove a black pawn
        let mut board2 = board.clone();
        board2.remove_piece(48); // a7
        assert_eq!(board2.material_balance(), 100); // White is up a pawn
    }

    #[test]
    fn test_castling_rights_fen() {
        assert_eq!(CastlingRights::ALL.to_fen(), "KQkq");
        assert_eq!(CastlingRights::NONE.to_fen(), "-");

        let mut rights = CastlingRights::ALL;
        rights.remove_kingside(Color::White);
        assert_eq!(rights.to_fen(), "Qkq");
    }

    #[test]
    fn test_piece_at_position() {
        let board = BitBoardState::startpos();
        assert_eq!(
            board.piece_at_position(&A1),
            Some(ColoredPiece::new(PieceType::Rook, Color::White))
        );
        assert_eq!(
            board.piece_at_position(&E1),
            Some(ColoredPiece::new(PieceType::King, Color::White))
        );
        assert!(board.piece_at_position(&E4).is_none());
    }
}

