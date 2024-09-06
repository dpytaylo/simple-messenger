// Taken from https://stackoverflow.com/questions/29963449/golang-like-defer-in-rust
pub(crate) struct ScopeCall<F: FnOnce()> {
    pub c: Option<F>,
}

impl<F: FnOnce()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}

macro_rules! expr {
    ($e: expr) => {
        $e
    };
} // tt hack

macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = crate::utils::defer::ScopeCall {
            c: Some(|| -> () { crate::utils::defer::expr!({ $($data)* }) })
        };
    )
}

pub(crate) use defer;
pub(crate) use expr;
