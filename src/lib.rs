extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr, ExprArray, ExprLit, Ident, Lit, LitStr, Token,
};

struct ExclusiveFeatures {
    name: Ident,
    features: Vec<String>,
    msg: LitStr,
}

impl Parse for ExclusiveFeatures {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let features: ExprArray = input.parse()?;
        input.parse::<Token![,]>()?;
        let msg: LitStr = input.parse()?;
        Ok(ExclusiveFeatures {
            name,
            features: features
                .elems
                .iter()
                .map(|e| {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = e
                    {
                        s.value()
                    } else {
                        panic!("Expected string literal")
                    }
                })
                .collect(),
            msg,
        })
    }
}

#[proc_macro]
pub fn cfg_exclusive(input: TokenStream) -> TokenStream {
    let ExclusiveFeatures {
        name,
        features,
        msg,
    } = parse_macro_input!(input);

    let mut permutations = vec![];

    for f in features.iter() {
        let mut cfg_exprs = vec![];

        for ef in features.iter().filter(|&x| x != f) {
            cfg_exprs.push(quote! {feature = #ef});
        }

        permutations.push(quote! {all(feature = #f, any(#(#cfg_exprs),*))});
    }

    TokenStream::from(quote! {
        fn #name() {
            #[cfg(any(#(#permutations),*))]
            compile_error!(#msg);
        }
    })
}
