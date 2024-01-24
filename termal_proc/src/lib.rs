use proc_macro::TokenStream;

/// Creates formatted and colorized string. Expands to call to a [`format!`]
/// macro.
#[proc_macro]
pub fn colorize(input: TokenStream) -> TokenStream {
    match termal_core::proc::colorize(input.into()) {
        Ok(r) => r.into(),
        Err(r) => r.to_stream().into(),
    }
}

/// Removes terminal commands from the string. Expands to call to a [`format!`]
/// macro.
#[proc_macro]
pub fn uncolor(input: TokenStream) -> TokenStream {
    match termal_core::proc::uncolor(input.into()) {
        Ok(r) => r.into(),
        Err(r) => r.to_stream().into(),
    }
}
