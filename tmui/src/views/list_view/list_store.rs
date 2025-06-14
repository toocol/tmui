use super::{
    list_group::ListGroup,
    list_item::ListItem,
    list_node::ListNode,
    list_separator::{GroupSeparator, DEFAULT_SEPARATOR_HEIGHT},
    list_view_object::ListViewObject,
    WidgetHnd,
};
use crate::{
    views::{list_view::list_item::ListItemCast, node::Status},
    widget::WidgetImpl,
};
use nohash_hasher::IntMap;
use once_cell::sync::Lazy;
use std::{
    ptr::{addr_of_mut, NonNull},
    sync::{atomic::Ordering, Arc},
};
use tipc::parking_lot::{lock_api::MutexGuard, Mutex, RawMutex};
use tlib::{
    extends,
    global::SemanticExt,
    nonnull_mut,
    object::{IdGenerator, ObjectId, ObjectSubclass},
    prelude::*,
    signal, signals,
};

pub type ConcurrentStoreMutexGuard<'a> = MutexGuard<'a, RawMutex, ConcurrentStore>;

pub struct ConcurrentStore {
    id: ObjectId,
    pub(crate) items: Vec<Box<dyn ListItem>>,

    separator: GroupSeparator,
    separator_cnt: usize,
    separator_pos: Vec<usize>,

    id_increment: IdGenerator,
}
impl ConcurrentStore {
    #[inline]
    fn new(id: ObjectId) -> Self {
        Self {
            id,
            items: vec![],
            separator: GroupSeparator::default(),
            separator_cnt: 0,
            separator_pos: vec![],
            id_increment: IdGenerator::new(0),
        }
    }

    #[inline]
    fn next_id(&self) -> ObjectId {
        self.id_increment.fetch_add(1, Ordering::SeqCst)
    }
}
impl ConcurrentStore {
    /// @return the index of added node
    pub fn add_node(&mut self, obj: &dyn ListViewObject) -> usize {
        if let Some(last) = self.items.last() {
            if let Some(node) = last.downcast_ref::<ListNode>() {
                if node.is_group_managed() {
                    let separator = self.separator.clone().boxed();
                    self.items.push(separator.as_list_item());
                    self.separator_cnt += 1;
                    self.separator_pos.push(self.items.len() - 1);
                }
            }
        }

        let mut node = ListNode::from(obj).boxed();
        node.set_store_id(self.id);
        node.set_id(self.next_id());
        self.items.push(node);
        self.items.len() - 1
    }

    /// @return the index of added node
    pub fn add_node_directly(&mut self, node: ListNode) -> usize {
        if let Some(last) = self.items.last() {
            if let Some(node) = last.downcast_ref::<ListNode>() {
                if node.is_group_managed() {
                    let separator = self.separator.clone().boxed();
                    self.items.push(separator.as_list_item());
                    self.separator_cnt += 1;
                    self.separator_pos.push(self.items.len() - 1);
                }
            }
        }

        let mut node = node.boxed();
        node.set_store_id(self.id);
        node.set_id(self.next_id());
        self.items.push(node);
        self.items.len() - 1
    }

