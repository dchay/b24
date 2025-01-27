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

use crate::hsh::bit_cell_map::BitCellMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

/// Traits for N dimensional hash IDs
pub trait IUseSubHshAlg {
    fn same_alg(&self, alg: &Arc<SubHshAlg>) -> bool{
        self.alg().same_alg(alg)
    }

    fn alg(&self) -> &Arc<SubHshAlg>;
}

const MAX_BIT: u8 = 10;

/// Traits for a hash-based "type" system with Hash Tree indexing
pub trait ISubHshAlg {
    /// the size of subdirectories
    fn dir_size(&self) -> u8;
    /// last index of a subdirectory
    fn dir_last(&self) -> u8;
    fn id(&self) -> i64;
    fn get_cell_map(&self) -> &Arc<BitCellMap>;
    fn sub_hsh(&self, raw_id: u64, depth: u8) -> u8;
}

/// <summary>
/// The basic algorithm for Hash Tree indexing.  
/// Each Hash Tree Node has a data object and
/// a directory of children that is bits squared in size.
/// </summary>
pub struct SubHshAlg {
    bits: u8,
    size: u8,
    last: u8,
    id: i64,
    cell_map: Arc<BitCellMap>,
}

impl SubHshAlg {
    pub fn new(sub_hsh_bits: u8, map: &Arc<BitCellMap>) -> Self {
        if sub_hsh_bits > MAX_BIT {
            panic!(
                "SubHshAlg cannot be configured with more than {} bits!",
                MAX_BIT
            );
        }
        let bits = sub_hsh_bits;
        let size = 1 << bits;
        let last = size - 1;
        let id = Self::id_tally();
        let cell_map = map.clone();

        Self {
            bits,
            size,
            last,
            id,
            cell_map,
        }
    }

    fn id_tally() -> i64 {
        static ID_TALLY: AtomicI64 = AtomicI64::new(0);
        // remember this is a post increment that returns current value before incrementing like ID_TALLY++ in c++
        // so... to use this the way we did in C# we do a +1
        ID_TALLY.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn same_alg(&self, alg: &Arc<SubHshAlg>) -> bool{
        alg.id == self.id
    }
}

impl ISubHshAlg for SubHshAlg {
    fn dir_size(&self) -> u8 {
        self.size
    }

    fn dir_last(&self) -> u8 {
        self.last
    }

    /// <summary>
    /// Every algorithm will have its own id.<para/>
    /// Grouped algorithms can be contained in a master algorithm.<para/>
    /// Only group algorithms that are guaranteed to never collide;<para/>
    /// e.g. We need 128 bits for info, one alg covers the first 64 bits<para/>
    /// while the other alg covers the last 64 bits.<para/>
    /// <see cref="str.StringAlg"/>str::StrAlg
    /// implements facets to group 256 bits.<para/>
    /// <see cref="source.Source{O, OI}"/>
    /// Consider implementing Isomech PolyID to group arbitrary bits
    /// like in Isomech source.Source and the powerful VennID.
    /// </summary>
    fn id(&self) -> i64 {
        self.id
    }

    fn get_cell_map(&self) -> &Arc<BitCellMap> {
        &self.cell_map
    }

    fn sub_hsh(&self, raw_id: u64, depth: u8) -> u8 {
        self.last & ((raw_id >> (self.bits * depth)) as u8)
    }
}
