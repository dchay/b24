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
#![allow(non_upper_case_globals)]
// using n as shorthand for negative

use crate::vec::dec2::Dec2;
use crate::{
    math_trait::{Const, Func, CONST, FUNC},
    vec::dec3::Dec3,
    vec::dec_vec::DecVec,
};

use bigdecimal::BigDecimal as Dec;
use num_traits::ToPrimitive;
use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};
// N Pole
const NP_n1_P_0: usize = 0;

// Arctic Circle
const N0__1_P_0: usize = 1;
const N2__0_1nP: usize = 2;
const N4_nP_0n1: usize = 3;
const N6_nP_0_1: usize = 4;
const N8__0_1_P: usize = 5;

// Antarctic Circle
const S1__P_0n1: usize = 6;
const S3__0n1nP: usize = 7;
const S5_n1nP_0: usize = 8;
const S7__0n1_P: usize = 9;
const S9__P_0_1: usize = 10;

// S Pole
const SP__1nP_0: usize = 11;


pub struct IcoBase {
    vert_dec: Arc<RwLock<Vec<Arc<Dec3>>>>,
    norm_dec: Arc<RwLock<Vec<Arc<Dec3>>>>,
    uvs_dec: Arc<RwLock<Vec<Arc<Dec2>>>>,
    vertices: Arc<RwLock<Vec<[f32; 3]>>>,
    normals: Arc<RwLock<Vec<[f32; 3]>>>,
    indices: Arc<RwLock<Vec<u32>>>,
    uvs: Arc<RwLock<Vec<[f32; 2]>>>,
}

