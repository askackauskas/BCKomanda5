use std::sync::{Arc, Weak};

use educe::Educe;

// Based on <https://github.com/rust-lang/rfcs/issues/2564#issuecomment-429654108>.
// The explanation in that comment is misleading, however.
// The pointer is needed because `Weak::<T>::as_ptr` currently requires `T` to be `Sized`.
// The bound will be relaxed in the future (see <https://github.com/rust-lang/rust/pull/74160>).
#[derive(Clone, Educe)]
#[educe(PartialEq, Eq, Hash)]
pub struct WeakKey {
    // Use a trait object to eliminate the type parameter so we can store this in a global cache.
    // The trait object makes this a fat pointer despite the traits not having any methods.
    #[educe(PartialEq(ignore), Hash(ignore))]
    weak: Weak<dyn Send + Sync>,
    // Store the pointer as `usize` because `*const T` isn't `Send`.
    pointer: usize,
}

impl<T: Send + Sync + 'static> From<&Arc<T>> for WeakKey {
    fn from(arc: &Arc<T>) -> Self {
        // Fields of struct expressions are not coercion sites.
        let weak = Arc::downgrade(arc);
        let pointer = Arc::as_ptr(arc) as _;
        Self { weak, pointer }
    }
}
