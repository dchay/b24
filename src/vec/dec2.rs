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

#[allow(unused)]
use crate::math_trait::Trig;
use crate::math_trait::CONST;
#[allow(unused)]
use crate::vec::dec_vec::DecVec;
use bigdecimal::BigDecimal as Dec;
use std::ops::Deref;
use std::{
    fmt::{Display, Formatter},
    hash::Hash,
    ops::{Add, Div, Sub},
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct Dec2 {
    x: Arc<Dec>,
    y: Arc<Dec>,
}

impl Dec2 {
    pub fn x(&self) -> &Dec {
        &*self.x
    }
    pub fn y(&self) -> &Dec {
        &*self.y
    }

    pub fn new(x: Arc<Dec>, y: Arc<Dec>) -> Self {
        Self {
            x: x.clone(),
            y: y.clone(),
        }
    }
}

impl DecVec for Dec2 {
    fn sqr_len(&self) -> Dec {
        let x_squared: Dec = self.x() * self.x();
        let y_squared: Dec = self.y() * self.y();
        x_squared + y_squared
    }

    fn length(&self) -> Dec {
        self.sqr_len().sqrt().unwrap_or_default()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: Arc::from(self.x() / &length),
            y: Arc::from(self.y() / &length),
        }
    }

    fn sqr_dist(&self, other: &Self) -> Dec {
        let dx: &Dec = &(self.x() - other.x());
        let dy: &Dec = &(self.y() - other.y());
        dx * dx + dy * dy
    }

    fn distance(&self, other: &Self) -> Dec {
        self.sqr_dist(other).sqrt().unwrap_or_default()
    }

    fn cross2d(&self, other: &Self) -> Dec {
        self.x() * other.y() - self.y() * other.x()
    }

    /** Using cross2d value for y */
    fn cross3d(&self, other: &Self) -> Self {
        let x = CONST.deref()._0.clone();
        // let y = _0;
        let z = Arc::from(self.cross2d(other));
        Self::new(x, z)
    }

    fn dot(&self, other: &Self) -> Dec {
        self.x() * other.x() + self.y() * other.y()
    }

    fn scale(&self, scalar: &Dec) -> Self {
        Self {
            x: Arc::from(self.x() * scalar),
            y: Arc::from(self.y() * scalar),
        }
    }
}

impl Display for Dec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Dec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: Arc::from(self.x() + other.x()),
            y: Arc::from(self.y() + other.y()),
        }
    }
}

impl Sub for Dec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: Arc::from(self.x() - other.x()),
            y: Arc::from(self.y() - other.y()),
        }
    }
}

impl Div<Dec> for Dec2 {
    type Output = Self;

    fn div(self, scalar: Dec) -> Self::Output {
        Self {
            x: Arc::from(self.x() / &scalar),
            y: Arc::from(self.y() / &scalar),
        }
    }
}
