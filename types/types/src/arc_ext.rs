use std::sync::Arc;

use easy_ext::ext;

#[ext(ArcExt)]
impl<T: Clone> Arc<T> {
    pub fn make_mut(&mut self) -> &mut T {
        Self::make_mut(self)
    }
}
