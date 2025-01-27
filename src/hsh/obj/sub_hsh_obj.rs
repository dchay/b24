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

use crate::hsh::id::num_id::INumID;
use crate::hsh::sub_hsh_alg::{IUseSubHshAlg, SubHshAlg};
use std::sync::{Arc, RwLock};
use crate::hsh::hash_trie::MAX_FACETS;

pub trait ISubHshObj<I>: IUseSubHshAlg
where
    I: INumID + Default,
{
    fn id(&self) -> &Arc<RwLock<I>>;
    fn is_same_by_id(&self, id: Arc<RwLock<I>>) -> bool{
        let write_id = id.read().unwrap();
        if !self.same_alg(write_id.alg()) {return false;}
        let r_self = self.id().read().unwrap();
        if r_self.has_facets() != write_id.has_facets() {return false;}
        if !r_self.has_facets() {
            write_id.raw_id() == r_self.raw_id()
        }
        else {
            let mut f: u8 = 0;
            'facet: while f <= MAX_FACETS {
                let s = r_self.raw_facet(f);
                if write_id.raw_facet(f) != s {return false;}
                if s == 0 { break 'facet; }
                f += 1;
            }
            if 0 < f {
                return true;
            }
            false
        }
    }
}

pub struct SubHshObj<I>
where
    I: INumID + Clone + Default,
{
    id: Arc<RwLock<I>>,
    alg: Arc<SubHshAlg>,
}

impl<I> SubHshObj<I>
where
    I: INumID + Clone + Default,
{
    pub fn new(raw_id: u64) -> Self {
        let id = Arc::new(RwLock::new(I::new_0_facet_id(raw_id)));
        let alg = id.read().unwrap().alg().clone();
        Self { id, alg }
    }
}

impl<I> IUseSubHshAlg for SubHshObj<I>
where
    I: INumID + Clone + Default,
{
    fn alg(&self) -> &Arc<SubHshAlg> {
        &self.alg
    }
}

impl<I> ISubHshObj<I> for SubHshObj<I>
where
    I: INumID + Clone + Default,
{
    fn id(&self) -> &Arc<RwLock<I>> {
        &self.id
    }
}