#![allow(dead_code)]
use log::warn;
use std::ops::Mul;
use tlib::global::{fuzzy_is_null_32, round32};
use TransformType::*;
use super::Point;

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum TransformType {
    #[default]
    None = 0x00,
    Translate = 0x01,
    Scale = 0x02,
    Rotate = 0x04,
    Shear = 0x08,
    Project = 0x10,
}
impl TransformType {
    pub fn as_u8(&self) -> u8 {
        match self {
            None => 0x00,
            Translate => 0x01,
            Scale => 0x02,
            Rotate => 0x04,
            Shear => 0x08,
            Project => 0x10,
        }
    }

    pub fn max(self, rhs: Self) -> Self {
        if self.as_u8() >= rhs.as_u8() {
            self
        } else {
            rhs
        }
    }
}

/// | | | |
/// |---|---|---|
/// | m11 `Horizontal zoom` | m12 `Vertical shear` | m13 `Horizontal projection` |
/// | m21 `Horizontal shear` | m22 `Vertical zoom` | m23 `Vertical projection` |
/// | m31(dx) `horizontal translation` | m32(dy) `vertical translation` | m33 `Projection factor` |
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transform {
    matrix: [[f32; 3]; 3],
    type_: TransformType,
    dirty: TransformType,
}

impl Transform {
    #[inline]
    pub fn new() -> Self {
        Self {
            matrix: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            type_: None,
            dirty: None,
        }
    }

    #[inline]
    pub fn new_project(
        h11: f32,
        h12: f32,
        h13: f32,
        h21: f32,
        h22: f32,
        h23: f32,
        h31: f32,
        h32: f32,
        h33: f32,
    ) -> Self {
        Self {
            matrix: [[h11, h12, h13], [h21, h22, h23], [h31, h32, h33]],
            type_: None,
            dirty: Project,
        }
    }

    #[inline]
    pub fn new_shear(h11: f32, h12: f32, h21: f32, h22: f32, dx: f32, dy: f32) -> Self {
        Self {
            matrix: [[h11, h12, 0.], [h21, h22, 0.], [dx, dy, 1.]],
            type_: None,
            dirty: Shear,
        }
    }

    pub fn scale(&mut self, sx: f32, sy: f32) -> &mut Self {
        if sx == 1. && sy == 1. {
            return self;
        }

        if sx == 0. || sy == 0. {
            warn!("`Transform` scale was NaN");
            return self;
        }

        match self.inline_type() {
            None | Translate => {
                self.set_m11(sx);
                self.set_m22(sy);
            }
            Project => {
                self.set_m13(self.m13() * sx);
                self.set_m23(self.m23() * sy);

                self.set_m12(self.m12() * sx);
                self.set_m21(self.m21() * sy);

                self.set_m11(self.m11() * sx);
                self.set_m22(self.m22() * sy);
            }
            Rotate | Shear => {
                self.set_m12(self.m12() * sx);
                self.set_m21(self.m21() * sy);

                self.set_m11(self.m11() * sx);
                self.set_m22(self.m22() * sy);
            }
            Scale => {
                self.set_m11(self.m11() * sx);
                self.set_m22(self.m22() * sy);
            }
        }

        if self.dirty.as_u8() < Scale.as_u8() {
            self.dirty = Scale
        }

        self
    }

    pub fn inverted(&mut self) -> Self {
        let mut invert = Self::new();
        let mut inv = true;
        match self.inline_type() {
            None => {}
            Translate => {
                invert.set_dx(-self.dx());
                invert.set_dy(-self.dy());
            }
            Scale => {
                inv = !fuzzy_is_null_32(self.m11());
                inv &= !fuzzy_is_null_32(self.m22());
                if inv {
                    invert.set_m11(1. / self.m11());
                    invert.set_m22(1. / self.m22());
                    invert.set_dx(-self.dx() * invert.m11());
                    invert.set_dy(-self.dy() * invert.m22());
                }
            }
            Rotate | Shear => {
                let dtr = self.m11() * self.m22() - self.m12() * self.m21();
                if dtr == 0. {
                    if inv {
                        inv = false;
                    }
                    self.set_m11(1.);
                    self.set_m12(0.);
                    self.set_m21(0.);
                    self.set_m22(1.);
                    self.set_dx(0.);
                    self.set_dy(0.);
                } else {
                    let dinv = 1. / dtr;
                    self.set_m11(self.m22() * dinv);
                    self.set_m12(-self.m12() * dinv);
                    self.set_m21(-self.m21() * dinv);
                    self.set_m22(self.m11() * dinv);
                    self.set_dx((self.m21() * self.dy() - self.m22() * self.dx()) * dinv);
                    self.set_dy((self.m12() * self.dx() - self.m11() * self.dy()) * dinv);
                }
            }
            _ => {
                let det = self.determinant();
                inv = !fuzzy_is_null_32(det);
                if inv {
                    invert = self.adjoint();
                    invert.div(det);
                }
            }
        }

        if inv {
            // inverting doesn't change the type
            invert.type_ = self.type_;
            invert.dirty = self.dirty;
        }

        invert
    }

