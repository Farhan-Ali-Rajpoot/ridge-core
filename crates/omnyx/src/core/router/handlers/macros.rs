#[macro_export]
macro_rules! impl_handler {
    ( $trait:ident, $method:ident, $kind:ident; $($ty:ident),* ) => {
        #[allow(refining_impl_trait)]
        impl<F, Fut, R, $($ty,)*> $trait<($($ty,)*)> for F
        where
            F: Fn($($ty,)*) -> Fut + Clone + Send + Sync + 'static,
            Fut: std::future::Future<Output = R> + Send + 'static,
            R: $crate::core::router::io::IntoResponse + Send,
            // Updated FromContext constraint to be generic over your passed $kind
            $( $ty: $crate::core::router::handlers::FromContext<$kind> + Send, )*
        {
            fn $method(self, request: $crate::core::router::io::Request<$kind>) -> $crate::types::BoxFuture<$crate::core::router::io::Response> {
                std::boxed::Box::pin(async move {
                    $(
                        // Calls the exact type-safe extractor bound to this specific route kind
                        let $ty = <$ty as $crate::core::router::handlers::FromContext<$kind>>::from_request(&request).await;
                    )*

                    let result = (self)($($ty,)*).await;
                    result.into_response()
                })
            }
        }
    };
}
