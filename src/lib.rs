use proc_macro::TokenStream;
use rand::prelude::*;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut rng = rand::rng();
    /// A 1% chance to drop a token.
    const DROP_PERCENT_CHANCE: u32 = 1;
    tokens
        .into_iter()
        .filter(|_| !rng.random_ratio(DROP_PERCENT_CHANCE, 100))
        .collect()
}
