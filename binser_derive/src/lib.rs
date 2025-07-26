use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, DeriveInput, Field};
use syn::Type;

fn get_type_ident(t: &Type) -> String {
    match t {
        Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
        _ => panic!("Unsupported type"),
    }
}

fn get_deserialized_fields(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = TokenStream2> {
    let mut deserialize_fields: Vec<TokenStream2> = Vec::new();

    for f in fields.iter()
    {
        let name = &f.ident;
        let ty = &f.ty;
        let ident = get_type_ident(ty);
        let token = match ident.as_str() {
            "u32" => {
                let tokens = quote! {
                    #name: (|| {
                        let end = *byte_offset + 4;
                        let result = u32::from_le_bytes(data[*byte_offset..end].try_into().unwrap());
                        *byte_offset = end;
                        result
                    })()
                };
                tokens
            },
            "String" => {                
                let tokens = quote! {
                    #name: (|| {
                        let end = *byte_offset + 4;
                        let len = u32::from_le_bytes(data[*byte_offset..end].try_into().unwrap()) as usize;
                        if len == 0
                        {
                            return Err(String::from("Failed to deserialize!"));
                        }
                        *byte_offset = end;
                        let str_bytes = &data[*byte_offset..*byte_offset + len];
                        *byte_offset += len;
                        String::from_utf8(str_bytes.to_vec()).unwrap()
                    })()
                };
                tokens
            },
            _ => {
                quote! {
                    #name: #ty::default(),
                }
            }
        };
        deserialize_fields.push(token);
    } 

    deserialize_fields.into_iter()   
}

#[proc_macro_derive(BinarySerializable)]
pub fn binary_serializable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(ref fields), .. }) = input.data {
        &fields.named
    } else {
        panic!("#[derive(BinarySerializable)] can only be used with structs with named fields");
    };

    let serialized_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let ident = get_type_ident(ty);

        match ident.as_str() {
            "u32" => {
                quote! {
                    buf.extend(&self.#name.to_le_bytes());
                }
            },
            "bool" => {
                quote! {
                    buf.push(self.#name as u8);
                }
            },
            "Vec<u8>" => {      
                quote! {
                    let vec_bytes = &self.#name;
                    buf.extend((vec_bytes.len() as u32).to_le_bytes());
                    buf.extend(vec_bytes);
                }
            },
            "String" => {
                quote! {
                    let str_bytes = self.#name.as_bytes();
                    buf.extend((str_bytes.len() as u32).to_le_bytes());
                    buf.extend(str_bytes);
                }
            },
            _ => {
                quote! {
                    compile_error!(concat!("Unsupported field type: ", stringify!(#ty)));
                }
            }
        }
    });

    let deserialize_fields = get_deserialized_fields(fields);

    let expanded = quote! {
        impl BinarySerializable for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut buf = Vec::new();
                #(#serialized_fields)*
                buf
            }

            fn deserialize(data: &[u8], byte_offset: &mut usize) -> Result<Self, String> where Self: Sized {
                Ok(Self {
                    #(#deserialize_fields),*
                })
            }

        }
    };

    println!("{}", expanded.to_string());

    TokenStream::from(expanded)
}
