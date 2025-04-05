use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use super::{CoordRect, FPoint, FRect, Point, Rect};

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
    pub fn add_region(&mut self, region: &Region) {
        for rect in region.iter() {
            self.add_rect(*rect);
        }
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
    pub fn contains_point(&self, point: &Point) -> bool {
        for rect in self.regions.iter() {
            if rect.contains(point) {
                return true;
            }
        }
        false
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

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// FRegion
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FRegion {
    regions: Vec<FRect>,
}

impl FRegion {
    #[inline]
    pub fn new() -> Self {
        Self { regions: vec![] }
    }

    #[inline]
    pub fn add_rect(&mut self, rect: FRect) {
        self.regions.push(rect)
    }

    #[inline]
    pub fn add_region(&mut self, region: &FRegion) {
        for rect in region.iter() {
            self.add_rect(*rect);
        }
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
    pub fn contains_point(&self, point: &FPoint) -> bool {
        for rect in self.regions.iter() {
            if rect.contains(point) {
                return true;
            }
        }
        false
    }

    #[inline]
    pub fn intersects_rect(&self, other: &FRect) -> FRegion {
        let mut region = FRegion::new();

        for rect in self.regions.iter() {
            if let Some(rect) = rect.intersects(other) {
                region.add_rect(rect)
            }
        }

        region
    }

    #[inline]
    pub fn regions(&self) -> &Vec<FRect> {
        &self.regions
    }

    #[inline]
    pub fn iter(&self) -> Iter<FRect> {
        self.regions.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<FRect> {
        self.regions.iter_mut()
    }

    #[inline]
    pub fn offset(&mut self, offset: (f32, f32)) {
        for rect in self.iter_mut() {
            rect.offset(offset.0, offset.1);
        }
    }
}

impl IntoIterator for FRegion {
    type Item = FRect;

    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.regions.into_iter()
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// CoordRegion
//////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CoordRegion {
    regions: Vec<CoordRect>,
}
impl CoordRegion {
    #[inline]
    pub fn new() -> Self {
        Self { regions: vec![] }
    }

    #[inline]
    pub fn add_rect(&mut self, rect: CoordRect) {
        self.regions.push(rect)
    }

    #[inline]
    pub fn add_region(&mut self, region: &CoordRegion) {
        for rect in region.iter() {
            self.add_rect(*rect);
        }
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
    pub fn regions(&self) -> &Vec<CoordRect> {
        &self.regions
    }

    #[inline]
    pub fn iter(&self) -> Iter<CoordRect> {
        self.regions.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<CoordRect> {
        self.regions.iter_mut()
    }

    pub fn merge_all(&mut self) {
        if self.regions.is_empty() {
            return;
        }

        loop {
            let mut merged = Vec::new();
            let mut used = vec![false; self.regions.len()];
            let mut changed = false;

            for i in 0..self.regions.len() {
                if used[i] {
                    continue;
                }
                let mut current = self.regions[i];
                used[i] = true;

                let mut j = 0;
                while j < self.regions.len() {
                    if i != j && !used[j] {
                        if let Some(merged_rect) = merge_if_intersect(&current, &self.regions[j]) {
                            current = merged_rect;
                            used[j] = true;
                            changed = true;
                            j = 0;
                            continue;
                        }
                    }
                    j += 1;
                }
                merged.push(current);
            }

            if !changed {
                return;
            }
            self.regions = merged;
        }
    }
}
impl IntoIterator for CoordRegion {
    type Item = CoordRect;

    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.regions.into_iter()
    }
}

fn merge_if_intersect(a: &CoordRect, b: &CoordRect) -> Option<CoordRect> {
    if a.coord() != b.coord() {
        return None;
    }
    if let Some(rect) = a.rect().intersects(&b.rect()) {
        Some(CoordRect::new(rect, a.coord()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{FRegion, Region};
    use crate::figure::{FRect, Rect};

    #[test]
    fn test_region() {
        let mut region = Region::new();
        let origin = Rect::new(0, 5, 120, 300);
        region.add_rect(origin);
        for rect in region.iter() {
            assert_eq!(*rect, origin);
        }
    }

    #[test]
    fn test_fregion() {
        let mut region = FRegion::new();
        let origin = FRect::new(0., 5., 120., 300.);
        region.add_rect(origin);
        for rect in region.iter() {
            assert_eq!(*rect, origin);
        }
    }
}
