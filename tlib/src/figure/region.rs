use std::{vec::IntoIter, slice::{Iter, IterMut}};
use skia_safe::region::RegionOp;

use super::Rect;

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Region
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Region {
    regions: Vec<Rect>,
}

impl Region {
    #[inline]
    pub fn new() -> Self {
        Self { regions: vec![] }
    }

    #[inline]
    pub fn add_rect(&mut self, rect: Rect) {
        self.regions.push(rect)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.regions.clear()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    #[inline]
    pub fn intersects_rect(&self, other: &Rect) -> Region {
        let mut region = Region::new();

        for rect in self.regions.iter() {
            if let Some(rect) = rect.intersects(other) {
                region.add_rect(rect)
            }
        }

        region
    }

    #[inline]
    pub fn regions(&self) -> &Vec<Rect> {
        &self.regions
    }

    #[inline]
    pub fn iter(&self) -> Iter<Rect> {
        self.regions.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Rect> {
        self.regions.iter_mut()
    }

    #[inline]
    pub fn offset(&mut self, offset: (i32, i32)) {
        for rect in self.iter_mut() {
            rect.offset(offset.0, offset.1);
        }
    }
}

impl IntoIterator for Region {
    type Item = Rect;

    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.regions.into_iter()
    }
}

impl Into<skia_safe::Region> for Region {
    fn into(self) -> skia_safe::Region {
        let mut region = skia_safe::Region::new();
        for rect in self.into_iter() {
            let rect: skia_safe::IRect = rect.into();
            region.op_rect(rect, RegionOp::Union);
        }
        region
    }
}

#[cfg(test)]
mod tests {
    use crate::figure::Rect;
    use super::Region;

    #[test]
    fn test_region() {
        let mut region = Region::new();
        let origin = Rect::new(0, 5, 120, 300);
        region.add_rect(origin);
        for rect in region.iter() {
            assert_eq!(*rect, origin);
        }
    }
}