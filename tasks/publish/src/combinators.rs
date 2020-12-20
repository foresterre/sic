// Unlike and_then it expects the same in and output, namely T
pub trait ConditionallyDo<T, E> {
    fn do_if<B: FnOnce() -> bool, F: FnOnce(T) -> Result<T, E>>(
        self,
        condition: B,
        op: F,
    ) -> Result<T, E>
    where
        Self: Into<Result<T, E>>,
    {
        let execute = condition();

        match self.into() {
            Ok(t) if execute => op(t),
            Ok(t) => Ok(t),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> ConditionallyDo<T, E> for Result<T, E> {}
