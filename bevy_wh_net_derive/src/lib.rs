extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Packet)]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet(&ast)
}

fn impl_packet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut to_server = false;
    let mut to_client = false;

    for attr in &ast.attrs {
        if !matches!(attr.style, syn::AttrStyle::Outer) {
            continue;
        };

        if attr.path().is_ident("to_server") {
            to_server = true;
        } else if attr.path().is_ident("to_client") {
            to_client = true;
        }
    }

    let gen = quote::quote! {
        #[typetag::serde]
        impl bevy_wh_net::Packet for #name {
            fn can_send_to_client(&self) -> bool {
                #to_client
            }

            fn can_send_to_server(&self) -> bool {
                #to_server
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
