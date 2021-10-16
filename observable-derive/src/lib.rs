use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Index, Type, parse_macro_input};

#[proc_macro_derive(Observable)]
pub fn derive_observable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let observable_struct = match data {
        Data::Struct(data) => ObservableStructImpl { ident, fields: data.fields },
        _ => todo!()
    };

    let output = quote! {
        #observable_struct
    };

    output.into()
}

struct ObservableStructImpl {
    ident: Ident,
    fields: Fields,
}

impl ToTokens for ObservableStructImpl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ObservableStructImpl { ident, fields } = self;

        let observable_ident = format_ident!("{}Observable", ident);

        let make_observable_ty = |ty: &Type| quote! {
            ::sycamore::reactive::Signal<<#ty as Observable>::Reflection>,
        };

        let observable_fields = fields.iter().map(|f| {
            let ty = &f.ty;
            let ty = match ty {
                Type::Tuple(tuple) => {
                    let tuple_elems = tuple.elems.iter().map(|ty| {
                        make_observable_ty(ty)
                    });
                    quote! {
                        (#(#tuple_elems)*),
                    }
                },
                _ => make_observable_ty(ty)
            };

            if let Some(ident) = &f.ident {
                quote! {
                    #ident: #ty
                }
            } else {
                quote! {
                    #ty
                }
            }
        });

        let is_tuple_like = matches!(fields, Fields::Unnamed(_));

        quote! {
            #[derive(Clone)]
        }.to_tokens(tokens);
        if is_tuple_like {
            quote! {
                struct #observable_ident(#(#observable_fields)*);
            }.to_tokens(tokens);
        } else {
            quote! {
                struct #observable_ident {
                    #(#observable_fields)*
                }
            }.to_tokens(tokens);
        }

        let make_into_reflection_field_ty = |ty: &Type, field: proc_macro2::TokenStream| quote! { ::sycamore::reactive::Signal::new(<#ty as Observable>::into_observable(#field)), };
        let into_reflection_fields = fields.iter().enumerate().map(|(i, f)| {
            let ty = &f.ty;
            let field_path = if is_tuple_like {
                let field_index = Index::from(i);
                quote! { self.#field_index }
            } else {
                let field_ident = f.ident.as_ref().unwrap();
                quote! { self.#field_ident }
            };

            let ty = match ty {
                Type::Tuple(tuple) => {
                    let tuple_elems = tuple.elems.iter().enumerate().map(|(j, ty)| {
                        let tuple_index = Index::from(j);
                        make_into_reflection_field_ty(ty, quote! { #field_path.#tuple_index })
                    });
                    quote! {
                        (#(#tuple_elems)*),
                    }
                },
                _ => {
                    make_into_reflection_field_ty(ty, field_path)
                }
            };

            if is_tuple_like {
                ty
            } else {
                let ident = f.ident.as_ref().unwrap();
                quote! {
                    #ident: #ty
                }
            }
        });

        let reflection_construction = if is_tuple_like {
            quote! {
                #observable_ident(#(#into_reflection_fields)*)
            }
        } else {
            quote! {
                #observable_ident {
                    #(#into_reflection_fields)*
                }
            }
        };

        quote! {
            impl Observable for #ident {
                type Reflection = #observable_ident;

                fn into_observable(self) -> Self::Reflection {
                    #reflection_construction
                }

                fn from_observable(other: Self::Reflection) -> Self {
                    todo!();
                }
            }
        }.to_tokens(tokens);
    }
}
