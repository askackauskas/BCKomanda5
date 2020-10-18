use ssz_new::types::FixedVector;
use typenum::Unsigned;

// `FixedVector::<_, N>::default()` returns an empty vector. It appears to be a bug.
#[must_use]
pub fn default<T: Default + Clone, N: Unsigned>() -> FixedVector<T, N> {
    FixedVector::from_elem(T::default())
}
