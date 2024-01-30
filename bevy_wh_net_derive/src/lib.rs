extern crate proc_macro;

use nameof::name_of;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Packet)]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet(&ast, false)
}

#[proc_macro_derive(PacketCore)]
pub fn derive_packet_core(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_packet(&ast, true)
}

#[proc_macro_attribute]
pub fn packet_to_server(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn packet_to_client(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

fn impl_packet(ast: &syn::DeriveInput, core: bool) -> TokenStream {
    let name = &ast.ident;

    let trait_path = match core {
        true => quote!(crate::common::PacketImpl),
        false => quote!(bevy_wh_net::common::PacketImpl),
    };

    let typetag_path = match core {
        true => quote!(crate::common::reexport::typetag::serde),
        false => quote!(bevy_wh_net::common::reexport::typetag::serde),
    };

    let mut send_to_server = false;
    let mut send_to_client = false;

    for attr in &ast.attrs {
        if !matches!(attr.style, syn::AttrStyle::Outer) {
            continue;
        };

        if attr.path().is_ident(name_of!(packet_to_server)) {
            send_to_server = true;
        } else if attr.path().is_ident(name_of!(packet_to_client)) {
            send_to_client = true;
        }
    }

    let gen = quote::quote! {
        #[#typetag_path]
        impl #trait_path for #name {
            fn can_send_to_client(&self) -> bool {
                #send_to_client
            }

            fn can_send_to_server(&self) -> bool {
                #send_to_server
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
