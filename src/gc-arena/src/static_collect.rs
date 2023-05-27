use crate::collect::Collect;

/// A wrapper type that implements Collect whenever the contained T is 'static, which is useful in
/// generic contexts
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct StaticCollect<T>(pub T);

unsafe impl<T: 'static> Collect for StaticCollect<T> {
    #[inline]
    fn needs_trace() -> bool {
        false
    }
}
