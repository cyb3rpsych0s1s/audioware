use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_str, punctuated::Punctuated, spanned::Spanned, BinOp, Expr,
    ExprBinary, ExprLit, ExprPath, Lit, LitStr, Meta, MetaNameValue, Path, Token, Type,
};

const HINT_OFFSET: &str = r#"= hint: offset = 0x140975FE4 - 0x140000000
= hint: offset = 0x975FE4
= hint: offset = SOME_CONSTANT
"#;
const HINT_INPUTS: &str = "= hint: inputs = (CName, EntityId, CName)";
const HINT_EVENT: &str = "= hint: event = AudioEvent";
const HINT_HANDLER: &str = "= hint: handler = my_function where fn(usize)->Self::Event";

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

#[proc_macro_derive(NativeFunc, attributes(hook))]
pub fn derive_native_func(input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let derive = parse_macro_input!(input as syn::DeriveInput);
    let struc = parse_macro_input!(input2 as syn::ItemStruct);
    let private = Ident::new(
        &format!(
            "__internals_func_{}",
            struc.ident.to_string().to_lowercase().as_str()
        ),
        Span::call_site(),
    );
    let name = struc.ident;
    let mut offset: Option<proc_macro2::TokenStream> = None;
    let mut inputs: Option<Type> = None;
    let mut inputs_impl: Vec<proc_macro2::TokenStream> = vec![];
    let mut allow: Option<String> = None;
    let mut detour: Option<String> = None;
    for ref attr in derive.attrs {
        let meta = &attr.meta;
        match meta {
            Meta::List(list) if list.path.is_ident("hook") => {
                if let Ok(list) =
                    list.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)
                {
                    for arg in list {
                        match arg {
                            MetaNameValue { path, value, .. } if path.is_ident("offset") => {
                                match get_offset(&value) {
                                    Ok(x) => {
                                        offset = Some(x);
                                    }
                                    Err(e) => return e.to_compile_error().into(),
                                }
                            }
                            MetaNameValue {
                                path,
                                value:
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }),
                                ..
                            } if path.is_ident("inputs") => match get_inputs(&lit) {
                                Ok((ty, impls)) => {
                                    inputs = Some(ty);
                                    inputs_impl = impls;
                                }
                                Err(e) => return e.to_compile_error().into(),
                            },
                            MetaNameValue {
                                path,
                                value:
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }),
                                ..
                            } if path.is_ident("allow") => {
                                allow = Some(lit.value());
                            }
                            MetaNameValue {
                                path,
                                value:
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }),
                                ..
                            } if path.is_ident("detour") => {
                                detour = Some(lit.value());
                            }
                            _ => {
                                return syn::Error::new(arg.span(), "unknown or invalid attribute")
                                    .to_compile_error()
                                    .into()
                            }
                        }
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
    if inputs.is_none() || inputs_impl.is_empty() {
        return syn::Error::new(name.span(), "missing inputs attribute")
            .to_compile_error()
            .into();
    }
    if allow.is_none() {
        return syn::Error::new(name.span(), "missing allow attribute")
            .to_compile_error()
            .into();
    }
    if detour.is_none() {
        return syn::Error::new(name.span(), "missing detour attribute")
            .to_compile_error()
            .into();
    }
    let allow = Ident::new(allow.unwrap().as_str(), Span::call_site());
    let detour = Ident::new(detour.unwrap().as_str(), Span::call_site());
    let storage = quote! {
        mod #private {
            fn storage() -> &'static ::std::sync::RwLock<::std::option::Option<::retour::RawDetour>> {
                static INSTANCE: ::once_cell::sync::OnceCell<::std::sync::RwLock<::std::option::Option<::retour::RawDetour>>> = ::once_cell::sync::OnceCell::new();
                return INSTANCE.get_or_init(::std::default::Default::default)
            }
            pub(super) fn store(detour: ::std::option::Option<::retour::RawDetour>) {
                if detour.is_some() {
                    if let Ok(mut guard) = self::storage().try_write() {
                        *guard = detour;
                    } else {
                        ::red4ext_rs::error!("lock contention (store {})", stringify!(#name));
                    }
                } else {
                    if let Ok(mut guard) = self::storage().try_write() {
                        let _ = guard.take();
                    } else {
                        ::red4ext_rs::error!("lock contention (store {})", stringify!(#name));
                    }
                }
            }
            pub(super) fn trampoline(closure: ::std::boxed::Box<dyn ::std::ops::Fn(&::retour::RawDetour)>) {
                if let Ok(Some(guard)) = self::storage().try_read().as_deref() {
                    closure(guard);
                } else {
                    ::red4ext_rs::error!("lock contention (trampoline {})", stringify!(#name));
                }
            }
        }
        unsafe impl ::audioware_mem::DetourFunc for #name {
            const OFFSET: usize = #offset;
            type Inputs = #inputs;
            unsafe fn from_frame(frame: *mut red4ext_rs::ffi::CStackFrame) -> Self::Inputs {
                #(#inputs_impl)*
            }
        }
        impl ::audioware_mem::NativeFunc for #name {
            const HOOK: fn(Self::Inputs) -> () = #detour;
            const CONDITION: fn(&Self::Inputs) -> bool = #allow;
            const TRAMPOLINE: fn(Box<dyn Fn(&::retour::RawDetour)>) = #private::trampoline;
            const STORE: fn(Option<::retour::RawDetour>) = #private::store;
        }
    };
    quote! {
        #storage
    }
    .into()
}

fn get_offset(value: &Expr) -> Result<proc_macro2::TokenStream, syn::Error> {
    match value {
        // offset = 0x123
        Expr::Lit(ExprLit {
            lit: Lit::Int(lit), ..
        }) => {
            if let Ok(lit) = lit.base10_parse::<usize>() {
                return Ok(format_hex(lit)?.to_token_stream());
            }
            Err(syn::Error::new(
                value.span(),
                format!("unparseable offset attribute\n{}", HINT_OFFSET),
            ))
        }
        // offset = 0x456 - 0x123
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Sub(_),
            ..
        }) => {
            if let (
                Expr::Lit(ExprLit {
                    lit: Lit::Int(ref left),
                    ..
                }),
                Expr::Lit(ExprLit {
                    lit: Lit::Int(ref right),
                    ..
                }),
            ) = (left.deref(), right.deref())
            {
                if let (Ok(left), Ok(right)) =
                    (left.base10_parse::<usize>(), right.base10_parse::<usize>())
                {
                    return Ok(format_hex(left - right)?.to_token_stream());
                }
                return Err(syn::Error::new(
                    value.span(),
                    format!("unparseable offset attribute\n{}", HINT_OFFSET),
                ));
            }
            Err(syn::Error::new(
                value.span(),
                format!("unparseable offset attribute\n{}", HINT_OFFSET),
            ))
        }
        // offset = SOME_CONST
        Expr::Path(ExprPath { path, qself, .. }) => {
            if qself.is_none() {
                return Ok(path.to_token_stream());
            }
            Err(syn::Error::new(
                value.span(),
                "qualified path is not supported for offset attribute",
            ))
        }
        _ => Err(syn::Error::new(
            value.span(),
            format!("unparseable offset attribute\n{}", HINT_OFFSET),
        )),
    }
}

