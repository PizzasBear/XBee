use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// trait ByteLen {
//     fn byte_len(&self) -> usize;
// }

// #[proc_macro_derive(Writable)]
// pub fn derive_writable(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let ident = input.ident;
//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
//
//     let trait_ident = quote!(Writable);
//
//     let expanded = match &input.data {
//         syn::Data::Struct(data) => {
//             let fields: Vec<_> = data
//                 .fields
//                 .iter()
//                 .enumerate()
//                 .map(|(i, field)| match &field.ident {
//                     Some(ident) => quote!(#ident),
//                     None => {
//                         let idx = syn::Index::from(i);
//                         quote!(#idx)
//                     }
//                 })
//                 .collect();
//             let fields_iter1 = fields.iter();
//             let fields_iter2 = fields.iter();
//
//             quote! {
//                 #[automatically_derived]
//                 impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
//                     fn byte_len(&self) -> usize {
//                         0 #(+ #trait_ident::byte_len(&self.#fields_iter1))*
//                     }
//                     fn write<F: FnMut(&[u8])>(&self, write_f: &mut F) {
//                         #(
//                             #trait_ident::write(&self.#fields_iter2, write_f);
//                         )*
//                     }
//                 }
//             }
//         }
//         syn::Data::Enum(data) => {
//             let fields: Vec<Vec<_>> = data
//                 .variants
//                 .iter()
//                 .map(|variant| {
//                     variant
//                         .fields
//                         .iter()
//                         .enumerate()
//                         .map(|(i, field)| match &field.ident {
//                             Some(ident) => ident.clone(),
//                             None => quote::format_ident!("a{i}"),
//                         })
//                         .collect()
//                 })
//                 .collect();
//
//             let variants: Vec<_> = data
//                 .variants
//                 .iter()
//                 .zip(fields.iter())
//                 .map(|(variant, fields)| {
//                     let ident = &variant.ident;
//                     let fields_iter = fields.iter();
//
//                     let var_fields = match &variant.fields {
//                         syn::Fields::Unnamed(_) => {
//                             quote!(Self::#ident(#(#fields_iter)*))
//                         }
//                         syn::Fields::Named(_) => {
//                             quote!(Self::#ident { #(#fields_iter)* })
//                         }
//                         syn::Fields::Unit => quote!(Self::#ident),
//                     };
//
//                     quote!(#var_fields)
//                 })
//                 .collect();
//
//             let byte_len_variants = variants.iter().zip(fields.iter()).map(|(variant, fields)| {
//                 let fields_iter = fields.iter();
//                 quote!(#variant => 0 #(+ #trait_ident::byte_len(&#fields_iter))*)
//             });
//
//             let write_variants = variants.iter().zip(fields.iter()).map(|(variant, fields)| {
//                 let fields_iter = fields.iter();
//                 quote! {
//                     #variant => {
//                         #(
//                             #trait_ident::write(&#fields_iter, write_f);
//                         )*
//                     }
//                 }
//             });
//
//             quote! {
//                 #[automatically_derived]
//                 impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
//                     fn byte_len(&self) -> usize {
//                         match self {
//                             #(#byte_len_variants)*
//                         }
//                     }
//                     fn write<F: FnMut(&[u8])>(&self, write_f: &mut F) {
//                         match self {
//                             #(#write_variants)*
//                         }
//                     }
//                 }
//             }
//         }
//         syn::Data::Union(_) => panic!("Cannot derive `Writable` trait for a `union`"),
//     };
//
//     TokenStream::from(expanded)
// }

fn make_generic_trait_bound(generics: &mut syn::Generics, traits: &[syn::Path]) {
    generics.make_where_clause();
    let mut where_clause = generics.where_clause.take().unwrap();
    let bounds: syn::punctuated::Punctuated<_, _> = traits
        .iter()
        .map(|trait_ident| {
            syn::TypeParamBound::Trait(syn::TraitBound {
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path: trait_ident.clone(),
            })
        })
        .collect();
    for ty in generics.type_params() {
        where_clause.predicates.push(
            syn::PredicateType {
                lifetimes: None,
                bounded_ty: syn::TypePath {
                    qself: None,
                    path: ty.ident.clone().into(),
                }
                .into(),
                colon_token: <syn::Token![:]>::default(),
                bounds: bounds.clone(),
            }
            .into(),
        )
    }
    generics.where_clause = Some(where_clause);
}

