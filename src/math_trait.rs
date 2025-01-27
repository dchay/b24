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
#![allow(clippy::collection_is_never_read)]
#![allow(clippy::just_underscores_and_digits)]

use bigdecimal::BigDecimal as Dec;
use once_cell::sync::Lazy;
use std::{marker::PhantomData, ops::Deref, str::FromStr, sync::Arc};
use std::num::Wrapping;

const PI_50_DIGITS: &str = "3.14159265358979323846264338327950288419716939937510";

pub static PLACES: u32 = 40;

// Define the Constants struct
pub struct Const<'a> {
    pub n1: Arc<Dec>,
    pub _0: Arc<Dec>,
    pub _1: Arc<Dec>,
    pub _2: Arc<Dec>,
    pub pi: Arc<Dec>,
    pub _5: Arc<Dec>,
    pub epsilon: Arc<Dec>,
    pub bud: Arc<Wrapping<u64>>,
    _marker: PhantomData<&'a ()>,
    pub _3: Arc<Dec>,
    pub _4: Arc<Dec>,
    pub _6: Arc<Dec>,
    pub _7: Arc<Dec>,
    pub _8: Arc<Dec>,
    pub _9: Arc<Dec>,
    pub _a: Arc<Dec>,
    pub _b: Arc<Dec>,
}

impl<'a> Const<'a> {
    fn new() -> Self {
        let e_str = &format!("{}{}", "1e-", PLACES);
        Const {
            epsilon: Arc::new(Dec::from_str(e_str).unwrap()),
            n1: Arc::new(Dec::from(-1)),
            _0: Arc::new(Dec::from(0)),
            _1: Arc::new(Dec::from(1)),
            _2: Arc::new(Dec::from(2)),
            _3: Arc::new(Dec::from(3)),
            _4: Arc::new(Dec::from(4)),
            pi: Arc::new(Dec::from_str(PI_50_DIGITS).unwrap()),
            _5: Arc::new(Dec::from(5)),
            _6: Arc::new(Dec::from(6)),
            _7: Arc::new(Dec::from(7)),
            _8: Arc::new(Dec::from(8)),
            _9: Arc::new(Dec::from(9)),
            _a: Arc::new(Dec::from(10)),
            _b: Arc::new(Dec::from(11)),
            bud: Arc::new(Wrapping(10007)),//0x5555555555555555)),
            _marker: PhantomData,
        }
    }
}
pub static CONST: Lazy<Const<'static>> = Lazy::new(Const::new);

pub struct Func<'a> {
    pub n_p: Arc<Dec>,
    pub phi: Arc<Dec>,
    pub u0: Arc<Dec>,
    pub u1: Arc<Dec>,
    pub u2: Arc<Dec>,
    pub u3: Arc<Dec>,
    pub u4: Arc<Dec>,
    pub u5: Arc<Dec>,
    pub u6: Arc<Dec>,
    pub u7: Arc<Dec>,
    pub u8: Arc<Dec>,
    pub u9: Arc<Dec>,
    pub ua: Arc<Dec>,
    pub ub: Arc<Dec>,
    pub v0: Arc<Dec>,
    pub v1: Arc<Dec>,
    pub v2: Arc<Dec>,
    pub v3: Arc<Dec>,
    pub v4: Arc<Dec>,
    pub v5: Arc<Dec>,
    pub v6: Arc<Dec>,
    _marker: PhantomData<&'a ()>,
}
impl<'a> Func<'a> {
    fn new() -> Self {
        let c: &Const = CONST.deref();
        let _0: &Dec = &*c._0;
        let _1: &Dec = &*c._1;
        let _2: &Dec = &*c._2;
        let _3: &Dec = &*c._3;
        let _4: &Dec = &*c._4;
        let _5: &Dec = &*c._5;
        let _6: &Dec = &*c._6;
        let _7: &Dec = &*c._7;
        let _8: &Dec = &*c._8;
        let _9: &Dec = &*c._9;
        let _a: &Dec = &*c._a;
        let _b: &Dec = &*c._b;
        Func {
            n_p: Arc::new(-((_1 + _5.sqrt().unwrap()) / _2)),
            phi: Arc::new((_1 + _5.sqrt().unwrap()) / _2),
            u0: Arc::new(_0.clone()),
            u1: Arc::new(_1 / _b),
            u2: Arc::new(_2 / _b),
            u3: Arc::new(_3 / _b),
            u4: Arc::new(_4 / _b),
            u5: Arc::new(_5 / _b),
            u6: Arc::new(_6 / _b),
            u7: Arc::new(_7 / _b),
            u8: Arc::new(_8 / _b),
            u9: Arc::new(_9 / _b),
            ua: Arc::new(_a / _b),
            ub: Arc::new(_1.clone()),
            v0: Arc::new(_0.clone()),
            v1: Arc::new(_1 / _6),
            v2: Arc::new(_2 / _6),
            v3: Arc::new(_3 / _6),
            v4: Arc::new(_4 / _6),
            v5: Arc::new(_5 / _6),
            v6: Arc::new(_1.clone()),
            _marker: PhantomData,
        }
    }
}
pub static FUNC: Lazy<Func<'static>> = Lazy::new(Func::new);

