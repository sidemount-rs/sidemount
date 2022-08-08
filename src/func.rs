pub trait Func<Args, T> {
    type Output;
    fn call(&self, args: Args) -> Self::Output;
}

// Default implementation of a func for T as output
impl<A, B, Args, T> Func<Args, ()> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Fn<T>,
{
    type Output = B::Output;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call(args)
    }
}

// Subset of (A, B) T is (T,)
impl<A, B, Args, T> Func<Args, (T,)> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Fn<(T,)>,
{
    type Output = B::Output;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call((args,))
    }
}

// Subset of (A, B) where A is already a tuple that implements Func
impl<A, B, Args, T, F> Func<Args, ((), (), F)> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Func<T, F>,
{
    type Output = B::Output;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call(args)
    }
}

// Subset of (A, B) where is A is Func and B takes (T,)
impl<A, B, Args, T, F> Func<Args, ((), (T,), F)> for (A, B)
where
    A: Fn<Args, Output = T>,
    B: Func<(T,), F>,
{
    type Output = B::Output;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        let args = self.0.call(args);
        self.1.call((args,))
    }
}

// Default implementation for a func where T is a result
impl<A, B, Args, T, R, T2, E> Func<Args, (Result<T, E>, (), T2)> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<T, Output = R>,
    R: Into<Result<T2, E>>,
{
    type Output = Result<T2, E>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => self.1.call(args).into(),
            Err(e) => Err(e),
        }
    }
}

// Subset of (A, B) T is (T,)
impl<A, B, Args, T, R, T2, E> Func<Args, (Result<T, E>, ((),), T2)> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Fn<(T,), Output = R>,
    R: Into<Result<T2, E>>,
{
    type Output = Result<T2, E>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => self.1.call((args,)).into(),
            Err(e) => Err(e),
        }
    }
}

// Subset of (A, B) where A is already a tuple that implements Func
impl<A, B, Args, T, E, F, R, T2> Func<Args, (Result<T, E>, (), F, T2)> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Func<T, F, Output = R>,
    R: Into<Result<T2, E>>,
{
    type Output = Result<T2, E>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => self.1.call(args).into(),
            Err(e) => Err(e),
        }
    }
}

// Subset of (A, B) where is A is Func and B takes (T,)
impl<A, B, Args, T, E, F, R, T2> Func<Args, (Result<T, E>, ((),), F, T2)> for (A, B)
where
    A: Fn<Args, Output = Result<T, E>>,
    B: Func<(T,), F, Output = R>,
    R: Into<Result<T2, E>>,
{
    type Output = Result<T2, E>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Ok(args) => self.1.call((args,)).into(),
            Err(e) => Err(e),
        }
    }
}

// Default implementation for a func where T is an option
impl<A, B, Args, T> Func<Args, (Option<T>, ())> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Fn<T>,
{
    type Output = Option<B::Output>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call(args)),
            None => None,
        }
    }
}

// Subset of (A, B) T is (T,)
impl<A, B, Args, T> Func<Args, (Option<T>, ((),))> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Fn<(T,)>,
{
    type Output = Option<B::Output>;

    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call((args,))),
            None => None,
        }
    }
}

// Subset of (A, B) where A is already a tuple that implements Func
impl<A, B, Args, T, F> Func<Args, (Option<T>, (), F)> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Func<T, F>,
{
    type Output = Option<B::Output>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call(args)),
            None => None,
        }
    }
}

// Subset of (A, B) where is A is Func and B takes (T,)
impl<A, B, Args, T, F> Func<Args, (Option<T>, ((),), F)> for (A, B)
where
    A: Fn<Args, Output = Option<T>>,
    B: Func<(T,), F>,
{
    type Output = Option<B::Output>;
    #[inline]
    fn call(&self, args: Args) -> Self::Output {
        match self.0.call(args) {
            Some(args) => Some(self.1.call((args,))),
            None => None,
        }
    }
}

pub struct Function<F, T>(pub F, pub std::marker::PhantomData<T>);

impl<F, Args, T> Fn<Args> for Function<F, T>
where
    F: Func<Args, T>,
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        self.0.call(args)
    }
}

impl<F, Args, T> FnMut<Args> for Function<F, T>
where
    F: Func<Args, T>,
{
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.0.call(args)
    }
}

impl<F, Args, T> FnOnce<Args> for Function<F, T>
where
    F: Func<Args, T>,
{
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.0.call(args)
    }
}

#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        ($head, compose!($($tail),+))
    };
}

#[macro_export]
macro_rules! func {
    ( $head:expr, $($tail:expr), +) => {
        crate::func::Function(($head, crate::compose!($($tail),+)), std::marker::PhantomData::default())
    };
}
