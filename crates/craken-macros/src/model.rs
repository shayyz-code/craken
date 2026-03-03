use heck::ToSnakeCase;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

pub fn expand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Infer table name from struct name (PascalCase -> snake_case)
    let mut table_name = name.to_string().to_snake_case();

    // Allow override via #[table("custom_name")]
    for attr in &input.attrs {
        if attr.path().is_ident("table") {
            if let Ok(lit) = attr.parse_args::<LitStr>() {
                table_name = lit.value();
            }
        }
    }

    let expanded = quote! {
        #[automatically_derived]
        impl ::craken_database::model::Model for #name {
            fn table_name() -> &'static str {
                #table_name
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
