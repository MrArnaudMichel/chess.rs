//! Attack generation functions for chess pieces using bitboards.
//!
//! This module provides functions to compute attack patterns for all piece types.
//! For knights and kings, we use precomputed lookup tables.
//! For sliding pieces (bishops, rooks, queens), we use classical ray-casting
//! (magic bitboards can be added later for optimization).

use super::bitboard::{BitBoard, NOT_A_FILE, NOT_H_FILE, NOT_AB_FILE, NOT_GH_FILE, RANK_2, RANK_7};
use super::piece_type::Color;

/// Precomputed knight attack masks for all 64 squares.
static KNIGHT_ATTACKS: [BitBoard; 64] = {
    let mut attacks = [BitBoard::EMPTY; 64];
    let mut sq = 0u8;
    while sq < 64 {
        attacks[sq as usize] = compute_knight_attacks(sq);
        sq += 1;
    }
    attacks
};

/// Precomputed king attack masks for all 64 squares.
static KING_ATTACKS: [BitBoard; 64] = {
    let mut attacks = [BitBoard::EMPTY; 64];
    let mut sq = 0u8;
    while sq < 64 {
        attacks[sq as usize] = compute_king_attacks(sq);
        sq += 1;
    }
    attacks
};

/// Computes knight attack pattern for a given square at compile time.
const fn compute_knight_attacks(square: u8) -> BitBoard {
    let bb = BitBoard::new(1u64 << square);

    // Knight moves in an L-shape: 2 squares in one direction, 1 in perpendicular
    let mut attacks = 0u64;

    // North-North-East: +17 (up 2, right 1)
    attacks |= (bb.0 << 17) & NOT_A_FILE.0;
    // North-North-West: +15 (up 2, left 1)
    attacks |= (bb.0 << 15) & NOT_H_FILE.0;
    // North-East-East: +10 (up 1, right 2)
    attacks |= (bb.0 << 10) & NOT_AB_FILE.0;
    // North-West-West: +6 (up 1, left 2)
    attacks |= (bb.0 << 6) & NOT_GH_FILE.0;

    // South-South-East: -15 (down 2, right 1)
    attacks |= (bb.0 >> 15) & NOT_A_FILE.0;
    // South-South-West: -17 (down 2, left 1)
    attacks |= (bb.0 >> 17) & NOT_H_FILE.0;
    // South-East-East: -6 (down 1, right 2)
    attacks |= (bb.0 >> 6) & NOT_AB_FILE.0;
    // South-West-West: -10 (down 1, left 2)
    attacks |= (bb.0 >> 10) & NOT_GH_FILE.0;

    BitBoard::new(attacks)
}

/// Computes king attack pattern for a given square at compile time.
const fn compute_king_attacks(square: u8) -> BitBoard {
    let bb = BitBoard::new(1u64 << square);

    let mut attacks = 0u64;

    // North
    attacks |= bb.0 << 8;
    // South
    attacks |= bb.0 >> 8;
    // East (avoiding A-file wrap)
    attacks |= (bb.0 << 1) & NOT_A_FILE.0;
    // West (avoiding H-file wrap)
    attacks |= (bb.0 >> 1) & NOT_H_FILE.0;
    // North-East
    attacks |= (bb.0 << 9) & NOT_A_FILE.0;
    // North-West
    attacks |= (bb.0 << 7) & NOT_H_FILE.0;
    // South-East
    attacks |= (bb.0 >> 7) & NOT_A_FILE.0;
    // South-West
    attacks |= (bb.0 >> 9) & NOT_H_FILE.0;

    BitBoard::new(attacks)
}

/// Returns the precomputed knight attacks for a given square.
#[inline]
pub fn knight_attacks(square: u8) -> BitBoard {
    KNIGHT_ATTACKS[square as usize]
}

/// Returns the precomputed king attacks for a given square.
#[inline]
pub fn king_attacks(square: u8) -> BitBoard {
    KING_ATTACKS[square as usize]
}

