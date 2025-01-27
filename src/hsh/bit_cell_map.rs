/*
 * Copyright 2005-2024 Daniel Chay
 *
 *
 *  This file is part of Isomech.
 *
 *  Isomech is free software  you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Isomech is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with Isomech.  If not, see <http://www.gnu.org/licenses/>.
 */
#![allow(unused)]
use std::sync::Arc;

const ONE: u64 = 1;
const _64BIT: u8 = 64;
const _64BIT_N1: u8 = _64BIT - 1;

/// Defines one dimension of a hash that may have N dimensions
#[allow(non_snake_case)]
pub struct BitCellMap {
    CELL: Arc<Vec<u8>>,
    CELLSHIFT: Vec<u8>,
    CELLMASK: Vec<u64>,
    CELLZERO: Vec<u64>,
    Cells: u8,
}

impl BitCellMap {
    pub fn new(bit_cell: &Arc<Vec<u8>>) -> Self {
        let cells = bit_cell.len() as u8;
        if cells > _64BIT {
            panic!(
                "The length of bitCell must be less than or equal to {}!",
                _64BIT
            );
        }
        let mut bits_sum = 0;
        bit_cell.iter().for_each(|cell| {
            if *cell > _64BIT {
                panic!("The value for a bitCell cannot be greater than {}!", _64BIT);
            }
            bits_sum += cell;
            if bits_sum > _64BIT {
                panic!(
                    "The combined value for all bitCells cannot be greater than {}!",
                    _64BIT
                );
            }
        });

        let cell_shift = vec![0; cells as usize];
        let cell_mask = vec![0; cells as usize];
        let cell_zero = vec![0; cells as usize];

        BitCellMap {
            CELL: bit_cell.clone(),
            CELLSHIFT: cell_shift,
            CELLMASK: cell_mask,
            CELLZERO: cell_zero,
            Cells: cells,
        }
    }

    pub fn bits_at_cell(&self, cell: u8) -> u8 {
        if self.Cells <= cell {
            0
        } else {
            self.CELL[cell as usize]
        }
    }

    // used to calculate the size of combined bitmasks
    // for redefining bit cell mapping for planets, stars, etc.
    pub fn total_bits_at_cells(&self, cells: &[u8]) -> u8 {
        cells.iter().map(|&cell| self.bits_at_cell(cell)).sum()
    }

    pub fn bit_shift_cell(&mut self, cell: u8) -> u8 {
        if cell == 0 {
            return 0;
        }

        let mut shift = self.CELLSHIFT[cell as usize];
        if shift == 0 {
            let mut _c = cell;
            while _c != 0 {
                shift += self.CELL[_c as usize - 1];
                _c -= 1;
            }
            self.CELLSHIFT[cell as usize] = shift;
        }
        shift
    }

    pub fn and_mask_cell(&mut self, cell: u8) -> u64 {
        let mut mask = self.CELLMASK[cell as usize];
        if mask == 0 {
            mask = Self::solid_bit_mask(0, self.CELL[cell as usize] - 1);
            self.CELLMASK[cell as usize] = mask;
        }
        mask
    }

    pub fn zero_mask_cell(&mut self, cell: u8) -> u64 {
        let mut mask = self.CELLZERO[cell as usize];
        if mask == 0 {
            mask = !(self.and_mask_cell(cell) << self.bit_shift_cell(cell));
            self.CELLZERO[cell as usize] = mask;
        }
        mask
    }

    fn solid_bit_mask(first_on: u8, last_on: u8) -> u64 {
        if last_on > _64BIT_N1 || first_on > last_on {
            return 0;
        }
        let mut max = ONE << first_on;
        if last_on == first_on {
            return max;
        }
        let mut i = last_on;
        while i > first_on {
            max |= ONE << i;
            i -= 1;
        }
        max
    }
}
