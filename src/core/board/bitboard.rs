//! Bitboard representation for efficient board operations.
//!
//! A bitboard is a 64-bit integer where each bit represents a square on the chess board.
//! Bit 0 represents A1, bit 7 represents H1, bit 56 represents A8, and bit 63 represents H8.
//!
//! This representation allows for fast bit operations to compute moves, attacks, and other
//! board-related calculations.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};
use crate::core::types::position::Position;

/// A 64-bit representation of the chess board.
///
/// Each bit corresponds to a square:
/// - Bit 0 = A1, Bit 1 = B1, ..., Bit 7 = H1
/// - Bit 8 = A2, Bit 9 = B2, ..., Bit 15 = H2
/// - ...
/// - Bit 56 = A8, Bit 57 = B8, ..., Bit 63 = H8
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Empty bitboard with no bits set.
    pub const EMPTY: BitBoard = BitBoard(0);

    /// Full bitboard with all 64 bits set.
    pub const FULL: BitBoard = BitBoard(0xFFFF_FFFF_FFFF_FFFF);

    /// Creates a new BitBoard from a u64 value.
    #[inline]
    pub const fn new(value: u64) -> Self {
        BitBoard(value)
    }

    /// Creates a BitBoard with a single bit set at the given square index (0-63).
    #[inline]
    pub const fn from_square(square: u8) -> Self {
        BitBoard(1u64 << square)
    }

    /// Creates a BitBoard from a Position.
    #[inline]
    pub fn from_position(pos: &Position) -> Self {
        let square = pos.y as u8 * 8 + pos.x as u8;
        BitBoard::from_square(square)
    }

    /// Returns the raw u64 value.
    #[inline]
    pub const fn value(&self) -> u64 {
        self.0
    }

    /// Returns true if the bitboard is empty (no bits set).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if the bitboard is not empty.
    #[inline]
    pub const fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    /// Returns the number of set bits (population count).
    #[inline]
    pub const fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns true if the bit at the given square is set.
    #[inline]
    pub const fn is_set(&self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    /// Returns true if the bit at the given position is set.
    #[inline]
    pub fn is_position_set(&self, pos: &Position) -> bool {
        let square = pos.y as u8 * 8 + pos.x as u8;
        self.is_set(square)
    }

    /// Sets the bit at the given square.
    #[inline]
    pub fn set(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    /// Sets the bit at the given position.
    #[inline]
    pub fn set_position(&mut self, pos: &Position) {
        let square = pos.y as u8 * 8 + pos.x as u8;
        self.set(square);
    }

    /// Clears the bit at the given square.
    #[inline]
    pub fn clear(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }

    /// Clears the bit at the given position.
    #[inline]
    pub fn clear_position(&mut self, pos: &Position) {
        let square = pos.y as u8 * 8 + pos.x as u8;
        self.clear(square);
    }

    /// Toggles the bit at the given square.
    #[inline]
    pub fn toggle(&mut self, square: u8) {
        self.0 ^= 1u64 << square;
    }

    /// Returns the index of the least significant bit (LSB).
    /// Returns None if the bitboard is empty.
    #[inline]
    pub const fn lsb(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    /// Returns the index of the most significant bit (MSB).
    /// Returns None if the bitboard is empty.
    #[inline]
    pub const fn msb(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(63 - self.0.leading_zeros() as u8)
        }
    }

    /// Removes and returns the least significant bit.
    /// Returns None if the bitboard is empty.
    #[inline]
    pub fn pop_lsb(&mut self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            let lsb = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1; // Clear the LSB
            Some(lsb)
        }
    }

    /// Converts a square index to a Position.
    #[inline]
    pub fn square_to_position(square: u8) -> Position {
        Position::new((square % 8) as i8, (square / 8) as i8)
    }

    /// Converts a Position to a square index.
    #[inline]
    pub fn position_to_square(pos: &Position) -> u8 {
        pos.y as u8 * 8 + pos.x as u8
    }

    /// Returns an iterator over all set squares.
    pub fn iter(&self) -> BitBoardIterator {
        BitBoardIterator { bb: *self }
    }

    /// Shifts the bitboard north (towards rank 8).
    #[inline]
    pub const fn north(&self) -> BitBoard {
        BitBoard(self.0 << 8)
    }

    /// Shifts the bitboard south (towards rank 1).
    #[inline]
    pub const fn south(&self) -> BitBoard {
        BitBoard(self.0 >> 8)
    }

    /// Shifts the bitboard east (towards file H).
    #[inline]
    pub const fn east(&self) -> BitBoard {
        BitBoard((self.0 << 1) & NOT_A_FILE.0)
    }

    /// Shifts the bitboard west (towards file A).
    #[inline]
    pub const fn west(&self) -> BitBoard {
        BitBoard((self.0 >> 1) & NOT_H_FILE.0)
    }

    /// Shifts the bitboard north-east.
    #[inline]
    pub const fn north_east(&self) -> BitBoard {
        BitBoard((self.0 << 9) & NOT_A_FILE.0)
    }

    /// Shifts the bitboard north-west.
    #[inline]
    pub const fn north_west(&self) -> BitBoard {
        BitBoard((self.0 << 7) & NOT_H_FILE.0)
    }

    /// Shifts the bitboard south-east.
    #[inline]
    pub const fn south_east(&self) -> BitBoard {
        BitBoard((self.0 >> 7) & NOT_A_FILE.0)
    }

    /// Shifts the bitboard south-west.
    #[inline]
    pub const fn south_west(&self) -> BitBoard {
        BitBoard((self.0 >> 9) & NOT_H_FILE.0)
    }
}