/// Generates pawn attacks (captures) for a single pawn.
#[inline]
pub fn pawn_attacks(square: u8, color: Color) -> BitBoard {
    let bb = BitBoard::from_square(square);
    match color {
        Color::White => {
            let left = (bb.0 << 7) & NOT_H_FILE.0;
            let right = (bb.0 << 9) & NOT_A_FILE.0;
            BitBoard::new(left | right)
        }
        Color::Black => {
            let left = (bb.0 >> 9) & NOT_H_FILE.0;
            let right = (bb.0 >> 7) & NOT_A_FILE.0;
            BitBoard::new(left | right)
        }
    }
}

/// Generates pawn push targets (non-captures) for a set of pawns.
pub fn pawn_single_pushes(pawns: BitBoard, empty: BitBoard, color: Color) -> BitBoard {
    match color {
        Color::White => pawns.north() & empty,
        Color::Black => pawns.south() & empty,
    }
}

/// Generates pawn double push targets for pawns on their starting rank.
pub fn pawn_double_pushes(pawns: BitBoard, empty: BitBoard, color: Color) -> BitBoard {
    match color {
        Color::White => {
            let single = (pawns & RANK_2).north() & empty;
            single.north() & empty
        }
        Color::Black => {
            let single = (pawns & RANK_7).south() & empty;
            single.south() & empty
        }
    }
}

/// Generates all pawn attacks for a set of pawns.
pub fn pawn_attacks_all(pawns: BitBoard, color: Color) -> BitBoard {
    match color {
        Color::White => {
            let left = (pawns.0 << 7) & NOT_H_FILE.0;
            let right = (pawns.0 << 9) & NOT_A_FILE.0;
            BitBoard::new(left | right)
        }
        Color::Black => {
            let left = (pawns.0 >> 9) & NOT_H_FILE.0;
            let right = (pawns.0 >> 7) & NOT_A_FILE.0;
            BitBoard::new(left | right)
        }
    }
}

/// Generates bishop attacks using classical ray-casting.
/// `occupied` is the bitboard of all occupied squares.
pub fn bishop_attacks(square: u8, occupied: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    // North-East ray
    attacks |= ray_attacks(square, occupied, 9, NOT_A_FILE);
    // North-West ray
    attacks |= ray_attacks(square, occupied, 7, NOT_H_FILE);
    // South-East ray
    attacks |= ray_attacks_negative(square, occupied, 7, NOT_A_FILE);
    // South-West ray
    attacks |= ray_attacks_negative(square, occupied, 9, NOT_H_FILE);

    attacks
}

/// Generates rook attacks using classical ray-casting.
/// `occupied` is the bitboard of all occupied squares.
pub fn rook_attacks(square: u8, occupied: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;

    // North ray
    attacks |= ray_attacks(square, occupied, 8, BitBoard::FULL);
    // East ray
    attacks |= ray_attacks(square, occupied, 1, NOT_A_FILE);
    // South ray
    attacks |= ray_attacks_negative(square, occupied, 8, BitBoard::FULL);
    // West ray
    attacks |= ray_attacks_negative(square, occupied, 1, NOT_H_FILE);

    attacks
}

/// Generates queen attacks (combination of bishop and rook attacks).
pub fn queen_attacks(square: u8, occupied: BitBoard) -> BitBoard {
    bishop_attacks(square, occupied) | rook_attacks(square, occupied)
}

/// Helper function for positive direction ray attacks.
fn ray_attacks(square: u8, occupied: BitBoard, shift: u8, mask: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;
    let mut current = BitBoard::from_square(square);

    loop {
        current = BitBoard::new(current.0 << shift) & mask;
        if current.is_empty() {
            break;
        }
        attacks |= current;
        if (current & occupied).is_not_empty() {
            break;
        }
    }

    attacks
}