    pub fn map_point(&mut self, point: &Point) -> Point {
        let fx = point.x() as f32;
        let fy = point.y() as f32;

        let mut x;
        let mut y;

        let type_ = self.inline_type();
        match type_ {
            None => {
                x = fx;
                y = fy;
            }
            Translate => {
                x = fx + self.dx();
                y = fy + self.dy();
            }
            Scale => {
                x = self.m11() * fx + self.dx();
                y = self.m22() * fy + self.dy();
            }
            Rotate | Shear | Project => {
                x = self.m11() * fx + self.m21() * fy + self.dx();
                y = self.m12() * fx + self.m22() * fy + self.dy();
                if type_ == Project {
                    let w = 1. / (self.m13() * fx + self.m23() * fy + self.m33());
                    x *= w;
                    y *= w;
                }
            }
        }

        Point::new(round32(x), round32(y))
    }

    #[inline]
    pub fn inline_type(&mut self) -> TransformType {
        if self.dirty == None {
            return self.type_;
        }
        self.type_()
    }

    pub fn type_(&mut self) -> TransformType {
        if self.dirty == None || self.dirty.as_u8() < self.type_.as_u8() {
            return self.type_;
        }

        match self.dirty {
            Project => {
                if !fuzzy_is_null_32(self.m13())
                    || !fuzzy_is_null_32(self.m23())
                    || !fuzzy_is_null_32(self.m33() - 1.)
                {
                    self.type_ = Project;
                }
            }
            Shear | Rotate => {
                if !fuzzy_is_null_32(self.m12()) || !fuzzy_is_null_32(self.m21()) {
                    let dot = self.m11() * self.m12() + self.m21() * self.m22();
                    if fuzzy_is_null_32(dot) {
                        self.type_ = Rotate;
                    } else {
                        self.type_ = Shear;
                    }
                } else {
                    self.type_ = None;
                }
            }
            Scale => {
                if !fuzzy_is_null_32(self.m11() - 1.) || !fuzzy_is_null_32(self.m22() - 1.) {
                    self.type_ = Scale;
                } else {
                    self.type_ = None;
                }
            }
            Translate => {
                if !fuzzy_is_null_32(self.dx()) || !fuzzy_is_null_32(self.dy()) {
                    self.type_ = Translate;
                } else {
                    self.type_ = None;
                }
            }
            None => self.type_ = None,
        }

        self.dirty = None;
        self.type_
    }

    #[inline]
    pub fn adjoint(&self) -> Self {
        let h11 = self.m22() * self.m33() - self.m23() * self.dy();
        let h21 = self.m23() * self.dx() - self.m21() * self.m33();
        let h31 = self.m21() * self.dy() - self.m22() * self.dx();
        let h12 = self.m13() * self.dy() - self.m12() * self.m33();
        let h22 = self.m11() * self.m33() - self.m13() * self.dx();
        let h32 = self.m12() * self.dx() - self.m11() * self.dy();
        let h13 = self.m12() * self.m23() - self.m13() * self.m22();
        let h23 = self.m13() * self.m21() - self.m11() * self.m23();
        let h33 = self.m11() * self.m22() - self.m12() * self.m21();

        Transform::new_project(h11, h12, h13, h21, h22, h23, h31, h32, h33)
    }

