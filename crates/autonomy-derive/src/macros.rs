
macro_rules! parse_error {
    (tokens, $span:expr, $msg:literal) => { syn::Error::new_spanned($span, $msg).to_compile_error() };
    (err, $span:expr, $msg:literal) => { Err(syn::Error::new_spanned($span, $msg)) };
}

pub(crate) use parse_error;

