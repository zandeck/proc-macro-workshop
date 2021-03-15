use proc_macro2::{Span, Ident, TokenStream};
use syn::{Data, DeriveInput, Fields, parse_macro_input, spanned::Spanned};
use quote::{quote, quote_spanned};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", &name), Span::call_site());
    let data = input.data;   
    let functions = generate_functions(&data);


    let expanded = quote! {

        pub struct #builder_name {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

         impl #builder_name {
            #functions
        }

        impl #name {
            fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None
                }
            }
        }

   };

    proc_macro::TokenStream::from(expanded)
}

fn generate_functions(data: &Data) -> TokenStream {
    match *data {
       Data::Struct(ref data) => 
                match data.fields {
                    Fields::Named(ref fields) => {
                        let functions = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            let ty = &f.ty;
                            quote_spanned! {f.span() => 
                                pub fn #name(&mut self, value: #ty) -> &mut Self {
                                    self.#name = Some(value);
                                    self
                                }
                            }
                        });
                        quote! {
                            #(#functions)*
                        }
                    },
                    _ => unimplemented!()
            
       },
       _ => unimplemented!()
    }
}
