use crate::widget::WidgetImpl;

use super::{
    list_group::ListGroup, list_item::ListItem, list_node::ListNode,
    list_view_object::ListViewObject, WidgetHnd,
};
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ptr::{addr_of_mut, NonNull},
    sync::atomic::Ordering,
};
use tlib::{
    global::SemanticExt, nonnull_mut, object::{IdGenerator, ObjectId}
};

static STORE_ID_INCREMENT: IdGenerator = IdGenerator::new(0);

pub struct ListStore {
    id: ObjectId,
    view: WidgetHnd,
    items: Vec<Box<dyn ListItem>>,

    pub(crate) id_increment: IdGenerator,
}

impl ListStore {
    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) {
        let mut node = ListNode::create_from_obj(obj).boxed();
        node.set_id(self.next_id());
        self.items.push(node)
    }

    #[inline]
    pub fn add_group(&mut self, mut group: ListGroup) {
        group.set_id(&self.id_increment);
        self.items.push(group.boxed())
    }
}

impl ListStore {
    #[inline]
    pub(crate) fn store_map() -> &'static mut HashMap<ObjectId, Option<NonNull<ListStore>>> {
        static mut STORE_MAP: Lazy<HashMap<ObjectId, Option<NonNull<ListStore>>>> =
            Lazy::new(HashMap::new);
        unsafe { addr_of_mut!(STORE_MAP).as_mut().unwrap() }
    }

    #[inline]
    pub(crate) fn store_ref(id: ObjectId) -> Option<&'static ListStore> {
        Self::store_map()
            .get(&id)
            .and_then(|hnd| unsafe { Some(hnd.as_ref().unwrap().as_ref()) })
    }

    #[inline]
    pub(crate) fn store_mut(id: ObjectId) -> Option<&'static mut ListStore> {
        Self::store_map()
            .get_mut(&id)
            .and_then(|hnd| unsafe { Some(hnd.as_mut().unwrap().as_mut()) })
    }

    #[inline]
    pub(crate) fn new() -> Self {
        let mut store = ListStore {
            id: STORE_ID_INCREMENT.fetch_add(1, Ordering::SeqCst),
            view: None,
            items: vec![],
            id_increment: IdGenerator::new(0),
        };
        Self::store_map().insert(store.id, NonNull::new(&mut store));

        store
    }

    #[inline]
    pub(crate) fn get_image(&mut self) -> &mut [Box<dyn ListItem>] {
        &mut self.items
    }

    #[inline]
    pub(crate) fn next_id(&self) -> ObjectId {
        self.id_increment.fetch_add(1, Ordering::SeqCst)
    }

    #[inline]
    pub(crate) fn set_view(&mut self, view: WidgetHnd) {
        self.view = view
    }

    #[inline]
    pub(crate) fn get_view(&mut self) -> &mut dyn WidgetImpl {
        nonnull_mut!(self.view)
    }
}

impl Default for ListStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
