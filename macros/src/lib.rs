use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_str, punctuated::Punctuated, spanned::Spanned, BinOp, Expr,
    ExprBinary, ExprLit, ExprPath, Lit, LitStr, Meta, MetaNameValue, Token, Type,
};

const HINT_OFFSET: &str = r#"= hint: offset = 0x140975FE4 - 0x140000000
= hint: offset = 0x975FE4
= hint: offset = SOME_CONSTANT
"#;
const HINT_INPUTS: &str = "= hint: inputs = (CName, EntityId, CName)";

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

/// automatically derive [`audioware_mem::NativeFunc`] for a given struct
/// which already implements [`audioware_mem::Detour`].
///
/// # Examples
///
/// Here's an example on how to detour [AudioSystem::Play](https://jac3km4.github.io/cyberdoc/#33326)
/// whose signature is:
///
/// ```swift
/// public native func Play(eventName: CName, opt entityID: EntityID, opt emitterName: CName) -> Void
/// ```
///
/// Here's how:
///
/// ```
/// # use audioware_macros::NativeFunc;
/// # use red4ext_rs::types::{CName, EntityId};
///
/// #[derive(NativeFunc)]
/// #[hook(
///     // memory offset
///     offset = 0x975FE4,
///     // function input parameters
///     inputs = "(CName, EntityId, CName)",
///     // control wheter to allow detouring on each call
///     allow = "allow",
///     // custom detouring logic
///     detour = "detour"
/// )]
/// pub struct AudioSystemPlay;
/// # #[allow(unused_variables)]
/// fn detour(params: (CName, EntityId, CName)) {}
/// # #[allow(unused_variables)]
/// fn allow(params: &(CName, EntityId, CName)) -> bool { false }
/// ```
#[proc_macro_derive(NativeFunc, attributes(hook))]
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
            ::lazy_static::lazy_static! {
                static ref STORAGE: ::std::sync::Arc<::std::sync::Mutex<::std::option::Option<::retour::RawDetour>>> =
                    ::std::sync::Arc::new(::std::sync::Mutex::new(None));
            }
            pub(super) fn store(detour: ::std::option::Option<::retour::RawDetour>) {
                if let Ok(guard) = self::STORAGE.clone().try_lock().as_deref_mut() {
                    *guard = detour;
                } else {
                    ::red4ext_rs::error!("lock contention (store)");
                }
            }
            pub(super) fn trampoline(closure: std::boxed::Box<dyn std::ops::Fn(&::retour::RawDetour)>) {
                if let Ok(Some(guard)) = self::STORAGE.clone().try_lock().as_deref() {
                    closure(guard);
                } else {
                    ::red4ext_rs::error!("lock contention (trampoline)");
                }
            }
        }
        unsafe impl ::audioware_mem::Detour for #name {
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

fn format_hex(lit: usize) -> Result<proc_macro2::Literal, syn::Error> {
    format!("{:#X}", lit)
        .parse::<proc_macro2::Literal>()
        .map_err(|_| syn::Error::new(lit.span(), "invalid hexadecimal"))
}
