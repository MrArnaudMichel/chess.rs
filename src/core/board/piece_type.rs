//! Piece type enumeration for bitboard representation.

use std::fmt;

/// Enumeration of all chess piece types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    /// Number of piece types.
    pub const COUNT: usize = 6;

    /// All piece types in order.
    pub const ALL: [PieceType; 6] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];

    /// Returns the index of the piece type (0-5).
    #[inline]
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// Creates a PieceType from an index (0-5).
    /// Returns None if the index is out of range.
    #[inline]
    pub const fn from_index(index: usize) -> Option<PieceType> {
        match index {
            0 => Some(PieceType::Pawn),
            1 => Some(PieceType::Knight),
            2 => Some(PieceType::Bishop),
            3 => Some(PieceType::Rook),
            4 => Some(PieceType::Queen),
            5 => Some(PieceType::King),
            _ => None,
        }
    }

    /// Returns the standard material value of the piece in centipawns.
    #[inline]
    pub const fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 20000, // Very high value, king can't be captured
        }
    }

    /// Returns the character representation of the piece type.
    #[inline]
    pub const fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }

    /// Creates a PieceType from a character.
    /// Accepts both uppercase and lowercase.
    #[inline]
    pub fn from_char(c: char) -> Option<PieceType> {
        match c.to_ascii_uppercase() {
            'P' => Some(PieceType::Pawn),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }

    /// Returns the name of the piece type.
    pub const fn name(&self) -> &'static str {
        match self {
            PieceType::Pawn => "Pawn",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        }
    }

    /// Returns true if this is a sliding piece (Bishop, Rook, Queen).
    #[inline]
    pub const fn is_slider(&self) -> bool {
        matches!(self, PieceType::Bishop | PieceType::Rook | PieceType::Queen)
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Color enumeration for chess pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    /// Number of colors.
    pub const COUNT: usize = 2;

    /// All colors.
    pub const ALL: [Color; 2] = [Color::White, Color::Black];

    /// Returns the index of the color (0 for White, 1 for Black).
    #[inline]
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// Returns the opposite color.
    #[inline]
    pub const fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /// Returns the pawn direction for this color (+1 for White, -1 for Black).
    #[inline]
    pub const fn pawn_direction(&self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    /// Returns the starting rank for pawns (1 for White, 6 for Black, 0-indexed).
    #[inline]
    pub const fn pawn_start_rank(&self) -> u8 {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    /// Returns the promotion rank for pawns (7 for White, 0 for Black, 0-indexed).
    #[inline]
    pub const fn promotion_rank(&self) -> u8 {
        match self {
            Color::White => 7,
            Color::Black => 0,
        }
    }

    /// Returns the back rank for this color (0 for White, 7 for Black).
    #[inline]
    pub const fn back_rank(&self) -> u8 {
        match self {
            Color::White => 0,
            Color::Black => 7,
        }
    }

    /// Converts from the legacy u8 color representation.
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Color> {
        match value {
            0 => Some(Color::White),
            1 => Some(Color::Black),
            _ => None,
        }
    }

    /// Converts to the legacy u8 color representation.
    #[inline]
    pub const fn to_u8(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

/// A piece with its type and color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColoredPiece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl ColoredPiece {
    /// Creates a new colored piece.
    #[inline]
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    /// Returns the character representation.
    /// Uppercase for White, lowercase for Black.
    pub fn to_char(&self) -> char {
        let c = self.piece_type.to_char();
        match self.color {
            Color::White => c,
            Color::Black => c.to_ascii_lowercase(),
        }
    }

    /// Creates a ColoredPiece from a FEN character.
    pub fn from_char(c: char) -> Option<Self> {
        let piece_type = PieceType::from_char(c)?;
        let color = if c.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        Some(ColoredPiece::new(piece_type, color))
    }

    /// Returns the material value (positive for white, negative for black).
    #[inline]
    pub fn signed_value(&self) -> i32 {
        match self.color {
            Color::White => self.piece_type.value(),
            Color::Black => -self.piece_type.value(),
        }
    }
}

impl fmt::Display for ColoredPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_type_index() {
        assert_eq!(PieceType::Pawn.index(), 0);
        assert_eq!(PieceType::King.index(), 5);
    }

    #[test]
    fn test_piece_type_from_index() {
        assert_eq!(PieceType::from_index(0), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_index(5), Some(PieceType::King));
        assert_eq!(PieceType::from_index(6), None);
    }

    #[test]
    fn test_piece_type_char() {
        assert_eq!(PieceType::Knight.to_char(), 'N');
        assert_eq!(PieceType::from_char('n'), Some(PieceType::Knight));
        assert_eq!(PieceType::from_char('N'), Some(PieceType::Knight));
    }

    #[test]
    fn test_color_opposite() {
        assert_eq!(Color::White.opposite(), Color::Black);
        assert_eq!(Color::Black.opposite(), Color::White);
    }

    #[test]
    fn test_color_pawn_direction() {
        assert_eq!(Color::White.pawn_direction(), 1);
        assert_eq!(Color::Black.pawn_direction(), -1);
    }

    #[test]
    fn test_colored_piece_char() {
        let white_knight = ColoredPiece::new(PieceType::Knight, Color::White);
        assert_eq!(white_knight.to_char(), 'N');

        let black_knight = ColoredPiece::new(PieceType::Knight, Color::Black);
        assert_eq!(black_knight.to_char(), 'n');

        assert_eq!(ColoredPiece::from_char('N'), Some(white_knight));
        assert_eq!(ColoredPiece::from_char('n'), Some(black_knight));
    }

    #[test]
    fn test_is_slider() {
        assert!(!PieceType::Pawn.is_slider());
        assert!(!PieceType::Knight.is_slider());
        assert!(PieceType::Bishop.is_slider());
        assert!(PieceType::Rook.is_slider());
        assert!(PieceType::Queen.is_slider());
        assert!(!PieceType::King.is_slider());
    }
}

