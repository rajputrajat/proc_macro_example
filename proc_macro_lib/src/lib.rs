use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    format!(
        r#"fn answer(a: i32, b: i32) -> String {{ "{:#?}".to_owned() }}"#,
        _item
    )
    .parse()
    .unwrap()
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer_derive() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper_1, helper_2, helper_3))]
pub fn derive_attr_answer(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

// following are from the books

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
