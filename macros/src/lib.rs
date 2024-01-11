use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Type};

#[proc_macro_derive(FromMemory)]
pub fn derive_from_memory(item: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident,
        generics,
        fields,
        attrs,
        ..
    } = parse_macro_input!(item as syn::ItemStruct);
    // check that struct has no generics / lifetimes
    assert_eq!(generics.params.len(), 0);
    // check that struct is no tuple
    assert!(fields.iter().all(|x| x.ident.is_some()));
    // check that struct is annotated with #[repr(C)]
    assert!(attrs.iter().any(|x| {
        x.path().is_ident("repr")
            && x.parse_nested_meta(|x| {
                if x.path.is_ident("C") {
                    Ok(())
                } else {
                    Err(x.error("struct must be annotated #[repr(C)]"))
                }
            })
            .is_ok()
    }));

    let mut field_name: Ident;
    let mut field_type: Type;
    let mut from_mem: Vec<proc_macro2::TokenStream> = vec![];
    let mut field_names: Vec<Ident> = vec![];
    for field in fields {
        field_name = field.ident.expect("already checked above");
        field_names.push(field_name.clone());
        field_type = field.ty;
        from_mem.push(quote! {
            let #field_name: #field_type = unsafe {
                ::core::slice::from_raw_parts::<#field_type>((address + ::memoffset::offset_of!(#ident, #field_name)) as *const #field_type, 1)
                .get_unchecked(0)
                .clone()
            };
        });
    }
    quote! {
        unsafe impl ::audioware_mem::FromMemory for #ident {
            #[allow(non_snake_case)]
            fn from_memory(address: usize) -> Self {
                #(#from_mem)*
                Self {
                    #(#field_names),*
                }
            }
        }
    }
    .into()
}
