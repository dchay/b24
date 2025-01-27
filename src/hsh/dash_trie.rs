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
    hash_trie::IHashTrie,
    id::num_id::INumID,
    obj::sub_hsh_obj::ISubHshObj,
    sub_hsh_alg::{IUseSubHshAlg, SubHshAlg}
};
use dashmap::DashMap;
use std::{
    hash::Hash,
    marker::PhantomData,
    sync::{
        Arc, RwLock
    }
};

/// A simple unfaceted hash trie using a DashMap
pub struct DashTrie<O,I>
where
    O: ISubHshObj<I>,
    I: INumID + Default + Eq + Hash,
{
    map: DashMap<u64, Arc<RwLock<O>>>,
    sub_hsh_alg: Arc<SubHshAlg>,
    _marker: PhantomData<I>,
}

impl<O, I> IUseSubHshAlg for DashTrie<O, I>
where
    I: Default + Eq + Hash + INumID,
    O: ISubHshObj<I>,
{
    fn alg(&self) -> &Arc<SubHshAlg> {
        &self.sub_hsh_alg
    }
}

impl<O, I> IHashTrie<O, I> for DashTrie<O, I>
where
    O: ISubHshObj<I>,
    I: INumID + Default + Eq + Hash,
{
    fn faceted(&self) -> bool {
        false
    }

    fn size(&self) -> u64 {
        self.map.len() as u64
    }

    fn insert(&self, obj: Arc<RwLock<O>>, replace: bool)-> Option<bool> {
        let id = obj.read().unwrap().id().read().unwrap().raw_id();
        if !replace && self.map.contains_key(&id) {
            return Some(false);
        }
        self.map.insert(id, obj);
        Some(true)
    }

    fn add(&self, obj: Arc<RwLock<O>>) -> Option<bool> {
        self.insert(obj, false)
    }

    fn get(&self, id: Arc<RwLock<I>>) -> Option<Arc<RwLock<O>>> {
        let out = self.map.get(&id.read().unwrap().raw_id());
        if out.is_some() { 
            Some(out.unwrap().value().clone())
        } else {
            None
        }
    }

    fn remove(&self, id: Arc<RwLock<I>>) -> bool {
        let ret = self.map.remove(&id.read().unwrap().raw_id());
        if ret.is_some() {
            true
        } else {
            false
        }
    }

    fn dispose(&self) {
        self.map.clear();
    }
}

impl<'a, O, I> DashTrie<O, I>
where
    O: ISubHshObj<I>,
    I: INumID + Default + Eq + Hash,
{
    pub fn new() -> Self {
        DashTrie {
            map: DashMap::new(),
            sub_hsh_alg: I::new_0_facet_id(0).alg().clone(),
            _marker: Default::default(),
        }
    }
}
