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
use std::num::Wrapping;
use std::ops::Deref;
use num_traits::FromPrimitive;
use crate::math_trait::{Const, CONST};

pub struct FastPrng {
    lo: Wrapping<u64>,
    hi: Wrapping<u64>,
    prng_stack: Vec<PrngInst>,
    bud: Wrapping<u64>,
}

const SHIFT:u32 = 17;
impl FastPrng {
    pub fn new() -> Self {
        let c: &Const = CONST.deref();
        let b = c.bud.clone().deref().clone();
        FastPrng {
            lo: b,
            hi: !b,
            prng_stack: vec![],
            bud: b,
        }
    }
    
    pub fn seed(&mut self, seed: u64) {
        if 0 != seed {
            self.lo = Wrapping::from_u64(seed).unwrap();
            self.hi = Wrapping::from_u64(!seed).unwrap();
        }
        else {
            self.lo = self.bud;
            self.hi = !self.bud;
        }
    }
    
    pub fn push(&mut self, seed: u64) {
        self.prng_stack.push(PrngInst { lo: self.lo.0, hi: self.hi.0 });
        self.seed(seed);
    }
    
    pub fn pop(&mut self) {
        let inst= self.prng_stack.pop().unwrap();
        self.lo = Wrapping::from_u64(inst.lo).unwrap();
        self.hi = Wrapping::from_u64(inst.hi).unwrap();
    }
    
    pub fn clear_stack(&mut self) {
        self.prng_stack.clear();
    }
    
    pub fn u64(&mut self) -> u64 {
        self.hi = self.hi.rotate_left(SHIFT) + self.hi.rotate_right(SHIFT) + self.lo;
        self.lo += self.hi + self.bud;
        self.hi.0
    }
    
    pub fn u8(&mut self) -> u8 {
        let max = u8::MAX as f64;
        (self.f64() * max).round() as u8
    }
    
    ///  Random number from 0 to 1
    pub fn f64(&mut self) -> f64 {
        (self.u64() / 2) as f64 / (u64::MAX / 2) as f64
    }

    ///  Random number from 0 to 1
    pub fn f32(&mut self) -> f32 {
        self.f64() as f32
    }
}

struct PrngInst {
    lo: u64,
    hi: u64,
}