/// Helper function for negative direction ray attacks.
fn ray_attacks_negative(square: u8, occupied: BitBoard, shift: u8, mask: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::EMPTY;
    let mut current = BitBoard::from_square(square);

    loop {
        current = BitBoard::new(current.0 >> shift) & mask;
        if current.is_empty() {
            break;
        }
        attacks |= current;
        if (current & occupied).is_not_empty() {
            break;
        }
    }

    attacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_center() {
        // Knight on e4 (square 28) should attack 8 squares
        let attacks = knight_attacks(28);
        assert_eq!(attacks.count(), 8);

        // Check specific squares: d6, f6, c5, g5, c3, g3, d2, f2
        assert!(attacks.is_set(43)); // d6
        assert!(attacks.is_set(45)); // f6
        assert!(attacks.is_set(34)); // c5
        assert!(attacks.is_set(38)); // g5
        assert!(attacks.is_set(18)); // c3
        assert!(attacks.is_set(22)); // g3
        assert!(attacks.is_set(11)); // d2
        assert!(attacks.is_set(13)); // f2
    }

    #[test]
    fn test_knight_attacks_corner() {
        // Knight on a1 (square 0) should attack only 2 squares
        let attacks = knight_attacks(0);
        assert_eq!(attacks.count(), 2);
        assert!(attacks.is_set(10)); // c2
        assert!(attacks.is_set(17)); // b3
    }

    #[test]
    fn test_king_attacks_center() {
        // King on e4 (square 28) should attack 8 squares
        let attacks = king_attacks(28);
        assert_eq!(attacks.count(), 8);
    }

    #[test]
    fn test_king_attacks_corner() {
        // King on a1 (square 0) should attack 3 squares
        let attacks = king_attacks(0);
        assert_eq!(attacks.count(), 3);
        assert!(attacks.is_set(1));  // b1
        assert!(attacks.is_set(8));  // a2
        assert!(attacks.is_set(9));  // b2
    }

    #[test]
    fn test_pawn_attacks_white() {
        // White pawn on e4 (square 28)
        let attacks = pawn_attacks(28, Color::White);
        assert_eq!(attacks.count(), 2);
        assert!(attacks.is_set(35)); // d5
        assert!(attacks.is_set(37)); // f5
    }

    #[test]
    fn test_pawn_attacks_black() {
        // Black pawn on e5 (square 36)
        let attacks = pawn_attacks(36, Color::Black);
        assert_eq!(attacks.count(), 2);
        assert!(attacks.is_set(27)); // d4
        assert!(attacks.is_set(29)); // f4
    }

    #[test]
    fn test_pawn_attacks_edge() {
        // White pawn on a4 (square 24) - left edge
        let attacks = pawn_attacks(24, Color::White);
        assert_eq!(attacks.count(), 1);
        assert!(attacks.is_set(33)); // b5 only

        // White pawn on h4 (square 31) - right edge
        let attacks = pawn_attacks(31, Color::White);
        assert_eq!(attacks.count(), 1);
        assert!(attacks.is_set(38)); // g5 only
    }

    #[test]
    fn test_rook_attacks_empty_board() {
        // Rook on e4 (square 28) on empty board
        let attacks = rook_attacks(28, BitBoard::EMPTY);
        assert_eq!(attacks.count(), 14); // 7 horizontal + 7 vertical
    }

    #[test]
    fn test_rook_attacks_with_blockers() {
        // Rook on e4 (square 28), blocker on e6 (square 44)
        let occupied = BitBoard::from_square(44);
        let attacks = rook_attacks(28, occupied);

        // Should include e5 and e6 but not e7, e8
        assert!(attacks.is_set(36)); // e5
        assert!(attacks.is_set(44)); // e6 (blocker)
        assert!(!attacks.is_set(52)); // e7
        assert!(!attacks.is_set(60)); // e8
    }

    #[test]
    fn test_bishop_attacks_empty_board() {
        // Bishop on e4 (square 28) on empty board
        let attacks = bishop_attacks(28, BitBoard::EMPTY);
        assert_eq!(attacks.count(), 13);
    }

    #[test]
    fn test_queen_attacks_empty_board() {
        // Queen on e4 (square 28) on empty board
        let attacks = queen_attacks(28, BitBoard::EMPTY);
        assert_eq!(attacks.count(), 27); // 14 (rook) + 13 (bishop)
    }

    #[test]
    fn test_pawn_single_pushes() {
        let pawns = BitBoard::from_square(12); // e2
        let empty = BitBoard::FULL; // All squares empty for testing
        let pushes = pawn_single_pushes(pawns, empty, Color::White);
        assert!(pushes.is_set(20)); // e3
    }

    #[test]
    fn test_pawn_double_pushes() {
        let pawns = BitBoard::from_square(12); // e2
        let empty = BitBoard::FULL;
        let pushes = pawn_double_pushes(pawns, empty, Color::White);
        assert!(pushes.is_set(28)); // e4
    }
}

