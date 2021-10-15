use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type, parse_macro_input};

#[proc_macro_derive(Observable)]
pub fn derive_observable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let observable_struct = match data {
        Data::Struct(data) => process_struct(&ident, &data),
        _ => todo!()
    };

    let output = quote! {
        #observable_struct
    };

    output.into()
}

struct ObservableStruct {
    original_ident: Ident,
    tuple_like: bool,
    fields: Vec<ObservableField>,
}

impl ToTokens for ObservableStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ObservableStruct { original_ident, fields, tuple_like } = self;

        let observable_ident = format_ident!("{}Observable", original_ident);

        if *tuple_like {
            quote! {
                struct #observable_ident(#(#fields)*);
            }.to_tokens(tokens);
        } else {
            quote! {
                struct #observable_ident {
                    #(#fields)*
                }
            }.to_tokens(tokens);
        }

    }
}

fn process_struct(ident: &Ident, data_struct: &DataStruct) -> ObservableStruct {
    ObservableStruct {
        original_ident: ident.clone(),
        tuple_like: matches!(data_struct.fields, Fields::Unnamed(_) | Fields::Unit),
        fields: process_fields(&data_struct.fields),
    }
}

struct ObservableField {
    ident: Option<Ident>,
    original_type: Type,
}

impl ToTokens for ObservableField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ObservableField { ident, original_type } = self;

        let observable_type = quote! {::sycamore::reactive::Signal<<#original_type as Observable>::Reflection>};

        if let Some(ident) = ident {
            quote! {
                #ident: #observable_type,
            }.to_tokens(tokens);
        } else {
            quote! {
                #observable_type,
            }.to_tokens(tokens);
        }
    }
}

fn process_fields(fields: &Fields) -> Vec<ObservableField> {
    fields.into_iter().map(|f| {
        ObservableField {
            ident: f.ident.clone(),
            original_type: f.ty.clone(),
        }
    }).collect()
}