/// Iterator over set bits in a BitBoard.
pub struct BitBoardIterator {
    bb: BitBoard,
}

impl Iterator for BitBoardIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.bb.pop_lsb()
    }
}

// Implement standard bit operations

impl BitAnd for BitBoard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Shl<u8> for BitBoard {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<u8> for BitBoard {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

// Constants for file masks
pub const A_FILE: BitBoard = BitBoard(0x0101_0101_0101_0101);
pub const B_FILE: BitBoard = BitBoard(0x0202_0202_0202_0202);
pub const C_FILE: BitBoard = BitBoard(0x0404_0404_0404_0404);
pub const D_FILE: BitBoard = BitBoard(0x0808_0808_0808_0808);
pub const E_FILE: BitBoard = BitBoard(0x1010_1010_1010_1010);
pub const F_FILE: BitBoard = BitBoard(0x2020_2020_2020_2020);
pub const G_FILE: BitBoard = BitBoard(0x4040_4040_4040_4040);
pub const H_FILE: BitBoard = BitBoard(0x8080_8080_8080_8080);

pub const NOT_A_FILE: BitBoard = BitBoard(!A_FILE.0);
pub const NOT_H_FILE: BitBoard = BitBoard(!H_FILE.0);
pub const NOT_AB_FILE: BitBoard = BitBoard(!(A_FILE.0 | B_FILE.0));
pub const NOT_GH_FILE: BitBoard = BitBoard(!(G_FILE.0 | H_FILE.0));

// Constants for rank masks
pub const RANK_1: BitBoard = BitBoard(0x0000_0000_0000_00FF);
pub const RANK_2: BitBoard = BitBoard(0x0000_0000_0000_FF00);
pub const RANK_3: BitBoard = BitBoard(0x0000_0000_00FF_0000);
pub const RANK_4: BitBoard = BitBoard(0x0000_0000_FF00_0000);
pub const RANK_5: BitBoard = BitBoard(0x0000_00FF_0000_0000);
pub const RANK_6: BitBoard = BitBoard(0x0000_FF00_0000_0000);
pub const RANK_7: BitBoard = BitBoard(0x00FF_0000_0000_0000);
pub const RANK_8: BitBoard = BitBoard(0xFF00_0000_0000_0000);

// Square constants (for convenience)
pub const A1_BB: BitBoard = BitBoard(1 << 0);
pub const B1_BB: BitBoard = BitBoard(1 << 1);
pub const C1_BB: BitBoard = BitBoard(1 << 2);
pub const D1_BB: BitBoard = BitBoard(1 << 3);
pub const E1_BB: BitBoard = BitBoard(1 << 4);
pub const F1_BB: BitBoard = BitBoard(1 << 5);
pub const G1_BB: BitBoard = BitBoard(1 << 6);
pub const H1_BB: BitBoard = BitBoard(1 << 7);

pub const A8_BB: BitBoard = BitBoard(1 << 56);
pub const B8_BB: BitBoard = BitBoard(1 << 57);
pub const C8_BB: BitBoard = BitBoard(1 << 58);
pub const D8_BB: BitBoard = BitBoard(1 << 59);
pub const E8_BB: BitBoard = BitBoard(1 << 60);
pub const F8_BB: BitBoard = BitBoard(1 << 61);
pub const G8_BB: BitBoard = BitBoard(1 << 62);
pub const H8_BB: BitBoard = BitBoard(1 << 63);

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.is_set(square) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::position::{A1, E4, H8};

