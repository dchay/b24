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

use std::{
    ops::Deref,
    sync::Arc
};
use std::sync::RwLock;
use crate::{
    hsh::{
        obj::sub_hsh_obj::ISubHshObj,
        str::str_id::IStrID,
        str::str_alg::STR_SUB_ALG,
        sub_hsh_alg::{IUseSubHshAlg, SubHshAlg}
    }
};

pub trait IStrObj<I>: ISubHshObj<I>
where I: IStrID + Default{}

pub struct StrObj<I>
where
    I: IStrID + Clone + Default,
{
    str_id: Arc<RwLock<I>>,
}

impl<I> StrObj<I>
where
    I: IStrID + Clone + Default,
{
    pub fn new(str: &'static str) -> Self {
        let id = Arc::new(RwLock::new(I::new_str_id(str)));
        Self { str_id: id }
    }
}

impl<I> ISubHshObj<I> for StrObj<I>
where
    I: IStrID + Clone + Default,
{
    fn id(&self) -> &Arc<RwLock<I>> {
        &self.str_id
    }
}

impl<I> IUseSubHshAlg for StrObj<I>
where
    I: IStrID + Clone + Default,
{
    fn alg(&self) -> &Arc<SubHshAlg> {
        STR_SUB_ALG.deref()
    }
}

impl<'a, I> IStrObj<I> for StrObj<I>
where I: IStrID + Clone + Default, {}