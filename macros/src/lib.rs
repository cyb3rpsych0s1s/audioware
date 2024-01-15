use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, parse_str, spanned::Spanned, Expr, ExprLit, Lit, Meta, MetaNameValue, Type,
};

/// automatically derive [`audioware_mem::FromMemory`] for any struct
/// with named fields which correctly upholds its invariants.
/// Failing to do so will lead to [undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#behavior-considered-undefined) at runtime.
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

#[proc_macro_derive(NativeFunc, attributes(offset, inputs, detour, should))]
pub fn derive_native_func(input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let derive = parse_macro_input!(input as syn::DeriveInput);
    let struc = parse_macro_input!(input2 as syn::ItemStruct);
    let private = Ident::new(
        &format!(
            "__internals_{}",
            struc.ident.to_string().to_lowercase().as_str()
        ),
        Span::call_site(),
    );
    let name = struc.ident;
    let mut offset: Option<usize> = None;
    let mut inputs: Option<syn::Type> = None;
    let mut inputs_impl: Vec<proc_macro2::TokenStream> = vec![];
    let mut detour: Option<String> = None;
    let mut should: Option<String> = None;
    for attr in derive.attrs {
        let meta = attr.meta;
        match meta {
            Meta::NameValue(MetaNameValue {
                ref path,
                ref value,
                ..
            }) if path.is_ident("offset") => {
                if let Expr::Lit(ExprLit { lit, .. }) = value {
                    if let Lit::Int(lit) = lit {
                        if let Ok(lit) = lit.base10_parse::<usize>() {
                            offset = Some(lit);
                        }
                    }
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path,
                ref value,
                ..
            }) if path.is_ident("inputs") => {
                if let Expr::Lit(ExprLit { lit, .. }) = value {
                    if let Lit::Str(lit) = lit {
                        if let Ok(value) = parse_str::<syn::Type>(lit.value().as_str()) {
                            inputs = Some(value.clone());
                            match value {
                                Type::Tuple(tuple) => {
                                    let mut args = vec![];
                                    for (idx, elem) in tuple.elems.iter().enumerate() {
                                        let arg: Ident =
                                            Ident::new(&format!("arg_{}", idx), Span::call_site());
                                        args.push(arg.clone());
                                        inputs_impl.push(quote!{
                                        let mut #arg: #elem = <#elem>::default();
                                        unsafe { ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut #arg)) };
                                    });
                                    }
                                    inputs_impl.push(quote! {
                                        (#(#args),*)
                                    })
                                }
                                _ => {
                                    return syn::Error::new(
                                        value.span(),
                                        "inputs attribute only supports tuple",
                                    )
                                    .to_compile_error()
                                    .into()
                                }
                            }
                        } else {
                            return syn::Error::new(value.span(), "invalid inputs attribute")
                                .to_compile_error()
                                .into();
                        }
                    }
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path,
                ref value,
                ..
            }) if path.is_ident("detour") => {
                if let Expr::Lit(ExprLit { lit, .. }) = value {
                    if let Lit::Str(lit) = lit {
                        detour = Some(lit.value());
                    }
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path,
                ref value,
                ..
            }) if path.is_ident("should") => {
                if let Expr::Lit(ExprLit { lit, .. }) = value {
                    if let Lit::Str(lit) = lit {
                        should = Some(lit.value());
                    }
                }
            }
            _ => {}
        }
    }
    if offset.is_none() {
        return syn::Error::new(name.span(), "missing offset attribute")
            .to_compile_error()
            .into();
    }
    if inputs.is_none() {
        return syn::Error::new(name.span(), "missing inputs attribute")
            .to_compile_error()
            .into();
    }
    if detour.is_none() {
        return syn::Error::new(name.span(), "missing detour attribute")
            .to_compile_error()
            .into();
    }
    if should.is_none() {
        return syn::Error::new(name.span(), "missing should attribute")
            .to_compile_error()
            .into();
    }
    let detour = Ident::new(detour.unwrap().as_str(), Span::call_site());
    let should = Ident::new(should.unwrap().as_str(), Span::call_site());
    let storage = quote! {
        mod #private {
            ::lazy_static::lazy_static! {
                static ref STORAGE: ::std::sync::Arc<::std::sync::Mutex<::std::option::Option<::retour::RawDetour>>> =
                    ::std::sync::Arc::new(::std::sync::Mutex::new(None));
            }
            pub(super) fn store(detour: ::std::option::Option<::retour::RawDetour>) {
                if let Ok(guard) = self::STORAGE.clone().try_lock().as_deref_mut() {
                    *guard = detour;
                }
            }
            pub(super) fn trampoline(closure: std::boxed::Box<dyn std::ops::Fn(&::retour::RawDetour)>) {
                if let Ok(Some(guard)) = self::STORAGE.clone().try_lock().as_deref() {
                    closure(guard);
                }
            }
        }
        impl ::audioware_mem::NativeFunc for #name {
            const OFFSET: usize = #offset;
            type Inputs = #inputs;
            const HOOK: fn(Self::Inputs) -> () = #detour;
            const CONDITION: fn(&Self::Inputs) -> bool = #should;
            const TRAMPOLINE: fn(Box<dyn Fn(&::retour::RawDetour)>) = self::#private::trampoline;
            const STORE: fn(Option<::retour::RawDetour>) = self::#private::store;
            unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs {
                #(#inputs_impl)*
            }
        }
    };
    quote! {
        #storage
    }
    .into()
}
