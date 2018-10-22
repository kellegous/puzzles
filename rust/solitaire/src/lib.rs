use std::fmt;
use std::collections::LinkedList;

static BIT_COUNTS: &[usize] = &[
	0, // 0
	1, // 1
	1, // 2
	2, // 3
	1, // 4
	2, // 5
	2, // 6
	3, // 7
	1, // 8
	2, // 9
	2, // a
	3, // b
	2, // c
	3, // d
	3, // e
	4, // f
];

static NONE_GLYPH: char = '○';
static SOME_GLYPH: char = '●';

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

#[derive(Copy)]
pub struct Board {
    pegs: u64,
}

impl Board {
    pub fn empty() -> Board {
        Board { pegs: 0 }
    }

    pub fn set(&self, ix: u32) -> Board {
        assert!(ix <= 33);
        Board { pegs:  self.pegs | 1<<ix }
    }

    pub fn unset(&self, ix: u32) -> Board {
        assert!(ix <= 33);
        Board { pegs: self.pegs & !(1<<ix) }
    }

    pub fn has_peg(&self, ix: u32) -> bool {
        assert!(ix <= 33);
        (self.pegs >> ix) & 1 == 1
    }

    pub fn count(&self) -> usize {
        let mut c = 0;
        let mut b = self.pegs as usize;
        for _ in 0..8 {
            c += BIT_COUNTS[b & 0xf];
            b >>= 4;
        }
        c += b & 1;
        c
    }

    fn available_moves(&self) -> Vec<Board> {
        VALID_MOVES.iter().filter_map(|(mask, value)| {
            if *mask & self.pegs == *value {
                Some(Board::from(self.pegs ^ *mask))
            } else {
                None
            }
        }).collect()
    }

    fn build_solution(&self, res: &mut LinkedList<Board>) -> bool {
        if self.count() == 1 {
            res.push_front(*self);
            return true;
        }

        let moves = self.available_moves();
        if moves.is_empty() {
            return false;
        }

        for m in moves {
            if m.build_solution(res) {
                res.push_front(*self);
                return true;
            }
        }

        false
    }

    pub fn find_solution(&self) -> Option<Vec<Self>> {
        let mut res: LinkedList<Board> = LinkedList::new();
        if self.build_solution(&mut res) {
            Some(res.into_iter().collect::<Vec<Board>>())
        } else {
            None
        }
    }
}

fn glyph_for(has_peg: bool) -> char {
    if has_peg {
        SOME_GLYPH
    } else {
        NONE_GLYPH
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // row 1
        write!(f, "    ")?;
        for i in 0..3 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")?;

        // row 2
        write!(f, "    ")?;
        for i in 3..6 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")?;

        // row 3
        for i in 6..13 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")?;

        // row 4
        for i in 13..20 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")?;

        // row 5
        for i in 20..27 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")?;

        // row 6
        write!(f, "    ")?;
        for i in 27..30 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")?;

        // row 7
        write!(f, "    ")?;
        for i in 30..33 {
            write!(f, " {}", glyph_for(self.has_peg(i)))?;
        }
        write!(f, "\n")
    }
}

impl From<u64> for Board {
    fn from(v: u64) -> Self {
        Board { pegs: v & 0x1ffffffff }
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        *self
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
