use core::fmt;
use std::{cell::RefCell, fmt::Display, num::ParseFloatError, ops::Deref, rc::Rc, str::{FromStr, ParseBoolError}};

use cgmath::{ElementWise, InnerSpace, MetricSpace, Vector2, Vector3, Zero};
use sdf_viewer::sdf::{defaults, ffi::set_root_sdf, SDFParam, SDFParamKind, SDFParamValue, SDFSample, SDFSurface};
use clap;

/// Entrypoint: only needs to set the root SDFSurface implementation.
#[no_mangle]
pub extern "C" fn init() {
    set_root_sdf(Box::new(SDFDemo));
}

#[derive(clap::Parser, Debug, Clone, PartialEq, Eq)]
pub struct SDFDemo;

fn shift_origin(xnorm: f32, ynorm: f32, shift: f32) -> (f32, f32) {
    if ynorm == 0.0 { if xnorm < 0.0 { (shift, 0.0) } else { (-shift, 0.0) } }
    else if xnorm == 0.0 { if ynorm < 0.0 { (0.0, shift) } else { (0.0, -shift) } }
    else { (-shift * xnorm, -shift * ynorm) }
}

fn outer_inner_sdf(p: Vector3<f32>) -> (f32, f32, Vector3<f32>, Vector3<f32>) {
    let x = p.x;
    let z = p.y;
    let y = p.z;
    let xy_distance = (x*x + y*y).sqrt();
    let xnorm = x / xy_distance;
    let ynorm = y / xy_distance;

    let oshift = 3.0;
    let (ox, oy) = shift_origin(xnorm, ynorm, oshift);
    let or = 2.82 + oshift;
    let onorm = Vector3::new(x, y, z).sub_element_wise(Vector3::new(ox, oy, 0.0));
    let sdf_outer = onorm.distance(Vector3::zero()) - or;

    let ishift = 2.0;
    let (ix, iy) = shift_origin(xnorm, ynorm, ishift);
    let ir = 2.32 + ishift;
    let inorm = Vector3::new(x, y, z).sub_element_wise(Vector3::new(ix, iy, 0.0));
    let sdf_inner = inorm.distance(Vector3::zero()) - ir;

    (sdf_outer, sdf_inner, onorm, inorm)
}

impl SDFSurface for SDFDemo {
    fn bounding_box(&self) -> [Vector3<f32>; 2] {
        [Vector3::new(-3.0, -0.1, -3.0), Vector3::new(3.0, 5.0, 3.0)]
    }

    fn sample(&self, p: Vector3<f32>, _distance_only: bool) -> SDFSample {
        let (sdf_outer, sdf_inner, _, _) = outer_inner_sdf(p);
        SDFSample::new(
            sdf_outer.max(-sdf_inner).max(-p.y).max(p.z),
            if 4.0 < p.y && p.y < 4.1 { Vector3::new(0.0, 1.0, 0.0) }
            else if 2.0 < p.y && p.y < 2.1 { Vector3::new(1.0, 0.0, 0.0) }
            else { Vector3::zero() },
        )
    }

    fn changed(&self) -> Option<[Vector3<f32>; 2]> {
        None
    }

    // fn normal(&self, p: Vector3<f32>, _eps: Option<f32>) -> Vector3<f32> {
    //     let (sdf_outer, sdf_inner, onorm, inorm) = outer_inner_sdf(p);
    //     if -sdf_inner < sdf_outer { onorm.normalize() } else { inorm.normalize() }
    // }
}