    #[test]
    fn test_from_square() {
        let bb = BitBoard::from_square(0);
        assert_eq!(bb.0, 1);

        let bb = BitBoard::from_square(63);
        assert_eq!(bb.0, 1u64 << 63);
    }

    #[test]
    fn test_from_position() {
        let bb = BitBoard::from_position(&A1);
        assert_eq!(bb.0, 1);

        let bb = BitBoard::from_position(&H8);
        assert_eq!(bb.0, 1u64 << 63);

        let bb = BitBoard::from_position(&E4);
        assert_eq!(bb.0, 1u64 << 28); // e4 = 4 + 3*8 = 28
    }

    #[test]
    fn test_count() {
        assert_eq!(BitBoard::EMPTY.count(), 0);
        assert_eq!(BitBoard::FULL.count(), 64);
        assert_eq!(BitBoard::from_square(0).count(), 1);
        assert_eq!(RANK_1.count(), 8);
    }

    #[test]
    fn test_is_set() {
        let bb = BitBoard::from_square(28);
        assert!(bb.is_set(28));
        assert!(!bb.is_set(27));
        assert!(!bb.is_set(29));
    }

    #[test]
    fn test_set_and_clear() {
        let mut bb = BitBoard::EMPTY;
        bb.set(0);
        assert!(bb.is_set(0));

        bb.set(63);
        assert!(bb.is_set(63));
        assert_eq!(bb.count(), 2);

        bb.clear(0);
        assert!(!bb.is_set(0));
        assert_eq!(bb.count(), 1);
    }

    #[test]
    fn test_lsb_msb() {
        let bb = BitBoard(0b10100);
        assert_eq!(bb.lsb(), Some(2));
        assert_eq!(bb.msb(), Some(4));

        assert_eq!(BitBoard::EMPTY.lsb(), None);
        assert_eq!(BitBoard::EMPTY.msb(), None);
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = BitBoard(0b10100);
        assert_eq!(bb.pop_lsb(), Some(2));
        assert_eq!(bb.0, 0b10000);
        assert_eq!(bb.pop_lsb(), Some(4));
        assert_eq!(bb.0, 0);
        assert_eq!(bb.pop_lsb(), None);
    }

    #[test]
    fn test_iterator() {
        let bb = BitBoard(0b10101);
        let squares: Vec<u8> = bb.iter().collect();
        assert_eq!(squares, vec![0, 2, 4]);
    }

    #[test]
    fn test_directional_shifts() {
        let e4 = BitBoard::from_square(28);

        // North: e4 -> e5 (square 36)
        assert_eq!(e4.north().lsb(), Some(36));

        // South: e4 -> e3 (square 20)
        assert_eq!(e4.south().lsb(), Some(20));

        // East: e4 -> f4 (square 29)
        assert_eq!(e4.east().lsb(), Some(29));

        // West: e4 -> d4 (square 27)
        assert_eq!(e4.west().lsb(), Some(27));
    }

    #[test]
    fn test_file_wrapping() {
        // Test that east shift doesn't wrap from H to A file
        let h4 = BitBoard::from_square(31); // h4
        assert!(h4.east().is_empty());

        // Test that west shift doesn't wrap from A to H file
        let a4 = BitBoard::from_square(24); // a4
        assert!(a4.west().is_empty());
    }

    #[test]
    fn test_bit_operations() {
        let a = BitBoard(0b1100);
        let b = BitBoard(0b1010);

        assert_eq!((a & b).0, 0b1000);
        assert_eq!((a | b).0, 0b1110);
        assert_eq!((a ^ b).0, 0b0110);
        assert_eq!((!BitBoard(0)).0, u64::MAX);
    }

    #[test]
    fn test_square_position_conversion() {
        let pos = BitBoard::square_to_position(28);
        assert_eq!(pos.x, 4);
        assert_eq!(pos.y, 3);

        let square = BitBoard::position_to_square(&E4);
        assert_eq!(square, 28);
    }
}

