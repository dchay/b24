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
    id::num_id::INumID,
    str::str_alg::{
        STR_SUB_ALG,
        IStrAlg,
        StrAlg
    },
    sub_hsh_alg::{
        ISubHshAlg,
        IUseSubHshAlg,
        SubHshAlg
    }
};
use std::{
    any::Any,
    ops::Deref,
    sync::{Arc, RwLock}
};

pub trait IStrID: INumID {
    fn get_str(&self) -> &str;
    fn new_str_id(str: &'static str) -> Self
    where
        Self: Sized
    ;
}

/// example of a non-numeric ID
pub struct StrID {
    sid: Arc<RwLock<[u64; 4]>>,
    _str: &'static str,
}

const SID_END: u8 = 3;

impl StrID {
    pub fn get_string_alg() -> &'static Arc<StrAlg> {
        StrAlg::inst()
    }
    
    pub fn new(str: &'static str) -> Self {
        StrID {
            _str: str,
            sid: Arc::new(RwLock::new([0, 0, 0, 0])),
        }
    }
}


impl Clone for StrID {
    fn clone(&self) -> Self {
        let s = self._str;
        let mut id = Self::new_str_id(s);
        id.sid = self.sid.clone();
        id
    }
}

impl Default for StrID {
    fn default() -> Self {
        Self::new_str_id("")
    }
}

impl IStrID for StrID {
    fn get_str(&self) -> &str {
        &self._str
    }

    fn new_str_id(str: &'static str) -> Self
    where
        Self: Sized
    {
        StrID::new(str)
    }
}

impl INumID for StrID {
    fn as_any(&self) -> &(dyn Any) { self }
    fn raw_id(& self) -> u64 {
        self.sid.read().unwrap().deref()[0]
    }
    fn raw_facet(&self, facet: u8) -> u64 {
        if facet > SID_END {
            return 0;
        }
        let mut binding = self.sid.write().unwrap();
        let sid = binding.as_mut();
        if facet >= sid.len() as u8 {
            return 0;
        }
        if sid[facet as usize] == 0 {
            sid[facet as usize] = StrAlg::make_id(&self._str, facet);
        }
        sid[facet as usize]
    }

    fn same_id<'a>(&self, id: &Arc<RwLock<(dyn INumID + 'a)>>) -> bool {
        let read_id = id.read().unwrap();
        if read_id.alg().id() == self.alg().id() {
            let string_id = read_id.as_any().downcast_ref::<StrID>();
            if string_id.is_some(){
                return string_id.unwrap().get_str() == self._str;
            }
        }
        false
    }

    fn has_facets(&self) -> bool { true }

    fn new_0_facet_id(_raw: u64) -> Self
    where
        Self: Sized
    {
        unimplemented!("Can only make a StrID with a str.")
    }
}

impl IUseSubHshAlg for StrID {
    fn alg(&self) -> &Arc<SubHshAlg> {
        STR_SUB_ALG.deref()
    }
}
