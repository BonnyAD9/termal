use proc_macro::TokenStream;

#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    termal_core::proc::colorize(input.into()).into()
}
