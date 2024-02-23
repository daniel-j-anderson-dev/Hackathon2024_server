use std::ops::{Index, IndexMut};

use color_eyre::{eyre::eyre, Report};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Board {
    board: [[Cell; 3]; 3],
}
impl Board {
    pub fn set_cell(&mut self, index: CellIndex, value: Cell) {
        self[index] = value;
    }
    pub fn get_cell(&self, index: CellIndex) -> Cell {
        return self[index];
    }
    pub fn iter(&self) -> impl Iterator<Item = Cell> {
        return self.board.into_iter().flatten();
    }
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (CellIndex, Cell)> {
        return self.iter().enumerate().map(|(i, cell)| {
            (
                CellIndex::try_from(i).expect("`Board::iter` has 9 elements so `i` will never be larger than eight"),
                cell,
            )
        })
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        return self.board.iter_mut().flatten();
    }
    pub fn iter_mut_enumerated(&mut self) -> impl Iterator<Item = (CellIndex, &mut Cell)> {
        return self.iter_mut().enumerate().map(|(i, cell)| {
            (
                CellIndex::try_from(i).expect("`Board::iter_mut` has 9 elements so `i` will never be larger than eight"),
                cell,
            )
        })
    }
}
impl IndexMut<CellIndex> for Board {
    fn index_mut(&mut self, i: CellIndex) -> &mut Self::Output {
        return self.board.index_mut(i.row()).index_mut(i.column());
    }
}
impl Index<CellIndex> for Board {
    type Output = Cell;
    fn index(&self, i: CellIndex) -> &Self::Output {
        return self.board.index(i.row()).index(i.column());
    }
}

/// This struct granites that the two indices are 0, 1, or 2.
pub struct CellIndex {
    row_index: usize,
    column_index: usize,
}
impl TryFrom<usize> for CellIndex {
    type Error = Report;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(CellIndex::TOP_LEFT),
            1 => Ok(CellIndex::TOP_MIDDLE),
            2 => Ok(CellIndex::TOP_RIGHT),
            3 => Ok(CellIndex::MIDDLE_LEFT),
            4 => Ok(CellIndex::CENTER),
            5 => Ok(CellIndex::MIDDLE_RIGHT),
            6 => Ok(CellIndex::BOTTOM_LEFT),
            7 => Ok(CellIndex::BOTTOM_MIDDLE),
            8 => Ok(CellIndex::BOTTOM_RIGHT),
            invalid_value => Err(eyre!("{invalid_value} is not a valid absolute cell index. there are only 9 cells on a TicTacToe game"))
        };
    }
}
impl TryFrom<(usize, usize)> for CellIndex {
    type Error = Report;
    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        return CellIndex::new(value.0, value.1);
    }
}
impl CellIndex {
    /// This function ensures that `row_index` and `column_index` are in the interval `[0, 2]`
    pub fn new(row_index: usize, column_index: usize) -> Result<CellIndex, Report> {
        if row_index < 3 && column_index < 3 {
            return Ok(CellIndex {
                row_index,
                column_index,
            });
        } else {
            let error = eyre!("({row_index}, {column_index}) is not within a three by three grid needed for TicTacToe");
            return Err(error);
        }
    }
    pub fn row(&self) -> usize {
        return self.row_index;
    }
    pub fn column(&self) -> usize {
        return self.column_index;
    }
    pub fn all_indices_in_order() -> [CellIndex; 9] {
        return [
            CellIndex {
                row_index: 0,
                column_index: 0,
            },
            CellIndex {
                row_index: 0,
                column_index: 1,
            },
            CellIndex {
                row_index: 0,
                column_index: 2,
            },
            CellIndex {
                row_index: 1,
                column_index: 0,
            },
            CellIndex {
                row_index: 1,
                column_index: 1,
            },
            CellIndex {
                row_index: 1,
                column_index: 2,
            },
            CellIndex {
                row_index: 2,
                column_index: 0,
            },
            CellIndex {
                row_index: 2,
                column_index: 1,
            },
            CellIndex {
                row_index: 2,
                column_index: 2,
            },
        ];
    }
    pub const TOP_LEFT: Self = CellIndex {
        row_index: 0,
        column_index: 0,
    };
    pub const TOP_MIDDLE: Self = CellIndex {
        row_index: 0,
        column_index: 1,
    };
    pub const TOP_RIGHT: Self = CellIndex {
        row_index: 0,
        column_index: 2,
    };
    pub const MIDDLE_LEFT: Self = CellIndex {
        row_index: 1,
        column_index: 0,
    };
    pub const CENTER: Self = CellIndex {
        row_index: 1,
        column_index: 1,
    };
    pub const MIDDLE_RIGHT: Self = CellIndex {
        row_index: 1,
        column_index: 2,
    };
    pub const BOTTOM_LEFT: Self = CellIndex {
        row_index: 2,
        column_index: 0,
    };
    pub const BOTTOM_MIDDLE: Self = CellIndex {
        row_index: 2,
        column_index: 1,
    };
    pub const BOTTOM_RIGHT: Self = CellIndex {
        row_index: 2,
        column_index: 2,
    };
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub enum Cell {
    #[default]
    Empty,
    X,
    O,
}
