use syn::{spanned::Spanned, parse_macro_input, Data, DeriveInput, Fields};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn derive_server_packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name  = &input.ident;
    let resp  = derive_ser(&input.data);
    let id    = attr.to_string().parse::<u8>().unwrap();
    
    let expanded = quote!(
        #input

        impl ServerPacket for #name {
            fn ser(&self) -> Vec<u8> {
                let mut res = vec!(#id);
                #resp

                res
            }
        }
    );

    TokenStream::from(expanded)
}

fn derive_ser(data: &Data) -> TokenStream2 {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let res = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty   = &f.ty;

                        match &quote!(#ty).to_string()[..] {
                            "u8" => {
                                quote_spanned!(f.span() =>
                                    res.push(self.#name);
                                )
                            },
                            "f32" => {
                                quote_spanned!(f.span() =>
                                    res.extend_from_slice(&self.#name.to_le_bytes());
                                )
                            },
                            "u32" => {
                                quote_spanned!(f.span() =>
                                    res.extend_from_slice(&self.#name.to_le_bytes());
                                )
                            },
                            "&str" | "String" => {
                                quote_spanned!(f.span() =>
                                    res.extend_from_slice(&self.#name.as_bytes());
                                )
                            },
                            _ => panic!("Can not ser type: {}", quote!(#ty))
                        }
                    });

                    quote!(#(#res)*)
                },
                _ => panic!("Can only ser named fields.")
            }
        },
        _ => panic!("Can only ser struct.")
    }
}

#[proc_macro_attribute]
pub fn derive_client_packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let str_l = attr.to_string().parse::<usize>().unwrap();
    let name  = &input.ident;
    let resp  = derive_der(&input.data, str_l);
    
    let expanded = quote!(
        #input

        impl ClientPacket for #name {
            fn der(packet: &[u8]) -> Self {
                Self {
                    #resp
                }
            }
        }
    );

    TokenStream::from(expanded)
}

fn derive_der(data: &Data, str_l: usize) -> TokenStream2 {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let mut i: usize = 1;
                    let res = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty   = &f.ty;

                        match &quote!(#ty).to_string()[..] {
                            "u8" => {
                                let res = quote_spanned!(f.span() =>
                                    #name: packet[#i],
                                );
                                i += 1;
                                res
                            },
                            "f32" => {
                                let res = quote_spanned!(f.span() =>
                                    #name: f32::from_le_bytes(packet[#i..#i+4].try_into().unwrap()),
                                );
                                i += 4;
                                res
                            },
                            "u32" => {
                                let res = quote_spanned!(f.span() =>
                                    #name: u32::from_le_bytes(packet[#i..#i+4].try_into().unwrap()),
                                );
                                i += 4;
                                res
                            },
                            "&str" | "String" => {
                                if str_l == 0 {
                                    quote_spanned!(f.span() =>
                                        #name: packet[#i..].iter().filter_map(|&x| if x != 0 { Some(x as char) } else { None }).collect::<String>(),
                                    )
                                } else {
                                    let res = quote_spanned!(f.span() =>
                                        #name: packet[#i..#i+#str_l].iter().filter_map(|&x| if x != 0 { Some(x as char) } else { None }).collect::<String>(),
                                    );
                                    i += str_l;
                                    res
                                }
                            },
                            _ => panic!("Can not ser type: {}", quote!(#ty))
                        }
                    });

                    quote!(#(#res)*)
                },
                _ => panic!("Can only ser named fields.")
            }
        },
        _ => panic!("Can only ser struct.")
    }
}