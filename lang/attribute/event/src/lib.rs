extern crate proc_macro;

use quote::quote;
use syn::parse_macro_input;

/// A data structure representing a Solana account.
#[proc_macro_attribute]
pub fn event(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let event_strct = parse_macro_input!(input as syn::ItemStruct);

    let event_name = &event_strct.ident;

    let discriminator: proc_macro2::TokenStream = {
        let discriminator_preimage = format!("event:{}", event_name.to_string());
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(
            &anchor_syn::hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8],
        );
        format!("{:?}", discriminator).parse().unwrap()
    };

    proc_macro::TokenStream::from(quote! {
        #[derive(anchor_lang::EventIndex, AnchorSerialize, AnchorDeserialize)]
        #event_strct

        impl anchor_lang::EventData for #event_name {
            fn data(&self) -> Vec<u8> {
                let mut d = #discriminator.to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }
        }

        impl anchor_lang::Discriminator for #event_name {
            fn discriminator() -> [u8; 8] {
                #discriminator
            }
        }
    })
}

#[proc_macro]
pub fn emit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data: proc_macro2::TokenStream = input.into();
    proc_macro::TokenStream::from(quote! {
        {
            let data = anchor_lang::EventData::data(&#data);
            let msg_str = &anchor_lang::__private::base64::encode(data);
            anchor_lang::solana_program::msg!(msg_str);
        }
    })
}

// EventIndex is a marker macro. It functionally does nothing other than
// allow one to mark fields with the `#[index]` inert attribute, which is
// used to add metadata to IDLs.
#[proc_macro_derive(EventIndex, attributes(index))]
pub fn derive_event(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {})
}