// #[proc_macro_derive(ExactInnerData)]
// pub fn derive_exact_inner_data(input: TokenStream) -> TokenStream {
//     let mut input = parse_macro_input!(input as DeriveInput);
//     let trait_ident = &quote::format_ident!("ExactInnerData");
//     let ident = input.ident;
//
//     make_generic_trait_bound(
//         &mut input.generics,
//         &[
//             quote::format_ident!("InnerData").into(),
//             trait_ident.clone().into(),
//         ],
//     );
//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
//
//     let expanded = match &input.data {
//         syn::Data::Struct(data) => {
//             let fields = data.fields.iter().map(|field| &field.ty);
//
//             quote! {
//                 #[automatically_derived]
//                 impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
//                     const BYTE_LEN: usize = 0 #(+ <#fields as #trait_ident>::BYTE_LEN)*;
//                 }
//             }
//         }
//         syn::Data::Enum(_) => panic!("Cannot derive `ExactInnerData` trait for an `enum`"),
//         syn::Data::Union(_) => panic!("Cannot derive `ExactInnerData` trait for a `union`"),
//     };
//
//     TokenStream::from(expanded)
// }

// #[derive(InnerData)]
// struct ApiFrame {
//     id: u8,
//     ieee_address: IeeeAddress,
//     network_address: NetworkAddress,
// }

#[proc_macro_derive(InnerData)]
pub fn derive_inner_data(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let trait_ident = &quote::format_ident!("InnerData");
    let ident = input.ident;

    make_generic_trait_bound(&mut input.generics, &[trait_ident.clone().into()]);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = match &input.data {
        syn::Data::Struct(data) => {
            let fields: Vec<_> = data
                .fields
                .iter()
                .enumerate()
                .map(|(i, field)| match &field.ident {
                    Some(ident) => quote!(#ident),
                    None => {
                        let idx = syn::Index::from(i);
                        quote!(#idx)
                    }
                })
                .collect();
            let field_types: &Vec<_> = &data.fields.iter().map(|field| &field.ty).collect();

            let fields_iter = &fields;

            let unique_ty_ident = (0..).find_map(|i| {
                let ident = if i == 0 {
                    quote::format_ident!("S")
                } else {
                    quote::format_ident!("S{i}")
                };
                if data
                    .fields
                    .iter()
                    .all(|field| field.ident.as_ref() != Some(&ident))
                {
                    Some(ident)
                } else {
                    None
                }
            });

            // field_types = [u8, IeeeAddress, NetworkAddress]

            quote! {
                #[automatically_derived]
                impl #impl_generics #trait_ident for #ident #ty_generics #where_clause {
                    const MIN_SIZE: usize = 0 #(+ <#field_types as #trait_ident>::MIN_SIZE)*;
                    const MAX_SIZE: Option<usize> = loop {
                        break Some(0 #(
                            + match <#field_types as #trait_ident>::MAX_SIZE {
                                Some(max_size) => max_size,
                                None => break None,
                            }
                        )*);
                    };

                    fn byte_size(&self) -> usize {
                        0 #(+ #trait_ident::byte_size(&self.#fields_iter))*
                    }
                    fn write<#unique_ty_ident: WriteStream>(&self, stream: &mut #unique_ty_ident) {
                        #(
                            #trait_ident::write(&self.#fields_iter, stream);
                        )*
                    }
                    fn read<#unique_ty_ident: ReadStream>(stream: &mut #unique_ty_ident, max_size: usize) -> Self {
                        let mut field_size = Self::MAX_SIZE
                                        .map_or(max_size, |c_max_size| c_max_size.min(max_size))
                                        .checked_sub(Self::MIN_SIZE)
                                        .expect("Called `InnerData::read` with `max_size` that is less than the minimum `InnerData::MIN_SIZE`");

                        Self {
                            #(#fields_iter: {
                                field_size += <#field_types as #trait_ident>::MIN_SIZE;
                                let value = <#field_types as #trait_ident>::read(stream, field_size);
                                field_size -= value.byte_size();
                                value
                            },)*
                        }
                    }
                }
            }
        }
        syn::Data::Enum(_) => panic!("Cannot derive `InnerData` trait for a `enum`"),
        syn::Data::Union(_) => panic!("Cannot derive `InnerData` trait for a `union`"),
    };

    TokenStream::from(expanded)
}
