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
    extends,
    global::SemanticExt,
    nonnull_mut,
    object::{IdGenerator, ObjectId, ObjectSubclass},
    prelude::*,
    signal, signals,
};

#[extends(Object, ignore_default = true)]
pub struct ListStore {
    view: WidgetHnd,
    items: Vec<Box<dyn ListItem>>,

    window_lines: i32,
    current_line: i32,
    y_offset: i32,

    pub(crate) id_increment: IdGenerator,
}

pub trait ListStoreSignals: ActionExt {
    signals!(
        ListStore:

        /// @param [`i32`]
        internal_scroll_value_changed();

        /// @param [`usize`]
        items_len_changed();
    );
}
impl ListStoreSignals for ListStore {}

impl ObjectSubclass for ListStore {
    const NAME: &'static str = "ListStore";
}
impl ObjectImpl for ListStore {}

impl ListStore {
    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) {
        let mut node = ListNode::create_from_obj(obj).boxed();
        node.set_id(self.next_id());
        self.items.push(node);

        emit!(self.items_len_changed(), self.items.len());
    }

    #[inline]
    pub fn add_group(&mut self, mut group: ListGroup) {
        if !self.items.is_empty() {
            let separator = group.take_separator().boxed();
            self.items.push(separator.as_list_item());
        }

        let nodes = group.take_nodes();
        for mut node in nodes.into_iter() {
            node.set_id(self.next_id());
            self.items.push(node.boxed())
        }

        emit!(self.items_len_changed(), self.items.len());
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
            .map(|hnd| unsafe { hnd.as_ref().unwrap().as_ref() })
    }

    #[inline]
    pub(crate) fn store_mut(id: ObjectId) -> Option<&'static mut ListStore> {
        Self::store_map()
            .get_mut(&id)
            .map(|hnd| unsafe { hnd.as_mut().unwrap().as_mut() })
    }

    #[inline]
    pub(crate) fn new() -> Self {
        let mut store = ListStore {
            object: Object::default(),
            view: None,
            items: vec![],
            window_lines: 0,
            current_line: 0,
            y_offset: 0,
            id_increment: IdGenerator::new(0),
        };
        Self::store_map().insert(store.id(), NonNull::new(&mut store));

        store
    }

    #[inline]
    pub(crate) fn get_image(&mut self) -> (&[Box<dyn ListItem>], i32) {
        let start = if self.y_offset == 0 {
            self.current_line
        } else {
            (self.current_line - 1).max(0)
        } as usize;

        let end = (start + self.window_lines as usize).min(self.items.len());

        (&self.items[start..end], self.y_offset)
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

    #[inline]
    pub(crate) fn set_window_lines(&mut self, window_lines: i32) {
        self.window_lines = window_lines
    }

    #[inline]
    pub(crate) fn get_window_lines(&self) -> i32 {
        self.window_lines
    }

    #[inline]
    pub(crate) fn get_items_len(&self) -> usize {
        self.items.len()
    }

    /// @param `internal`
    /// - true: The view scrolling triggered internally in ListView
    ///         requires notifying the scroll bar to change the value.
    ///
    /// @return `true` if scroll value has updated, should update the image.
    #[inline]
    pub(crate) fn scroll_to(&mut self, mut value: i32, internal: bool) -> bool {
        if internal {
            value *= 10;
        }

        let move_to = value / 10;
        if move_to < 0 || move_to > self.items.len() as i32 {
            return false;
        }
        self.y_offset = value % 10;
        if self.current_line == move_to {
            return true;
        }
        self.current_line = move_to;

        if internal {
            emit!(self.internal_scroll_value_changed())
        }

        true
    }
}

impl Default for ListStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
