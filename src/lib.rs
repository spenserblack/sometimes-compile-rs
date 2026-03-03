use proc_macro::{Group, TokenStream, TokenTree};
use rand::prelude::*;

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut rng = rand::rng();
    filter_tokens(tokens, &mut rng)
}

fn filter_tokens(tokens: TokenStream, rng: &mut ThreadRng) -> TokenStream {
    tokens
        .into_iter()
        .filter_map(|token| filter_token(token, rng))
        .collect()
}

fn filter_token(token: TokenTree, rng: &mut ThreadRng) -> Option<TokenTree> {
    /// A 1% chance to drop a token.
    const DROP_PERCENT_RATIO: (u32, u32) = (1, 100);
    if rng.random_ratio(DROP_PERCENT_RATIO.0, DROP_PERCENT_RATIO.1) {
        None
    } else if let TokenTree::Group(group) = token {
        let delimiter = group.delimiter();
        let stream = filter_tokens(group.stream(), rng);
        let group = Group::new(delimiter, stream);
        let token = TokenTree::Group(group);
        Some(token)
    } else {
        Some(token)
    }
}
