use super::InputEle;
use derivative::Derivative;
use log::warn;
use nohash_hasher::IntMap;
use std::{cell::RefCell, ptr::NonNull};
use tlib::{nonnull_mut, nonnull_ref, object::ObjectId};

type InputEleHnd = Option<NonNull<dyn InputEle>>;

thread_local! {
    static INSTANCE: RefCell<FocusMgr> = RefCell::new(FocusMgr::default());
}

#[derive(Derivative)]
#[derivative(Default)]
pub(crate) struct FocusMgr {
    eles: IntMap<ObjectId, Vec<InputEleHnd>>,
    current: Option<usize>,
    cur_root: ObjectId,
}

impl FocusMgr {
    #[inline]
    pub(crate) fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&RefCell<FocusMgr>) -> R,
    {
        INSTANCE.with(f)
    }

    pub(crate) fn add(&mut self, root: ObjectId, ele: &mut dyn InputEle) {
        if root == 0 {
            warn!(
                "Add input ele `{}` to `FocusMgr` failed, the `root` is 0.",
                ele.name()
            );
            return;
        }
        let mut shuffle = false;
        let vec = self.eles.entry(root).or_default();

        if let Some(last) = vec.last() {
            let last = nonnull_ref!(last);
            if last.tabindex() > ele.tabindex() {
                shuffle = true;
            }
        }

        vec.push(NonNull::new(ele));

        if shuffle {
            vec.sort_by(|a, b| {
                let a = nonnull_ref!(a).tabindex();
                let b = nonnull_ref!(b).tabindex();
                a.cmp(&b)
            })
        }
    }

    pub(crate) fn remove(&mut self, id: ObjectId) {
        if self.cur_root == id {
            self.eles.remove(&id);
            self.cur_root = 0;
            self.current = None;
        } else if self.cur_root == 0 {
            self.current = None;
            for eles in self.eles.values_mut() {
                eles.retain(|r| nonnull_ref!(r).id() != id);
            }
        } else {
            for (&root, eles) in self.eles.iter_mut() {
                if let Some(index) = eles.iter().position(|r| nonnull_ref!(r).id() == id) {
                    eles.remove(index);
                    if let Some(current) = self.current {
                        if current >= eles.len() && self.cur_root == root && current != 0 {
                            self.current = Some(current - 1);
                        }
                    }
                    break;
                }
            }
        }
    }

    /// Should pre-check the widget is `InputEle`.
    pub(crate) fn set_currrent(&mut self, root: ObjectId, id: Option<ObjectId>) {
        self.cur_root = root;
        if id.is_none() {
            self.current = None;
            return;
        }
        let id = id.unwrap();

        let eles = self.eles.entry(root).or_default();

        for (idx, ele) in eles.iter().enumerate() {
            let ele = nonnull_ref!(ele);
            if id == ele.id() {
                if ele._is_enable() {
                    self.current = Some(idx);
                }
                break;
            }
        }
    }

    pub(crate) fn next(&mut self) -> Option<ObjectId> {
        let last = self.current?;
        let mut current = last;

        let eles = self.eles.get_mut(&self.cur_root);
        let eles = eles?;

        loop {
            current += 1;
            if current == eles.len() {
                current = 0;
            }
            let ele = nonnull_ref!(eles.get(current).unwrap());
            if ele._is_enable() && ele.visible() {
                self.current = Some(current);
                break;
            }
        }

        let ele = nonnull_mut!(eles.get_mut(last).unwrap());
        ele.on_tab_lose_focus();

        let ele = nonnull_mut!(eles.get_mut(self.current.unwrap()).unwrap());
        ele.on_tab_focused();
        Some(ele.id())
    }
}