impl IcoBase {
    pub fn new() -> Self {
        Self {
            vert_dec: Arc::new(RwLock::new(Vec::new())),
            norm_dec: Arc::new(RwLock::new(Vec::new())),
            uvs_dec: Arc::new(RwLock::new(Vec::new())),
            vertices: Arc::new(RwLock::new(Vec::new())),
            normals: Arc::new(RwLock::new(Vec::new())),
            indices: Arc::new(RwLock::new(Vec::new())),
            uvs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn dec_verts(&self) -> Arc<RwLock<Vec<Arc<Dec3>>>> {
        let mut vert_dec = self.vert_dec.write().unwrap();
        if vert_dec.is_empty() {
            let default = Arc::new(Dec3::default());
            vert_dec.resize(12, default.clone());
            let c: &Const = CONST.deref();
            let f: &Func = FUNC.deref();

            // N Pole
            vert_dec[NP_n1_P_0] = Arc::new(Dec3::new(c.n1.clone(), f.phi.clone(), c._0.clone()));

            // Arctic Circle
            vert_dec[N0__1_P_0] = Arc::new(Dec3::new(c._1.clone(), f.phi.clone(), c._0.clone()));
            vert_dec[N2__0_1nP] = Arc::new(Dec3::new(c._0.clone(), c._1.clone(), f.n_p.clone()));
            vert_dec[N4_nP_0n1] = Arc::new(Dec3::new(f.n_p.clone(), c._0.clone(), c.n1.clone()));
            vert_dec[N6_nP_0_1] = Arc::new(Dec3::new(f.n_p.clone(), c._0.clone(), c._1.clone()));
            vert_dec[N8__0_1_P] = Arc::new(Dec3::new(c._0.clone(), c._1.clone(), f.phi.clone()));

            // Antarctic Circle
            vert_dec[S1__P_0n1] = Arc::new(Dec3::new(f.phi.clone(), c._0.clone(), c.n1.clone()));
            vert_dec[S3__0n1nP] = Arc::new(Dec3::new(c._0.clone(), c.n1.clone(), f.n_p.clone()));
            vert_dec[S5_n1nP_0] = Arc::new(Dec3::new(c.n1.clone(), f.n_p.clone(), c._0.clone()));
            vert_dec[S7__0n1_P] = Arc::new(Dec3::new(c._0.clone(), c.n1.clone(), f.phi.clone()));
            vert_dec[S9__P_0_1] = Arc::new(Dec3::new(f.phi.clone(), c._0.clone(), c._1.clone()));

            // S Pole
            vert_dec[SP__1nP_0] = Arc::new(Dec3::new(c._1.clone(), f.n_p.clone(), c._0.clone()));
        }
        Arc::clone(&self.vert_dec)
    }

    pub fn dec_norms(&self) -> Arc<RwLock<Vec<Arc<Dec3>>>> {
        let mut norm_dec = self.norm_dec.write().unwrap();
        if norm_dec.is_empty() {
            let default = Arc::new(Dec3::default());
            norm_dec.resize(12, default.clone());
            let vert_dec = self.dec_verts();
            let mut populate_norm_dec = |index: usize| {
                norm_dec[index] = Arc::new(vert_dec.read().unwrap()[index].normalize());
            };
            // N Pole
            populate_norm_dec(NP_n1_P_0);

            // Arctic Circle
            populate_norm_dec(N0__1_P_0);
            populate_norm_dec(N2__0_1nP);
            populate_norm_dec(N4_nP_0n1);
            populate_norm_dec(N6_nP_0_1);
            populate_norm_dec(N8__0_1_P);

            // Antarctic Circle
            populate_norm_dec(S1__P_0n1);
            populate_norm_dec(S3__0n1nP);
            populate_norm_dec(S5_n1nP_0);
            populate_norm_dec(S7__0n1_P);
            populate_norm_dec(S9__P_0_1);

            // S Pole
            populate_norm_dec(SP__1nP_0);
        }
        Arc::clone(&self.norm_dec)
    }

    pub fn dec_uvs(&self) -> Arc<RwLock<Vec<Arc<Dec2>>>> {
        let mut uvs_dec = self.uvs_dec.write().unwrap();
        if uvs_dec.is_empty() {
            let default = Arc::new(Dec2::default());
            uvs_dec.resize(12, default.clone());
            
            let mut populate_uv = |index: usize, u: Arc<Dec>, v: Arc<Dec>| {
              uvs_dec[index] = Arc::new(Dec2::new(u, v));  
            };
            
            let f: &Func = FUNC.deref();

            // N Pole, for now it squashes to a point
            populate_uv(NP_n1_P_0, f.u5.clone(), f.v6.clone());

            // Arctic Circle
            populate_uv(N0__1_P_0, f.u0.clone(), f.v4.clone());
            populate_uv(N2__0_1nP, f.u2.clone(), f.v4.clone());
            populate_uv(N4_nP_0n1, f.u4.clone(), f.v4.clone());
            populate_uv(N6_nP_0_1, f.u6.clone(), f.v4.clone());
            populate_uv(N8__0_1_P, f.u8.clone(), f.v4.clone());

            // Antarctic Circle
            populate_uv(S1__P_0n1, f.u6.clone(), f.v2.clone());
            populate_uv(S3__0n1nP, f.u6.clone(), f.v2.clone());
            populate_uv(S5_n1nP_0, f.u6.clone(), f.v2.clone());
            populate_uv(S7__0n1_P, f.u6.clone(), f.v2.clone());
            populate_uv(S9__P_0_1, f.u6.clone(), f.v2.clone());

            // S Pole, for now it squashes to a point
            populate_uv(SP__1nP_0, f.u5.clone(), f.v0.clone());
        }
        Arc::clone(&self.uvs_dec)
    }

    fn d3_to_f3(dec3d: Arc<Dec3>) -> [f32; 3] {
        let x = dec3d.x().to_f32().unwrap_or_default();
        let y = dec3d.y().to_f32().unwrap_or_default();
        let z = dec3d.z().to_f32().unwrap_or_default();
        [x, y, z]
    }

    fn d2_to_f2(dec3d: Arc<Dec2>) -> [f32; 2] {
        let x = dec3d.x().to_f32().unwrap_or_default();
        let y = dec3d.y().to_f32().unwrap_or_default();
        [x, y]
    }

    pub fn verts(&self) -> Arc<RwLock<Vec<[f32; 3]>>> {
        let mut vertices = self.vertices.write().unwrap();
        if vertices.is_empty() {
            vertices.resize(12, [0.0, 0.0, 0.0]);
            let vert_dec = self.dec_verts();
            let mut populate_vert = |index: usize| {
                vertices[index] = Self::d3_to_f3(vert_dec.read().unwrap()[index].clone());
            };

            // N Pole
            populate_vert(NP_n1_P_0);

            // Arctic Circle
            populate_vert(N0__1_P_0);
            populate_vert(N2__0_1nP);
            populate_vert(N4_nP_0n1);
            populate_vert(N6_nP_0_1);
            populate_vert(N8__0_1_P);

            // Antarctic Circle
            populate_vert(S1__P_0n1);
            populate_vert(S3__0n1nP);
            populate_vert(S5_n1nP_0);
            populate_vert(S7__0n1_P);
            populate_vert(S9__P_0_1);

            // S Pole
            populate_vert(SP__1nP_0);
        }
        Arc::clone(&self.vertices)
    }

    pub fn norms(&self) -> Arc<RwLock<Vec<[f32; 3]>>> {
        let mut normals = self.normals.write().unwrap();
        if normals.is_empty() {
            normals.resize(12, [0.0, 0.0, 0.0]);
            let norm_dec = self.dec_norms();
            let mut populate_norm = |index: usize| {
                let mut n = Self::d3_to_f3(norm_dec.read().unwrap()[index].clone());
                let length = (n[0]*n[0] + n[1]*n[1] + n[2]*n[2]).sqrt();
                n[0] /= length;
                n[1] /= length;
                n[2] /= length;
                normals[index] = n;
            };
            // N Pole
            populate_norm(NP_n1_P_0);

            // Arctic Circle
            populate_norm(N0__1_P_0);
            populate_norm(N2__0_1nP);
            populate_norm(N4_nP_0n1);
            populate_norm(N6_nP_0_1);
            populate_norm(N8__0_1_P);

            // Antarctic Circle
            populate_norm(S1__P_0n1);
            populate_norm(S3__0n1nP);
            populate_norm(S5_n1nP_0);
            populate_norm(S7__0n1_P);
            populate_norm(S9__P_0_1);

            // S Pole
            populate_norm(SP__1nP_0);
        }
        Arc::clone(&self.normals)
    }

    pub fn uvs(&self) -> Arc<RwLock<Vec<[f32; 2]>>> {
        let mut uvs = self.uvs.write().unwrap();
        if uvs.is_empty() {
            uvs.resize(12, [0.0, 0.0]);
            let uvs_dec = self.dec_uvs();

            let mut populate_uv = |index: usize| {
                uvs[index] = Self::d2_to_f2(uvs_dec.read().unwrap()[index].clone());
            };

            // N Pole, for now it squashes to a point
            populate_uv(NP_n1_P_0);

            // Arctic Circle
            populate_uv(N0__1_P_0);
            populate_uv(N2__0_1nP);
            populate_uv(N4_nP_0n1);
            populate_uv(N6_nP_0_1);
            populate_uv(N8__0_1_P);

            // Antarctic Circle
            populate_uv(S1__P_0n1);
            populate_uv(S3__0n1nP);
            populate_uv(S5_n1nP_0);
            populate_uv(S7__0n1_P);
            populate_uv(S9__P_0_1);

            // S Pole, for now it squashes to a point
            populate_uv(SP__1nP_0);
        }
        Arc::clone(&self.uvs)
    }

    pub fn tris(&self) -> Arc<RwLock<Vec<u32>>> {
        let mut indices = self.indices.write().unwrap();
        if indices.is_empty() {
            // ccw, base, base, point
            let indices_vertex: Vec<usize> = vec![
                // nP Cap
                NP_n1_P_0, N0__1_P_0, N2__0_1nP, NP_n1_P_0, N2__0_1nP, N4_nP_0n1, NP_n1_P_0,
                N8__0_1_P, N0__1_P_0, NP_n1_P_0, N6_nP_0_1, N8__0_1_P, NP_n1_P_0, N4_nP_0n1,
                N6_nP_0_1, // N Equator Band
                N2__0_1nP, N0__1_P_0, S1__P_0n1, N4_nP_0n1, N2__0_1nP, S3__0n1nP, N6_nP_0_1,
                N4_nP_0n1, S5_n1nP_0, N8__0_1_P, N6_nP_0_1, S7__0n1_P, N0__1_P_0, N8__0_1_P,
                S9__P_0_1, // S Equator Band
                S1__P_0n1, S3__0n1nP, N2__0_1nP, S3__0n1nP, S5_n1nP_0, N4_nP_0n1, S5_n1nP_0,
                S7__0n1_P, N6_nP_0_1, S7__0n1_P, S9__P_0_1, N8__0_1_P, S9__P_0_1, S1__P_0n1,
                N0__1_P_0, // S Pole Cap
                SP__1nP_0, S3__0n1nP, S1__P_0n1, SP__1nP_0, S5_n1nP_0, S3__0n1nP, SP__1nP_0,
                S7__0n1_P, S5_n1nP_0, SP__1nP_0, S9__P_0_1, S7__0n1_P, SP__1nP_0, S1__P_0n1,
                S9__P_0_1,
            ];

            let mut indices_u32: Vec<u32> = Vec::new();
            for index in indices_vertex {
                indices_u32.push(index as u32);
            }

            *indices = indices_u32;
        }
        Arc::clone(&self.indices)
    }
}
