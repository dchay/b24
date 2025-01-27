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

use crate::vec::dec2::Dec2;
use crate::vec::dec3::Dec3;
/// U   0                     1
/// V   0 1 2 3 4 5 6 7 8 9 A B   V
/// 1 6   Ỏ   Ỏ   Ỏ   Ỏ   Ỏ     6 1
///   5  /A\ /A\ /A\ /A\ /A\    5
///   4 Ö---O---O---O---O---Ö   4
///   3  \V/A\V/A\V/A\V/A\V/A\  3
///   2   Ȯ---O---O---O---O---Ȯ 2
///   1    \V/ \V/ \V/ \V/ \V/  1
/// 0 0     Ǫ   Ǫ   Ǫ   Ǫ   Ǫ   0 0
///     0 1 2 3 4 5 6 7 8 9 A B
/// U  0                      1

#[allow(dead_code)]
pub struct VertData {
    uv_land: Dec2,
    uv_biome: Dec2,
    loc_map: Dec2,
    loc_uni_globe: Dec3,
    point: bool,
    pent: bool,
    neighbors: [Dec3; 6], // even pentagons have 6 (2 are split UVS)
}
