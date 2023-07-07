extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro_attribute]
pub fn use_injected(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();

    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("Expected a function!"),
    };

    let args_string = args.to_string();

    let args = args_string.split(",").map(|x| x.trim());

    for arg in args {
        let inject_arg = format_ident!("{}", arg.to_string());

        let capital_inject_str: String;

        let injected = arg.to_owned();

        if injected.contains("_") {
            let strs = injected.split("_");

            let mut complete = "".to_string();

            for item in strs {
                complete += &(item.to_string()[0..1].to_uppercase() + &item.to_string()[1..]);
            }

            capital_inject_str = complete;
        } else {
            capital_inject_str =
                injected.to_string()[0..1].to_uppercase() + &injected.to_string()[1..];
        }

        let capital_inject = format_ident!("{}", capital_inject_str);

        if !arg
            .chars()
            .all(|x| x.is_alphanumeric() || "_".chars().any(|y| y == x))
        {
            panic!("Injected dependency must be a valid variable name!");
        }

        let gen = quote!(mut #inject_arg: std::sync::MutexGuard<'_, crate::#capital_inject>);

        fn_item.sig.inputs.insert(
            {
                if fn_item.sig.inputs.len() == 0 {
                    0
                } else {
                    fn_item.sig.inputs.len()
                }
            },
            syn::parse(gen.into()).unwrap(),
        );
    }

    use quote::ToTokens;

    item.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn inject(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();

    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("Expected a function!"),
    };

    let args_string = args.to_string();

    let args = args_string.split(",").map(|x| x.trim());

    for arg in args {
        let inject_arg = format_ident!("{}", arg.to_string());

        let temp_inj = format_ident!("_{}", arg.to_string());

        let capital_inject_str: String;

        let injected = arg.to_owned();

        if injected.contains("_") {
            let strs = injected.split("_");

            let mut complete = "".to_string();

            for item in strs {
                complete += &(item.to_string()[0..1].to_uppercase() + &item.to_string()[1..]);
            }

            capital_inject_str = complete;
        } else {
            capital_inject_str =
                injected.to_string()[0..1].to_uppercase() + &injected.to_string()[1..];
        }

        let capital_inject = format_ident!("{}", capital_inject_str);

        if !arg
            .chars()
            .all(|x| x.is_alphanumeric() || "_".chars().any(|y| y == x))
        {
            panic!("Injected dependency must be a valid variable name!");
        }

        let gen = quote!(let mut #inject_arg = #temp_inj.lock().unwrap(););

        fn_item
            .block
            .stmts
            .insert(0, syn::parse(gen.into()).unwrap());

        let gen = quote!(let #temp_inj = #inject_arg.clone(););

        fn_item
            .block
            .stmts
            .insert(0, syn::parse(gen.into()).unwrap());

        let gen = quote!(#inject_arg: tauri::State<'_, std::sync::Arc<std::sync::Mutex<crate::#capital_inject>>>);

        fn_item.sig.inputs.insert(
            {
                if fn_item.sig.inputs.len() == 0 {
                    0
                } else {
                    fn_item.sig.inputs.len()
                }
            },
            syn::parse(gen.into()).unwrap(),
        );
    }

    use quote::ToTokens;

    item.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn inject_from_handle(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(input).unwrap();

    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("Expected a function!"),
    };

    let args_string = args.to_string();

    let args = args_string.split(",").map(|x| x.trim());

    for arg in args {
        let inject_arg = format_ident!("{}", arg.to_string());

        let temp_inj = format_ident!("{}cc", arg.to_string());

        let capital_inject_str: String;

        let injected = arg.to_owned();

        if injected.contains("_") {
            let strs = injected.split("_");

            let mut complete = "".to_string();

            for item in strs {
                complete += &(item.to_string()[0..1].to_uppercase() + &item.to_string()[1..]);
            }

            capital_inject_str = complete;
        } else {
            capital_inject_str =
                injected.to_string()[0..1].to_uppercase() + &injected.to_string()[1..];
        }

        let capital_inject = format_ident!("{}", capital_inject_str);

        if !arg
            .chars()
            .all(|x| x.is_alphanumeric() || "_".chars().any(|y| y == x))
        {
            panic!("Injected dependency must be a valid variable name!");
        }

        let gen = quote!(let mut #inject_arg = #temp_inj.lock().unwrap(););

        fn_item
            .block
            .stmts
            .insert(0, syn::parse(gen.into()).unwrap());

        let gen = quote!(
            let mut #temp_inj = app.state::<std::sync::Arc<std::sync::Mutex<crate::#capital_inject>>>().clone();
        );

        fn_item
            .block
            .stmts
            .insert(0, syn::parse(gen.into()).unwrap());
    }

    use quote::ToTokens;

    item.into_token_stream().into()
}
