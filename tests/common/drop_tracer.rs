use super::drop_item::DropItem;
use std::sync::Mutex;

pub struct DropTracer {
    count: Mutex<usize>,
}

impl DropTracer {
    pub fn new() -> Self {
        Self {
            count: Mutex::new(0),
        }
    }

    pub fn count(&self) -> usize {
        *self.count.lock().unwrap()
    }

    pub fn new_item(&self) -> DropItem {
        *self.count.lock().unwrap() += 1;
        DropItem::new(self)
    }

    pub(super) fn drop_item(&self) {
        *self.count.lock().unwrap() -= 1;
    }
}
