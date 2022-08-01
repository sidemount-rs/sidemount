use std::sync::Arc;

pub trait Middleware: Sync + Send + 'static {
    fn call(&self);
}

/// Default middleware implementation for a function
///
/// ## Examples
/// ```rust
/// use sidemount::Middleware;
///
/// fn assert_impl_middleware(f: impl Middleware) {}
///
/// fn test() {}
/// fn test2() {}
///
/// assert_impl_middleware(test);
/// assert_impl_middleware(test2);
/// ```
impl<Func> Middleware for Func
where
    Func: Fn() + Send + Sync + 'static,
{
    fn call(&self) {
        (self)();
    }
}

/// Default middleware implementation for a tuple of middleware.
///
/// ## Examples
/// ```rust
/// use sidemount::Middleware;
///
/// fn assert_impl_middleware(f: impl Middleware) {}
///
/// fn test() {}
/// fn test2() {}
///
/// assert_impl_middleware((test, test2));
/// ```
macro_rules! ary {
    ($($name:ident)+) => (
        impl<$($name),*> Middleware for ($($name,)*)
            where $($name: Middleware),*
        {
            #[allow(non_snake_case)]
            fn call(&self) {
                let ($(ref $name,)*) = *self;
                $(
                    $name.call();
                )*
            }
        }
    );
}

ary! { A B }
ary! { A B C }
ary! { A B C D }
ary! { A B C D E }
ary! { A B C D E F }
ary! { A B C D E F G }
ary! { A B C D E F G H }
ary! { A B C D E F G H I }
ary! { A B C D E F G H I J }
ary! { A B C D E F G H I J K }
ary! { A B C D E F G H I J K L }
ary! { A B C D E F G H I J K L M }
ary! { A B C D E F G H I J K L M N }
ary! { A B C D E F G H I J K L M N O }
ary! { A B C D E F G H I J K L M N O P }
ary! { A B C D E F G H I J K L M N O P Q }
ary! { A B C D E F G H I J K L M N O P Q R }
ary! { A B C D E F G H I J K L M N O P Q R S }
ary! { A B C D E F G H I J K L M N O P Q R S T }
ary! { A B C D E F G H I J K L M N O P Q R S T U }
ary! { A B C D E F G H I J K L M N O P Q R S T U V }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
ary! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }

/// Default middleware implementation for an Arc middleware.
///
/// ## Examples
/// ```rust
/// use std::sync::Arc;
/// use sidemount::Middleware;
///
/// fn assert_impl_middleware(f: impl Middleware) {}
///
/// fn test() {}
///
/// assert_impl_middleware(Arc::new(test));
/// ```
impl<T> Middleware for Arc<T>
where
    T: Middleware,
{
    fn call(&self) {
        self.as_ref().call();
    }
}
