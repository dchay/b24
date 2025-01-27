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

use crate::prng::fast_prng::FastPrng;
use crate::hsh::{
    bit_cell_map::BitCellMap,
    sub_hsh_alg::ISubHshAlg,
    sub_hsh_alg::SubHshAlg
};
use num_traits::FromPrimitive;
use once_cell::sync::Lazy;
use std::{
    num::Wrapping,
    ops::Deref,
    sync::Arc
};

static STR_VEC: Lazy<Arc<Vec<u8>>> = Lazy::new(|| Arc::new(vec![64u8]));
static STR_MAP: Lazy<Arc<BitCellMap>> = Lazy::new(|| Arc::new(BitCellMap::new(&STR_VEC.deref())));
pub(super) static STR_SUB_ALG: Lazy<Arc<SubHshAlg>> =
    Lazy::new(|| Arc::new(SubHshAlg::new(4, &STR_MAP.deref())));
static STR_ALG: Lazy<Arc<StrAlg>> = Lazy::new(|| Arc::new(StrAlg::singleton()));

pub trait IStrAlg: ISubHshAlg {
    fn make_id(str: &str, facet: u8) -> u64;
}

/// Collision generates up to 8 facets of a 512-bit hash.
/// Most will only need the first 64 bits.
/// Testing has shown use of the first 3 facets for millions of ids. 
pub struct StrAlg;

impl StrAlg {
    fn singleton() -> Self {
        StrAlg {}
    }
    pub fn inst() -> &'static Arc<StrAlg> {
        STR_ALG.deref()
    }

    //noinspection t
    fn base_id(str: &str, bkd: bool, inv: bool, half: i64, bkbit: bool) -> u64 {
        const _7B: u64 = 127;
        let len = str.len() as i64;
        let end = half;//if bkd { 0 } else { len - 1 };
        let mut prng = FastPrng::new();
        let mut c = if bkd { len } else { -1 };
        let mut id: Wrapping<u64> = Wrapping::from_u64(0).unwrap();
        let chars: Vec<char> = str.chars().collect();

        prng.push(0);

        loop {
            
            let mult =
            if bkd {
                c -= 1;
                len - c
            } else {
                c += 1;
                c + 1
            };
            
            let ch =
                if bkbit { (chars[c as usize] as u64).reverse_bits() }
                else { chars[c as usize] as u64 };
            
            if inv {
                id += !ch;
            } else {
                id += ch;
            }

            id *= _7B * mult as u64;

            if c & 3 == 0 {
                prng.seed(id.0);
            }
            
            id += prng.u64();

            if c == end {
                break;
            }
        }

        prng.pop();
        
        id.0
    }
}

impl ISubHshAlg for StrAlg {
    fn dir_size(&self) -> u8 {
        STR_SUB_ALG.deref().dir_size()
    }

    fn dir_last(&self) -> u8 {
        STR_SUB_ALG.deref().dir_last()
    }

    fn id(&self) -> i64 {
        STR_SUB_ALG.deref().id()
    }

    fn get_cell_map(&self) -> &Arc<BitCellMap> {
        STR_MAP.deref()
    }

    fn sub_hsh(&self, raw_id: u64, depth: u8) -> u8 {
        STR_SUB_ALG.deref().sub_hsh(raw_id, depth)
    }
}

impl IStrAlg for StrAlg {
    fn make_id(str: &str, facet: u8) -> u64 {
        let len = str.len();
        if len == 0 {
            return 0;
        }
        let half: i64 = (len / 2) as i64;

        match facet {
            0 => Self::base_id(str, true, false, half, false),
            1 => Self::base_id(str, false, false, half, false),
            2 => Self::base_id(str, true, true, half, false),
            3 => Self::base_id(str, false, true, half, false),
            4 => Self::base_id(str, true, false, half, true),
            5 => Self::base_id(str, false, false, half, true),
            6 => Self::base_id(str, true, true, half, true),
            7 => Self::base_id(str, false, true, half, true),
            _ => 0,
        }
    }
}