    #[inline]
    pub fn mul(&mut self, num: f32) {
        if num == 1. {
            return;
        }
        self.matrix[0][0] *= num;
        self.matrix[0][1] *= num;
        self.matrix[0][2] *= num;
        self.matrix[1][0] *= num;
        self.matrix[1][1] *= num;
        self.matrix[1][2] *= num;
        self.matrix[2][0] *= num;
        self.matrix[2][1] *= num;
        self.matrix[2][2] *= num;
        if self.dirty.as_u8() < Scale.as_u8() {
            self.dirty = Scale
        }
    }

    #[inline]
    pub fn div(&mut self, mut div: f32) {
        if div == 0. {
            return;
        }
        div = 1. / div;
        self.mul(div)
    }

    #[inline]
    pub fn add(&mut self, num: f32) {
        if num == 0. {
            return;
        }
        self.matrix[0][0] += num;
        self.matrix[0][1] += num;
        self.matrix[0][2] += num;
        self.matrix[1][0] += num;
        self.matrix[1][1] += num;
        self.matrix[1][2] += num;
        self.matrix[2][0] += num;
        self.matrix[2][1] += num;
        self.matrix[2][2] += num;
        self.dirty = Project;
    }

    #[inline]
    pub fn sub(&mut self, num: f32) {
        if num == 0. {
            return;
        }
        self.matrix[0][0] -= num;
        self.matrix[0][1] -= num;
        self.matrix[0][2] -= num;
        self.matrix[1][0] -= num;
        self.matrix[1][1] -= num;
        self.matrix[1][2] -= num;
        self.matrix[2][0] -= num;
        self.matrix[2][1] -= num;
        self.matrix[2][2] -= num;
        self.dirty = Project;
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.m11() * (self.m33() * self.m22() - self.dy() * self.m23())
            - self.m21() * (self.m33() * self.m12() - self.dy() * self.m13())
            + self.dx() * (self.m23() * self.m12() - self.m22() * self.m13())
    }

    /// Get the m11 `Horizontal zoom`
    #[inline]
    pub fn m11(&self) -> f32 {
        self.matrix[0][0]
    }

    /// Get the m12 `Vertical shear`
    #[inline]
    pub fn m12(&self) -> f32 {
        self.matrix[0][1]
    }

    /// Get the m13 `Horizontal projection`
    #[inline]
    pub fn m13(&self) -> f32 {
        self.matrix[0][2]
    }

    /// Get the m21 `Horizontal shear`
    #[inline]
    pub fn m21(&self) -> f32 {
        self.matrix[1][0]
    }

    /// Get the m22 `Vertical zoom`
    #[inline]
    pub fn m22(&self) -> f32 {
        self.matrix[1][1]
    }

    /// Get the m23 `Vertical projection`
    #[inline]
    pub fn m23(&self) -> f32 {
        self.matrix[1][2]
    }

    /// Get the m31 `Horizontal translation`
    #[inline]
    pub fn m31(&self) -> f32 {
        self.matrix[2][0]
    }

    /// Get the m32 `Vertical translation`
    #[inline]
    pub fn m32(&self) -> f32 {
        self.matrix[2][1]
    }

    /// Get the m33 `Projection factor`
    #[inline]
    pub fn m33(&self) -> f32 {
        self.matrix[2][2]
    }

    /// Get the dx
    #[inline]
    pub fn dx(&self) -> f32 {
        self.matrix[2][0]
    }

    /// Get the dy
    #[inline]
    pub fn dy(&self) -> f32 {
        self.matrix[2][1]
    }

    /// Set `Horizontal zoom`
    #[inline]
    fn set_m11(&mut self, f: f32) {
        self.matrix[0][0] = f;
    }

    /// Set `Vertical shear`
    #[inline]
    fn set_m12(&mut self, f: f32) {
        self.matrix[0][1] = f;
    }

    /// Set `Horizontal projection`
    #[inline]
    fn set_m13(&mut self, f: f32) {
        self.matrix[0][2] = f;
    }

    /// Set `Horizontal shear`
    #[inline]
    fn set_m21(&mut self, f: f32) {
        self.matrix[1][0] = f;
    }

    /// Set `Vertical zoom`
    #[inline]
    fn set_m22(&mut self, f: f32) {
        self.matrix[1][1] = f;
    }

