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

        let mut input = std::mem::take(&mut self.regions);
        let mut result = Vec::new();

        while let Some(mut base) = input.pop() {
            let mut i = 0;
            while i < input.len() {
                if let Some(merged) = merge_if_intersect(&base, &input[i]) {
                    base = merged;
                    input.swap_remove(i);
                    i = 0;
                } else {
                    i += 1;
                }
            }
            result.push(base);
        }

        self.regions = result;
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
    a.rect()
        .union_intersects(&b.rect())
        .map(|rect| CoordRect::new(rect, a.coord()))
}

#[cfg(test)]
mod tests {
    use super::{CoordRegion, FRegion, Region};
    use crate::{
        figure::{CoordRect, FRect, Rect},
        prelude::Coordinate,
    };

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

    #[test]
    fn test_region_merge() {
        let mut coord_region = CoordRegion::new();

        // Merge chain: R0 merges with R1, then with R2 -> final rect (100, 100, 300, 100)
        coord_region.add_rect(CoordRect::new(
            FRect::new(100.0, 100.0, 100.0, 100.0),
            Coordinate::World,
        )); // R0
        coord_region.add_rect(CoordRect::new(
            FRect::new(200.0, 100.0, 100.0, 100.0),
            Coordinate::World,
        )); // R1
        coord_region.add_rect(CoordRect::new(
            FRect::new(299.9, 100.0, 100.0, 100.0),
            Coordinate::World,
        )); // R2 (touching by EPSILON)

        // Merge chain: R3 and R4 overlap vertically -> (500, 200, 100, 300)
        coord_region.add_rect(CoordRect::new(
            FRect::new(500.0, 200.0, 100.0, 100.0),
            Coordinate::World,
        )); // R3
        coord_region.add_rect(CoordRect::new(
            FRect::new(500.0, 300.0, 100.0, 200.0),
            Coordinate::World,
        )); // R4

        // Non-overlapping rectangles
        coord_region.add_rect(CoordRect::new(
            FRect::new(50.0, 50.0, 40.0, 40.0),
            Coordinate::World,
        )); // R5
        coord_region.add_rect(CoordRect::new(
            FRect::new(1000.0, 1000.0, 10.0, 10.0),
            Coordinate::World,
        )); // R6

        // Tiny overlap (barely touching)
        coord_region.add_rect(CoordRect::new(
            FRect::new(800.0, 800.0, 50.0, 50.0),
            Coordinate::World,
        )); // R7
        coord_region.add_rect(CoordRect::new(
            FRect::new(849.95, 800.0, 50.0, 50.0),
            Coordinate::World,
        )); // R8 (barely touches R7)

        // Trigger merging
        coord_region.merge_all();

        // Expected results:
        // - Merged: R0+R1+R2 -> (100, 100, 300, 100)
        // - Merged: R3+R4 -> (500, 200, 100, 300)
        // - Merged: R7+R8 -> (800, 800, 99.95, 50)
        // - Unmerged: R5, R6

        assert_eq!(coord_region.len(), 5);

        for (i, r) in coord_region.regions.iter().enumerate() {
            println!("Region {}: {:?}", i, r);
        }
    }
}
