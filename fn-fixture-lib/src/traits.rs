use std::fmt::{
    Debug,
    Arguments,
};

use proc_macro2::{
    Span,
    TokenStream,
};

pub(super) trait IntoCompileError<T: Sized>: Sized {
    #[inline(always)]
    fn compile_err(self, msg: &str) -> Result<T, TokenStream> {
        IntoCompileError::compile_error(self, format_args!("{}", msg))
    }

    fn compile_error(self, msg: Arguments) -> Result<T, TokenStream>;
}

impl<T> IntoCompileError<T> for () {
    fn compile_error(self, msg: Arguments) -> Result<T, TokenStream> {
        Err(
            syn::Error::new(Span::call_site(), msg)
                .to_compile_error()
        )
    }
}

impl<T, E: Debug> IntoCompileError<T> for Result<T, E> {
    #[inline(always)]
    fn compile_error(self, msg: Arguments) -> Result<T, TokenStream> {
        self
            .map_err(|err|
                syn::Error::new(
                    Span::call_site(),
                    format_args!("{}: Err({:?})", msg, err),
                ).to_compile_error()
            )
    }
}

impl<T> IntoCompileError<T> for Option<T> {
    #[inline(always)]
    fn compile_error(self, msg: Arguments) -> Result<T, TokenStream> {
        self
            .ok_or_else(||
                syn::Error::new(Span::call_site(), msg)
                    .to_compile_error()
            )
    }
}

pub(super) trait EitherResult<T: Sized> {
    fn either(self) -> T;
}

impl<T> EitherResult<T> for Result<T, T> {
    #[inline(always)]
    fn either(self) -> T {
        match self {
            Ok(object) => object,
            Err(object) => object,
        }
    }
}
