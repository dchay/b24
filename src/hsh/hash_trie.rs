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

use crate::{
    hsh::{
        id::num_id::INumID,
        obj::sub_hsh_obj::ISubHshObj,
        sub_hsh_alg::IUseSubHshAlg
    }
};
use std::sync::{Arc, RwLock};

pub const MAX_FACETS: u8 = 16;

/// stores objects with rawid NOT zero<br>
/// (the id of the Universe singleton)
pub trait IHashTrie<O,I>: IUseSubHshAlg
where O: ISubHshObj<I>,
      I: INumID + Default
{
    fn faceted(&self) -> bool;
    fn size(&self) -> u64;
    
    /// Will store any object who's rawid is NOT zero!<br>
    /// Use Replace false for most cases<br>
    /// Returns:<br>
    /// Some(true) if obj is stored<br>
    /// Some(false) if obj is already there<br>
    /// None if obj is replaced<br>
    fn insert(&self, obj: Arc<RwLock<O>>, replace: bool) -> Option<bool>;
    fn add(&self, obj: Arc<RwLock<O>>) -> Option<bool>;
    
    /// Will find any stored object who's rawid is NOT zero!
    fn get(&self, id: Arc<RwLock<I>>) -> Option<Arc<RwLock<O>>>;
    
    /// returns false if not found
    fn remove(&self, id: Arc<RwLock<I>>) -> bool;
    
    fn dispose(&self);
}