#[allow(unused)]
pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
}

impl Trig for Dec {
    fn sin(self) -> Self {
        let c: &Const = CONST.deref();
        let _0: &Dec = &*c._0;
        let _1: &Dec = &*c._1;
        let _2: &Dec = &*c._2;
        let e: &Dec = &*c.epsilon;

        let s: Arc<Dec> = Arc::new(self);
        let mut result: Arc<Dec> = Arc::new(_0.clone());
        let mut term: Arc<Dec> = s.clone();
        let mut n: Arc<Dec> = Arc::new(_1.clone());

        while term.abs() > *e {
            result = Arc::new((&*result + &*term).clone());
            n = Arc::new((&*n + _2).clone());
            term = Arc::new((-&*term * &*s * &*s / (&*n * (&*n - _1))).clone());
        }
        (*result).clone()
    }

    fn cos(self) -> Self {
        let c: &Const = CONST.deref();
        let _0: &Dec = &*c._0;
        let _1: &Dec = &*c._1;
        let _2: &Dec = &*c._2;
        let e: &Dec = &*c.epsilon;

        let s: Arc<Dec> = Arc::new(self);
        let mut result: Arc<Dec> = Arc::new(_1.clone());
        let mut term: Arc<Dec> = Arc::new(_1.clone());
        let mut n: Arc<Dec> = Arc::new(_0.clone());

        while term.abs() > *e {
            n = Arc::new((&*n + _2).clone());
            term = Arc::new((-&*term * &*s * &*s / (&*n * (&*n - _1))).clone());
            result = Arc::new((&*result + &*term).clone());
        }
        (*result).clone()
    }

    fn acos(self) -> Self {
        let c: &Const = CONST.deref();
        let _0: &Dec = &*c._0;
        let _1: &Dec = &*c._1;
        let _2: &Dec = &*c._2;
        let pi: &Dec = &*c.pi;
        let e: &Dec = &*c.epsilon;

        let s: Arc<Dec> = Arc::new(self);
        let mut term: Arc<Dec> = s.clone();
        let mut sum: Arc<Dec> = Arc::new((pi / _2.clone() - &*s).clone());
        let mut n: Arc<Dec> = Arc::new(_1.clone());
        let x_squared: Arc<Dec> = Arc::new((&*s * &*s).clone());

        while term.abs() > *e {
            term = Arc::new(
                (&*term
                    * &*x_squared
                    * (&*n * _2.clone() - _1.clone())
                    * (&*n * _2.clone() - _1.clone())
                    / ((&*n * _2.clone()) * (&*n * _2.clone() + _1.clone())))
                .clone(),
            );
            sum = Arc::new((&*sum - &*term).clone());
            n = Arc::new((&*n + _1.clone()).clone());
        }

        (*sum).clone()
    }
}
