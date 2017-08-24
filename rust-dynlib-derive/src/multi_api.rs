use syn::{Body, VariantData, Field, Ty, MetaItem, Lit, DeriveInput};
use proc_macro::TokenStream;
use quote;
use super::common::{get_fields, symbol_name};

const TRATIT_NAME: &str = "WrapperMultiApi";

pub fn impl_wrapper_multi_api(ast: &DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let generics = &ast.generics;
    let fields = get_fields(ast, TRATIT_NAME);

    let tok_iter = fields.iter().map(field_to_tokens);
    let q = quote! {
        impl #generics WrapperMultiApi for #name #generics{}

         impl #generics ::dynlib::wrapper::WrapperApi for # name #generics{
            unsafe fn load(lib: & ::dynlib::raw::RawLib) -> Result<Self,::dynlib::Error> {
                Ok(#name {
                #(#tok_iter),*
                })
            }

        }
    };

    //panic!("{}", q.as_str());
    q
}


fn field_to_tokens(field: &Field) -> quote::Tokens {
    let field_name = &field.ident;

    //panic!("type_name = {}, {:?}", field_type_name, field);

    quote! {
        #field_name: ::dynlib::wrapper::WrapperApi::load(&lib)?
    }

}