    /// Set `Vertical projection`
    #[inline]
    fn set_m23(&mut self, f: f32) {
        self.matrix[1][2] = f;
    }

    /// Set dx `Horizontal translation`
    #[inline]
    fn set_m31(&mut self, f: f32) {
        self.matrix[2][0] = f;
    }

    /// Set dy `Vertical translation`
    #[inline]
    fn set_m32(&mut self, f: f32) {
        self.matrix[2][1] = f;
    }

    /// Set `Projection factor`
    #[inline]
    fn set_m33(&mut self, f: f32) {
        self.matrix[2][2] = f;
    }

    /// Set dx `Horizontal translation`
    #[inline]
    fn set_dx(&mut self, f: f32) {
        self.matrix[2][0] = f;
    }

    /// Set dy `Vertical translation`
    #[inline]
    fn set_dy(&mut self, f: f32) {
        self.matrix[2][1] = f;
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let other_type = rhs.inline_type();
        if other_type == None {
            return self;
        }

        let this_type = self.inline_type();
        if this_type == None {
            return rhs;
        }

        let type_ = this_type.max(other_type);
        match type_ {
            None => {}
            Translate => {
                self.set_dx(self.dx() + rhs.dx());
                self.set_dy(self.dy() + rhs.dy());
            }
            Scale => {
                let m11 = self.m11() * rhs.m11();
                let m22 = self.m22() * rhs.m22();

                let m31 = self.dx() * rhs.m11() + rhs.dx();
                let m32 = self.dy() * rhs.m22() + rhs.dy();

                self.set_m11(m11);
                self.set_m22(m22);
                self.set_dx(m31);
                self.set_dy(m32);
            }
            Rotate | Shear => {
                let m11 = self.m11() * rhs.m11() + self.m12() * rhs.m21();
                let m12 = self.m11() * rhs.m12() + self.m12() * rhs.m22();

                let m21 = self.m21() * rhs.m11() + self.m22() * rhs.m21();
                let m22 = self.m21() * rhs.m12() + self.m22() * rhs.m22();

                let m31 = self.dx() * rhs.m11() + self.dy() * rhs.m21() + rhs.dx();
                let m32 = self.dx() * rhs.m12() + self.dy() * rhs.m22() + rhs.dy();

                self.set_m11(m11);
                self.set_m12(m12);
                self.set_m21(m21);
                self.set_m22(m22);
                self.set_dx(m31);
                self.set_dy(m32);
            }
            Project => {
                let m11 = self.m11() * rhs.m11() + self.m12() * rhs.m21() + self.m13() * rhs.dx();
                let m12 = self.m11() * rhs.m12() + self.m12() * rhs.m22() + self.m13() * rhs.dy();
                let m13 = self.m11() * rhs.m13() + self.m12() * rhs.m23() + self.m13() * rhs.m33();

                let m21 = self.m21() * rhs.m11() + self.m22() * rhs.m21() + self.m23() * rhs.dx();
                let m22 = self.m21() * rhs.m12() + self.m22() * rhs.m22() + self.m23() * rhs.dy();
                let m23 = self.m21() * rhs.m13() + self.m22() * rhs.m23() + self.m23() * rhs.m33();

                let m31 = self.dx() * rhs.m11() + self.dy() * rhs.m21() + self.m33() * rhs.dx();
                let m32 = self.dx() * rhs.m12() + self.dy() * rhs.m22() + self.m33() * rhs.dy();
                let m33 = self.dx() + rhs.m13() + self.dy() * rhs.m23() + self.m33() * rhs.m33();

                self.set_m11(m11);
                self.set_m12(m12);
                self.set_m13(m13);
                self.set_m21(m21);
                self.set_m22(m22);
                self.set_m23(m23);
                self.set_dx(m31);
                self.set_dy(m32);
                self.set_m33(m33);
            }
        }

        self.dirty = type_;
        self.type_ = type_;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::Transform;
    use crate::prelude::Rect;

    #[test]
    fn test_transform() {
        let rect = Rect::new(100, 100, 100, 100);

        let mut transform = Transform::new();
        transform.scale(2., 1.);
        transform.scale(1., 2.);
        let point = transform.inverted().map_point(&rect.top_left());
        println!("{:?}", point)
    }
}
