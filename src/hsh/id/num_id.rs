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

use crate::hsh::{
    bit_cell_map::BitCellMap,
    sub_hsh_alg::{IUseSubHshAlg, SubHshAlg}
};
use once_cell::sync::Lazy;
use std::{
    clone::Clone,
    ops::Deref,
    sync::Arc,
    any::Any
};
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

static NUM_VEC: Lazy<Arc<Vec<u8>>> = Lazy::new(|| Arc::new(vec![64u8]));
static NUM_MAP: Lazy<Arc<BitCellMap>> = Lazy::new(|| Arc::new(BitCellMap::new(NUM_VEC.deref())));
static NUM_ALG: Lazy<Arc<SubHshAlg>> = Lazy::new(|| Arc::new(SubHshAlg::new(4, NUM_MAP.deref())));

/// Traits for a simple single u64 hash id
pub trait INumID: IUseSubHshAlg {
    fn as_any(&self) -> &(dyn Any);
    fn raw_id(&self) -> u64;
    /// &mut self because facets lazy load.
    fn raw_facet(&self, facet: u8) -> u64;
    fn same_id(&self, id: &Arc<RwLock<dyn INumID>>) -> bool;
    fn has_facets(&self) -> bool;
    fn new_0_facet_id(raw: u64) -> Self
    where Self: Sized;
}

/// A simple single u64 hash id
pub struct NumID {
    raw: u64,
    alg: Arc<SubHshAlg>,
}

impl NumID {
    pub fn new(raw_id: u64) -> Self {
        NumID {
            raw: raw_id,
            alg: NUM_ALG.deref().clone(),
        }
    }
}

impl PartialEq for NumID {
    fn eq(&self, other: &Self) -> bool {
        self.raw_id() == other.raw_id()
    }
}

impl Eq for NumID {}

impl Hash for NumID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use only the raw_id for hashing, ignoring raw_facet
        self.raw_id().hash(state);
    }
}

impl std::fmt::Debug for NumID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NumID")
            .field("raw", &self.raw)
            .finish()
    }
}

impl Clone for NumID {
    fn clone(&self) -> Self {
        Self::new_0_facet_id(self.raw_id())
    }
}

impl Default for NumID {
    fn default() -> Self {
        Self::new_0_facet_id(0)
    }
}

impl IUseSubHshAlg for NumID {
    fn alg(&self) -> &Arc<SubHshAlg> {
        &self.alg
    }
}

impl INumID for NumID {
    fn as_any(&self) -> &(dyn Any) { self }

    fn raw_id(&self) -> u64 {
        self.raw
    }

    fn raw_facet(&self, facet: u8) -> u64 {
        if 0 < facet { 0 } else { self.raw_id() }
    }

    fn same_id(&self, id: &Arc<RwLock<dyn INumID>>) -> bool {
        let read_id = id.read().unwrap();
        if read_id.raw_id() == self.raw
            && read_id.same_alg(&self.alg) {
            return true;
        }
        false
    }
    fn has_facets(&self) -> bool { false }

    fn new_0_facet_id(raw: u64) -> Self {
        NumID {
            raw,
            alg: NUM_ALG.deref().clone(),
        }
    }
}

pub trait IRawID: INumID {
    fn set_rawid(&mut self, rawid: u64);
}

pub struct RawID {
    num_id: NumID,
}

impl INumID for RawID {
    fn as_any(&self) -> &(dyn Any) { self }
    fn raw_id(&self) -> u64 {
        self.num_id.raw
    }

    fn raw_facet(&self, facet: u8) -> u64 {
        self.num_id.raw_facet(facet)
    }

    fn same_id(&self, id: &Arc<RwLock<dyn INumID>>) -> bool {
        self.num_id.same_id(id)
    }

    fn has_facets(&self) -> bool {
        self.num_id.has_facets()
    }

    fn new_0_facet_id(raw: u64) -> Self {
        RawID {
            num_id: NumID::new_0_facet_id(raw),
        }
    }
}

impl Clone for RawID {
    fn clone(&self) -> Self {
        Self::new_0_facet_id(self.raw_id())
    }
}

impl Default for RawID {
    fn default() -> Self {
        Self::new_0_facet_id(0)
    }
}

impl IUseSubHshAlg for RawID {
    fn alg(&self) -> &Arc<SubHshAlg> {
        &self.num_id.alg
    }
}

impl IRawID for RawID {
    fn set_rawid(&mut self, raw_id: u64) {
        self.num_id.raw = raw_id;
    }
}

impl RawID {
    fn new(nid: &Arc<dyn INumID>) -> Self {
        RawID {
            num_id: NumID::new(nid.raw_id()),
        }
    }
}
