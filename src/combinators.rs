pub trait FallbackIf<T, E> {
    fn fallback_if<P, F, V>(self, predicate: P, f: F, alternative: V) -> Result<T, E>
    where
        P: Into<bool>,
        F: FnOnce(V) -> Result<T, E>;
}

impl<T, E> FallbackIf<T, E> for Result<T, E> {
    /// Fallback to an alternative when a result produces an error and the predicate evaluates to true,
    /// otherwise keep the current result
    fn fallback_if<P, F, V>(self, predicate: P, f: F, alternative: V) -> Result<T, E>
    where
        P: Into<bool>,
        F: FnOnce(V) -> Result<T, E>,
    {
        if self.is_err() && predicate.into() {
            f(alternative)
        } else {
            self
        }
    }
}