    /// @return the index of last node in items.
    pub fn add_group(&mut self, mut group: ListGroup) -> Option<usize> {
        if group.is_empty() {
            return None;
        }

        if !self.items.is_empty() {
            let separator = self.separator.clone().boxed();
            self.items.push(separator.as_list_item());
            self.separator_cnt += 1;
            self.separator_pos.push(self.items.len() - 1);
        }

        let nodes = group.take_nodes();
        for mut node in nodes.into_iter() {
            node.set_store_id(self.id);
            node.set_id(self.next_id());
            node.set_group_managed(true);
            self.items.push(node.boxed())
        }

        Some(self.items.len() - 1)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.items.clear()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn nodes_len(&self) -> usize {
        self.items.len() - self.separator_cnt
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn separator_pos(&self) -> &[usize] {
        &self.separator_pos
    }
}

#[extends(Object, ignore_default = true)]
pub struct ListStore {
    view: WidgetHnd,
    concurrent_store: Arc<Mutex<ConcurrentStore>>,

    scroll_index_rec: Option<usize>,

    window_lines: i32,
    current_line: i32,
    y_offset: i32,
    len_rec: usize,
    separator_height: i32,

    entered_node: Option<NonNull<ListNode>>,
    hovered_node: Option<NonNull<ListNode>>,
    selected_node: Option<NonNull<ListNode>>,
}

pub trait ListStoreSignals: ActionExt {
    signals!(
        ListStore:

        /// @param [`i32`]
        internal_scroll_value_changed(i32);

        /// @param [`usize`]
        items_len_changed(usize);
    );
}
impl ListStoreSignals for ListStore {}

impl ObjectSubclass for ListStore {
    const NAME: &'static str = "ListStore";
}
impl ObjectImpl for ListStore {}

impl ListStore {
    #[inline]
    pub fn add_node(&mut self, obj: &dyn ListViewObject) -> usize {
        let mut mutex = self.concurrent_store.lock();
        let idx = mutex.add_node(obj);

        emit!(self, items_len_changed(mutex.len()));
        idx
    }

    #[inline]
    pub fn add_node_directly(&mut self, node: ListNode) -> usize {
        let mut mutex = self.concurrent_store.lock();
        let idx = mutex.add_node_directly(node);

        emit!(self, items_len_changed(mutex.len()));
        idx
    }

    #[inline]
    pub fn add_group(&mut self, group: ListGroup) -> Option<usize> {
        let mut mutex = self.concurrent_store.lock();
        let idx = mutex.add_group(group);

        emit!(self, items_len_changed(mutex.len()));
        idx
    }

    #[inline]
    pub fn clear(&mut self) {
        self.concurrent_store.lock().clear();

        emit!(self, items_len_changed(0usize));
    }

    #[inline]
    pub fn notify_update(&mut self) {
        self.get_view().update()
    }

    #[inline]
    pub fn concurrent_store(&mut self) -> Arc<Mutex<ConcurrentStore>> {
        if let Some(mutex) = self.concurrent_store.try_lock() {
            self.len_rec = mutex.len();
        }
        self.concurrent_store.clone()
    }

    #[inline]
    pub fn nodes_len(&self) -> usize {
        self.concurrent_store.lock().nodes_len()
    }
}

impl ListStore {
    #[inline]
    pub(crate) fn store_map() -> &'static mut IntMap<ObjectId, Option<NonNull<ListStore>>> {
        static mut STORE_MAP: Lazy<IntMap<ObjectId, Option<NonNull<ListStore>>>> =
            Lazy::new(IntMap::default);
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
        let object = Object::default();
        let id = object.id();

        ListStore {
            object,
            concurrent_store: Arc::new(Mutex::new(ConcurrentStore::new(id))),
            scroll_index_rec: None,
            view: None,
            window_lines: 0,
            current_line: 0,
            y_offset: 0,
            len_rec: 0,
            separator_height: DEFAULT_SEPARATOR_HEIGHT,
            entered_node: None,
            hovered_node: None,
            selected_node: None,
        }
    }

    #[inline]
    pub(crate) fn with_image<F: FnOnce(&[Box<dyn ListItem>], i32)>(&self, f: F) {
        let start = if self.y_offset == 0 {
            self.current_line
        } else {
            (self.current_line - 1).max(0)
        } as usize;

        if let Some(mutex) = self.concurrent_store.try_lock() {
            let end = (start + self.window_lines as usize + mutex.separator_cnt).min(mutex.len());

            f(&mutex.items[start..end], self.y_offset);
        }
    }

    #[inline]
    pub(crate) fn with_image_mut<
        F: FnOnce(
            ConcurrentStoreMutexGuard,
            usize,
            usize,
            i32,
            &mut Option<NonNull<ListNode>>,
            &mut Option<NonNull<ListNode>>,
            &mut Option<NonNull<ListNode>>,
        ) -> bool,
    >(
        &mut self,
        f: F,
    ) -> bool {
        let start = if self.y_offset == 0 {
            self.current_line
        } else {
            (self.current_line - 1).max(0)
        } as usize;

        if let Some(mutex) = self.concurrent_store.try_lock() {
            let end = (start + self.window_lines as usize + mutex.separator_cnt).min(mutex.len());
            self.len_rec = mutex.len();

            f(
                mutex,
                start,
                end,
                self.y_offset,
                &mut self.entered_node,
                &mut self.hovered_node,
                &mut self.selected_node,
            )
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn remove_effected_node_status(&mut self) {
        if self.entered_node.is_some() {
            self.entered_node = None;
        }
        if self.hovered_node.is_some() {
            nonnull_mut!(self.hovered_node).remove_status(Status::Hovered);
            self.hovered_node = None;
        }
        if self.selected_node.is_some() {
            nonnull_mut!(self.selected_node).remove_status(Status::Selected);
            self.selected_node = None;
        }
    }

    #[inline]
    pub(crate) fn set_view(&mut self, view: WidgetHnd) {
        self.view = view;
    }

    #[inline]
    pub(crate) fn get_view(&mut self) -> &mut dyn WidgetImpl {
        nonnull_mut!(self.view)
    }

    #[inline]
    pub(crate) fn get_selected_node(&self) -> Option<NonNull<ListNode>> {
        self.selected_node
    }

    #[inline]
    pub(crate) fn set_group_separator(&mut self, group_separator: GroupSeparator) {
        self.separator_height = group_separator.separator_height();
        self.concurrent_store.lock().separator = group_separator
    }

    #[inline]
    pub(crate) fn separator_height(&self) -> i32 {
        self.separator_height
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
        if let Some(mutex) = self.concurrent_store.try_lock() {
            mutex.len()
        } else {
            self.len_rec
        }
    }

    /// @param `internal`
    /// - true: The view scrolling triggered internally in ListView
    ///   requires notifying the scroll bar to change the value.
    ///
    /// @return `true` if scroll value has updated, should update the image.
    #[inline]
    pub(crate) fn scroll_to(&mut self, value: i32) -> bool {
        let move_to = value / 10;

        let len = if let Some(mutex) = self.concurrent_store.try_lock() {
            mutex.len()
        } else {
            self.len_rec
        };

        if move_to < 0 || move_to > len as i32 {
            return false;
        }
        self.y_offset = value % 10;
        if self.current_line == move_to {
            return true;
        }
        self.current_line = move_to;

        true
    }

    pub(crate) fn scroll_to_index(&mut self, idx: usize) {
        let idx_offset = if let Some(mutex) = self.concurrent_store.try_lock() {
            let mut offset = 0;
            for &i in mutex.separator_pos().iter() {
                if idx >= i {
                    offset += 1;
                } else {
                    break;
                }
            }
            offset
        } else {
            self.scroll_index_rec = Some(idx);
            return;
        };

        let win_lines = self.get_window_lines();
        let item_len = self.get_items_len() as i32;
        let mut scroll_to = (idx as i32 + idx_offset).max(0).min(item_len - win_lines);
        scroll_to *= 10;

        let scrolled = self.scroll_to(scroll_to);

        if scrolled {
            emit!(self, internal_scroll_value_changed(scroll_to))
        }
    }

    #[inline]
    pub(crate) fn check_lock(&mut self) {
        let mut f = false;

        if let Some(mutex) = self.concurrent_store.try_lock() {
            let new_len = mutex.len();

            if self.len_rec != new_len {
                self.len_rec = new_len;
                emit!(self, items_len_changed(new_len));

                f = true;
            }
        }

        if !f {
            return;
        }

        if let Some(idx) = self.scroll_index_rec.take() {
            self.scroll_to_index(idx);
        }
    }

    #[inline]
    pub(crate) fn occupied(&self) -> bool {
        self.concurrent_store.is_locked()
    }

    #[inline]
    pub(crate) fn initialize(&mut self) {
        Self::store_map().insert(self.id(), NonNull::new(self));
    }
}

impl Default for ListStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
