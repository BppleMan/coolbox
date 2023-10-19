mod state;

use proc_macro::TokenStream;

#[proc_macro_derive(State)]
pub fn derive_state(item: TokenStream) -> TokenStream {
    state::derive_macro(item)
}
