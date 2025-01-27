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

use bigdecimal::BigDecimal as Dec;
use crate::math_trait::Trig;

#[allow(unused)]
pub trait DecVec {
    fn sqr_len(&self) -> Dec;
    fn length(&self) -> Dec;
    fn normalize(&self) -> Self;
    fn sqr_dist(&self, other: &Self) -> Dec;
    fn distance(&self, other: &Self) -> Dec;
    fn angle(&self, other: &Self) -> Dec {
        // determine the smallest angle between two 3D vectors
        let dot_product: Dec = self.dot(other);
        let lengths_product: Dec = self.length() * other.length();
        (dot_product / lengths_product).acos()
    }

    /** Always returns the cross of (x,y) */
    fn cross2d(&self, other: &Self) -> Dec;
    fn cross3d(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> Dec;
    fn scale(&self, scalar: &Dec) -> Self;
}
