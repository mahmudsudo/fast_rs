use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, FnArg, ItemFn, LitStr, Pat, PatIdent, PatType, ReturnType, Type, parse_macro_input,
};

#[proc_macro_derive(OpenApi, attributes(validate))]
pub fn derive_openapi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut properties = Vec::new();
    let mut required = Vec::new();

    if let syn::Data::Struct(data) = input.data
        && let syn::Fields::Named(fields) = data.fields
    {
        for field in fields.named {
            let field_name = field.ident.unwrap();
            let field_name_str = field_name.to_string();
            required.push(field_name_str.clone());

            let ty = field.ty;

            let mut format_opt = quote! { None };
            let mut min_length_opt = quote! { None };

            for attr in field.attrs {
                if attr.path().is_ident("validate") {
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("email") {
                            format_opt = quote! { Some("email".to_string()) };
                        } else if meta.path.is_ident("length") {
                            let _ = meta.parse_nested_meta(|inner_meta| {
                                if inner_meta.path.is_ident("min") {
                                    let lit: syn::LitInt = inner_meta.value()?.parse()?;
                                    let val = lit.base10_parse::<usize>()?;
                                    min_length_opt = quote! { Some(#val) };
                                }
                                Ok(())
                            });
                        }
                        Ok(())
                    });
                }
            }

            properties.push(quote! {
                {
                    let mut schema = <#ty as fastrs::OpenApiType>::schema();
                    if let Some(f) = #format_opt { schema.format = Some(f); }
                    if let Some(m) = #min_length_opt { schema.min_length = Some(m); }
                    props.insert(#field_name_str.to_string(), schema);
                }
            });
        }
    }

    let expanded = quote! {
        impl fastrs::OpenApiType for #name {
            fn schema() -> fastrs::Schema {
                let mut props = std::collections::BTreeMap::new();
                #(#properties)*
                fastrs::Schema {
                    type_: Some("object".to_string()),
                    properties: props,
                    required: vec![ #( #required.to_string() ),* ],
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let path_str = path.value();
    let mut func = parse_macro_input!(item as ItemFn);
    let orig_name = func.sig.ident.clone();

    // We rename the inner function
    let inner_name = syn::Ident::new(&format!("__fastrs_inner_{}", orig_name), orig_name.span());
    func.sig.ident = inner_name.clone();

    // Collect argument types and path parameters
    let mut extractor_calls = Vec::new();
    let mut path_params = Vec::new();
    let mut state_ty: Option<Type> = None;

    for arg in &func.sig.inputs {
        if let FnArg::Typed(PatType { ty, pat, .. }) = arg {
            // Check if type is Path
            let mut is_path = false;
            if let Type::Path(type_path) = &**ty
                && let Some(segment) = type_path.path.segments.last()
            {
                if segment.ident == "Path" {
                    is_path = true;
                } else if segment.ident == "State"
                    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
                    {
                        state_ty = Some(inner_ty.clone());
                    }
            }

            if is_path {
                if let Pat::TupleStruct(tuple_pat) = &**pat
                    && let Some(Pat::Ident(PatIdent { ident, .. })) = tuple_pat.elems.first()
                {
                    let param_name = ident.to_string();
                    // wait, the inner type of Path<T>
                    if let Type::Path(type_path) = &**ty
                        && let Some(segment) = type_path.path.segments.last()
                        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
                        && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
                    {
                        path_params.push(quote! {
                            op.parameters.push(fastrs::Parameter {
                                name: #param_name.to_string(),
                                in_: "path".to_string(),
                                required: true,
                                schema: <#inner_ty as fastrs::OpenApiType>::schema(),
                            });
                        });
                    }
                }
            } else {
                extractor_calls.push(quote! {
                    <#ty as fastrs::OpenApiExtractor>::modify_operation(&mut op);
                });
            }
        }
    }

    let mut responder_calls = Vec::new();
    if let ReturnType::Type(_, ty) = &func.sig.output {
        responder_calls.push(quote! {
            <#ty as fastrs::OpenApiResponder>::modify_operation(&mut op);
        });
    }

    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let method_enum = syn::Ident::new(
        match method {
            "get" => "Get",
            "post" => "Post",
            "put" => "Put",
            "patch" => "Patch",
            "delete" => "Delete",
            _ => "Get",
        },
        proc_macro2::Span::call_site(),
    );

    let expanded = if let Some(state_ty) = state_ty {
        quote! {
            #[allow(non_camel_case_types)]
            pub fn #orig_name() -> fastrs::RouteDef<#state_ty> {
                #func

                let mut op = fastrs::Operation::default();
                #(#path_params)*
                #(#extractor_calls)*
                #(#responder_calls)*

                fastrs::RouteDef {
                    path: #path_str,
                    method: fastrs::Method::#method_enum,
                    router: fastrs::axum::routing::#method_ident(#inner_name),
                    operation: op,
                }
            }
        }
    } else {
        quote! {
            #[allow(non_camel_case_types)]
            pub fn #orig_name<S>() -> fastrs::RouteDef<S>
            where
                S: Clone + Send + Sync + 'static,
            {
                #func

                let mut op = fastrs::Operation::default();
                #(#path_params)*
                #(#extractor_calls)*
                #(#responder_calls)*

                fastrs::RouteDef {
                    path: #path_str,
                    method: fastrs::Method::#method_enum,
                    router: fastrs::axum::routing::#method_ident(#inner_name),
                    operation: op,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route("get", attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route("post", attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route("put", attr, item)
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route("patch", attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route("delete", attr, item)
}
