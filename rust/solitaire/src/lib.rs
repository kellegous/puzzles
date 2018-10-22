static NONE_GLYPH: char = '○';
static SOME_GLYPH: char = '●';

/// A list of all valid moves in the game. You can use this if it's helpful,
/// it is used to validate the moves in the tests.
/// 
/// Each item in the slice contains two values, as mask and a value. The mask
/// has 1's for each pin that would be involved in the move. The lsb corresponds
/// to the pin in the upper left corner. The value has the value that needs to be
/// found (after it has been masked) for the move to available. As an example, the
/// first move is to move the 0th peg into the 2nd peg by jumping the first peg.
/// This move is represented by a mask of 0x7 (111) and a value of 0x3 (011).
static VALID_MOVES: &[(u64, u64)] = &[
    (0x7, 0x3),
    (0x109, 0x9),
    (0x212, 0x12),
    (0x424, 0x24),
    (0x7, 0x6),
    (0x38, 0x18),
    (0x8108, 0x108),
    (0x10210, 0x210),
    (0x20420, 0x420),
    (0x38, 0x30),
    (0x1c0, 0xc0),
    (0x102040, 0x2040),
    (0x380, 0x180),
    (0x204080, 0x4080),
    (0x109, 0x108),
    (0x700, 0x300),
    (0x408100, 0x8100),
    (0x1c0, 0x180),
    (0x212, 0x210),
    (0xe00, 0x600),
    (0x810200, 0x10200),
    (0x380, 0x300),
    (0x424, 0x420),
    (0x1c00, 0xc00),
    (0x1020400, 0x20400),
    (0x700, 0x600),
    (0x2040800, 0x40800),
    (0xe00, 0xc00),
    (0x4081000, 0x81000),
    (0x1c00, 0x1800),
    (0xe000, 0x6000),
    (0x1c000, 0xc000),
    (0x8108, 0x8100),
    (0x38000, 0x18000),
    (0x8408000, 0x408000),
    (0xe000, 0xc000),
    (0x10210, 0x10200),
    (0x70000, 0x30000),
    (0x10810000, 0x810000),
    (0x1c000, 0x18000),
    (0x20420, 0x20400),
    (0xe0000, 0x60000),
    (0x21020000, 0x1020000),
    (0x38000, 0x30000),
    (0x70000, 0x60000),
    (0xe0000, 0xc0000),
    (0x102040, 0x102000),
    (0x700000, 0x300000),
    (0x204080, 0x204000),
    (0xe00000, 0x600000),
    (0x408100, 0x408000),
    (0x1c00000, 0xc00000),
    (0x48400000, 0x8400000),
    (0x700000, 0x600000),
    (0x810200, 0x810000),
    (0x3800000, 0x1800000),
    (0x90800000, 0x10800000),
    (0xe00000, 0xc00000),
    (0x1020400, 0x1020000),
    (0x7000000, 0x3000000),
    (0x121000000, 0x21000000),
    (0x1c00000, 0x1800000),
    (0x2040800, 0x2040000),
    (0x3800000, 0x3000000),
    (0x4081000, 0x4080000),
    (0x7000000, 0x6000000),
    (0x8408000, 0x8400000),
    (0x38000000, 0x18000000),
    (0x10810000, 0x10800000),
    (0x21020000, 0x21000000),
    (0x38000000, 0x30000000),
    (0x48400000, 0x48000000),
    (0x1c0000000, 0xc0000000),
    (0x90800000, 0x90000000),
    (0x121000000, 0x120000000),
    (0x1c0000000, 0x180000000)
];

pub struct Board;

impl Board {
    pub fn empty() -> Board {
        unimplemented!();
    }

    pub fn set(&self, ix: u32) -> Board {
        unimplemented!();
    }

    pub fn unset(&self, ix: u32) -> Board {
        unimplemented!();
    }

    pub fn has_peg(&self, ix: u32) -> bool {
        unimplemented!();
    }

    pub fn count(&self) -> usize {
        unimplemented!();
    }

    /// Returns all moves needed to solve the board. Each item in the
    /// vector should represent the board after each move has been taken.
    pub fn find_solution(&self) -> Option<Vec<Self>> {
        unimplemented!();
    }
}

impl From<u64> for Board {
    /// Creates a board from a bitset. The lsb corresponds to the upper left
    /// peg. The lower right peg is represented by the 33rd bit.
    fn from(v: u64) -> Self {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use ::Board;
    use ::VALID_MOVES;

    #[test]
    fn set_unset_and_count_works() {
        let mut board = Board::empty();
        assert_eq!(board.count(), 0);

        for i in 0..33 {
            assert!(!board.has_peg(i));
            board = board.set(i);
            assert!(board.has_peg(i));
            assert_eq!(board.count(), i as usize + 1);
        }

        for i in 0..33 {
            board = board.unset(i);
            assert!(!board.has_peg(i));
            assert_eq!(board.count(), 32 - i as usize);
        }
    }

    #[test]
    fn from_u64_works() {
        let board = Board::from(0b111111111111111111111111111111111u64);
        for i in 0..33 {
            assert!(board.has_peg(i));
        }
        assert_eq!(board.count(), 33);
    }

    /// Turn a board into a u64 bitset where the lower 33 bits represent
    /// the pegs on the board.
    fn to_bit_set(b: Board) -> u64 {
        let mut v = 0u64;
        for i in 0..33 {
            if b.has_peg(i) {
                v |= 1<<i;
            }
        }
        v
    }

    fn assert_valid_solution(sol: &Vec<Board>) {
        assert_eq!(sol.len(), 32);
        for i in 1..32 {
            let fr = to_bit_set(sol[i - 1]);
            let to = to_bit_set(sol[i]);
            assert_eq!(sol[i].count(), 32 - i);
            match VALID_MOVES.iter().find(|(mask, _)| fr^*mask == to) {
                Some(_) => continue,
                None => panic!("not a valid move from {} to {}", i - 1, i),
            }
        }
    }

    #[test]
    fn finds_solution() {
        let board = Board::from(0x1ffffeffff);
        let solution = board.find_solution().unwrap();
        assert_valid_solution(&solution);
    }

    #[test]
    fn no_solution() {
        let board = Board::empty()
            .set(0)
            .set(32);
        if let Some(_) = board.find_solution() {
            panic!("no solution expected");
        }
    }

    #[test]
    fn can_display() {
        let board = Board::from(0x144111045);
        assert_eq!(
            format!("{}", board),
            "     ● ○ ●\n     ○ ○ ○\n ● ○ ○ ○ ○ ○ ●\n ○ ○ ○ ● ○ ○ ○\n ● ○ ○ ○ ○ ○ ●\n     ○ ○ ○\n     ● ○ ●\n");
    }
}