fn get_inputs(lit: &LitStr) -> Result<(Type, Vec<proc_macro2::TokenStream>), syn::Error> {
    let inputs: Type;
    let mut inputs_impl: Vec<proc_macro2::TokenStream>;
    if let Ok(value) = parse_str::<syn::Type>(lit.value().as_str()) {
        inputs = value.clone();
        match value {
            Type::Tuple(tuple) => {
                let mut args = vec![];
                inputs_impl = vec![];
                for (idx, elem) in tuple.elems.iter().enumerate() {
                    let arg: Ident = Ident::new(&format!("arg_{}", idx), Span::call_site());
                    args.push(arg.clone());
                    inputs_impl.push(quote!{
                    let mut #arg: #elem = <#elem>::default();
                    unsafe { ::red4ext_rs::ffi::get_parameter(frame, ::std::mem::transmute(&mut #arg)) };
                });
                }
                inputs_impl.push(quote! {
                    (#(#args),*)
                });
                return Ok((inputs, inputs_impl));
            }
            _ => {
                return Err(syn::Error::new(
                    value.span(),
                    format!("inputs attribute only supports tuple\n{HINT_INPUTS}"),
                ))
            }
        }
    }
    Err(syn::Error::new(
        lit.span(),
        format!("inputs attribute only supports tuple\n{HINT_INPUTS}"),
    ))
}

fn get_event(lit: &LitStr) -> Result<(Type, proc_macro2::TokenStream), syn::Error> {
    let event: Type;
    let event_impl: proc_macro2::TokenStream;
    if let Ok(value) = parse_str::<syn::Type>(lit.value().as_str()) {
        event = value.clone();
        match value {
            Type::Path(path) => {
                let path = path.path;
                event_impl = quote! {
                    let event: #path = <#path>::from_memory(event);
                    event
                };
                return Ok((event, event_impl));
            }
            _ => {
                return Err(syn::Error::new(
                    value.span(),
                    format!("event attribute only supports explicit type\n{HINT_EVENT}"),
                ))
            }
        }
    }
    Err(syn::Error::new(
        lit.span(),
        format!("event attribute only supports explicit type\n{HINT_EVENT}"),
    ))
}

fn get_handler(lit: &LitStr) -> Result<syn::Path, syn::Error> {
    if let Ok(value) = parse_str::<syn::Type>(lit.value().as_str()) {
        match value {
            Type::Path(path) => {
                return Ok(path.path);
            }
            _ => {
                return Err(syn::Error::new(
                    value.span(),
                    format!("handler attribute only supports explicit function\n{HINT_HANDLER}"),
                ))
            }
        }
    }
    Err(syn::Error::new(
        lit.span(),
        format!("handler attribute only supports explicit function\n{HINT_EVENT}"),
    ))
}

fn format_hex(lit: usize) -> Result<proc_macro2::Literal, syn::Error> {
    format!("{:#X}", lit)
        .parse::<proc_macro2::Literal>()
        .map_err(|_| syn::Error::new(lit.span(), "invalid hexadecimal"))
}

#[proc_macro_derive(NativeHandler, attributes(hook))]
pub fn derive_native_handler(input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let derive = parse_macro_input!(input as syn::DeriveInput);
    let struc = parse_macro_input!(input2 as syn::ItemStruct);
    let private = Ident::new(
        &format!(
            "__internals_handler_{}",
            struc.ident.to_string().to_lowercase().as_str()
        ),
        Span::call_site(),
    );
    let name = struc.ident;
    let mut offset: Option<proc_macro2::TokenStream> = None;
    let mut event: Option<Type> = None;
    let mut event_impl: Option<proc_macro2::TokenStream> = None;
    let mut detour: Option<String> = None;
    let mut handler: Option<Path> = None;
    for ref attr in derive.attrs {
        let meta = &attr.meta;
        match meta {
            Meta::List(list) if list.path.is_ident("hook") => {
                if let Ok(list) =
                    list.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)
                {
                    for arg in list {
                        match arg {
                            MetaNameValue { path, value, .. } if path.is_ident("offset") => {
                                match get_offset(&value) {
                                    Ok(x) => {
                                        offset = Some(x);
                                    }
                                    Err(e) => return e.to_compile_error().into(),
                                }
                            }
                            MetaNameValue {
                                path,
                                value:
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }),
                                ..
                            } if path.is_ident("event") => match get_event(&lit) {
                                Ok((ty, imp)) => {
                                    event = Some(ty);
                                    event_impl = Some(imp);
                                }
                                Err(e) => return e.to_compile_error().into(),
                            },
                            MetaNameValue {
                                path,
                                value:
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }),
                                ..
                            } if path.is_ident("detour") => {
                                detour = Some(lit.value());
                            }
                            MetaNameValue {
                                path,
                                value:
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }),
                                ..
                            } if path.is_ident("handler") => match get_handler(&lit) {
                                Ok(ty) => {
                                    handler = Some(ty);
                                }
                                Err(e) => return e.to_compile_error().into(),
                            },
                            _ => {
                                return syn::Error::new(arg.span(), "unknown or invalid attribute")
                                    .to_compile_error()
                                    .into()
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let handler = if let Some(handler) = handler {
        quote! { #handler(event) }
    } else {
        quote! { #event_impl }
    };
    let detour = Ident::new(detour.unwrap().as_str(), Span::call_site());
    let storage = quote! {
        mod #private {
            fn storage() -> &'static ::std::sync::RwLock<::std::option::Option<::retour::RawDetour>> {
                static INSTANCE: ::once_cell::sync::OnceCell<::std::sync::RwLock<::std::option::Option<::retour::RawDetour>>> = ::once_cell::sync::OnceCell::new();
                return INSTANCE.get_or_init(::std::default::Default::default)
            }
            pub(super) fn store(detour: ::std::option::Option<::retour::RawDetour>) {
                if detour.is_some() {
                    if let Ok(mut guard) = self::storage().try_write() {
                        *guard = detour;
                    } else {
                        ::red4ext_rs::error!("lock contention (store {})", stringify!(#name));
                    }
                } else {
                    if let Ok(mut guard) = self::storage().try_write() {
                        let _ = guard.take();
                    } else {
                        ::red4ext_rs::error!("lock contention (store {})", stringify!(#name));
                    }
                }
            }
            pub(super) fn trampoline(closure: ::std::boxed::Box<dyn ::std::ops::Fn(&::retour::RawDetour)>) {
                if let Ok(Some(guard)) = self::storage().try_read().as_deref() {
                    closure(guard);
                } else {
                    ::red4ext_rs::error!("lock contention (trampoline {})", stringify!(#name));
                }
            }
        }
        unsafe impl ::audioware_mem::DetourHandler for #name {
            const OFFSET: usize = #offset;
            type Event = #event;
            unsafe fn from_ptr(event: usize) -> Self::Event {
                #handler
            }
        }
        impl ::audioware_mem::NativeHandler for #name {
            const HOOK: fn(Self::Event) -> () = #detour;
            const TRAMPOLINE: fn(Box<dyn Fn(&::retour::RawDetour)>) = #private::trampoline;
            const STORE: fn(Option<::retour::RawDetour>) = #private::store;
        }
    };
    quote! {
        #storage
    }
    .into()
}
