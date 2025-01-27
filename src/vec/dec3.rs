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

use crate::vec::dec_vec::DecVec;
use bigdecimal::BigDecimal as Dec;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct Dec3 {
    x: Arc<Dec>,
    y: Arc<Dec>,
    z: Arc<Dec>,
}

impl Dec3 {
    pub fn x(&self) -> &Dec {
        &*self.x
    }
    pub fn y(&self) -> &Dec {
        &*self.y
    }
    pub fn z(&self) -> &Dec {
        &*self.z
    }

    pub fn new(x: Arc<Dec>, y: Arc<Dec>, z: Arc<Dec>) -> Self {
        Self {
            x: x.clone(),
            y: y.clone(),
            z: z.clone(),
        }
    }
}

impl DecVec for Dec3 {
    fn sqr_len(&self) -> Dec {
        let x_squared: Dec = self.x() * self.x();
        let y_squared: Dec = self.y() * self.y();
        let z_squared: Dec = self.z() * self.z();
        x_squared + y_squared + z_squared
    }

    fn length(&self) -> Dec {
        self.sqr_len().sqrt().unwrap_or_default()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: Arc::from(self.x() / &length),
            y: Arc::from(self.y() / &length),
            z: Arc::from(self.z() / &length),
        }
    }

    fn sqr_dist(&self, other: &Self) -> Dec {
        let dx: &Dec = &(self.x() - other.x());
        let dy: &Dec = &(self.y() - other.y());
        let dz: &Dec = &(self.z() - other.z());
        dx * dx + dy * dy + dz * dz
    }

    fn distance(&self, other: &Self) -> Dec {
        self.sqr_dist(other).sqrt().unwrap_or_default()
    }

    fn cross2d(&self, other: &Self) -> Dec {
        self.x() * other.y() - self.y() * other.x()
    }

    fn cross3d(&self, other: &Self) -> Self {
        let x = Arc::from(self.y() * other.z() - self.z() * other.y());
        let y = Arc::from(self.z() * other.x() - self.x() * other.z());
        let z = Arc::from(self.cross2d(other));
        Self::new(x, y, z)
    }

    fn dot(&self, other: &Self) -> Dec {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn scale(&self, scalar: &Dec) -> Self {
        Self {
            x: Arc::from(self.x() * scalar),
            y: Arc::from(self.y() * scalar),
            z: Arc::from(self.z() * scalar),
        }
    }
}
