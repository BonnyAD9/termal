use proc_macro::TokenStream;

#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    match termal_core::proc::colorize(input.into()) {
        Ok(r) => r.into(),
        Err(r) => r.to_stream().into(),
    }
}
