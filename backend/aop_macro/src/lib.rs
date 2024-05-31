extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn transactional(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_return_type = &input.sig.output;
    let fn_inputs = &input.sig.inputs;
    let fn_vis = &input.vis;
    let fn_asyncness = &input.sig.asyncness;

    let gen = quote! {
        #fn_vis #fn_asyncness fn #fn_name(#fn_inputs) #fn_return_type {
            tracing::info!("Starting transaction...");
            let pool = GlobalPool::get("main").unwrap().pool();
            let mut tx = match pool.begin().await {
                Ok(tx) => tx,
                Err(err) => {
                    tracing::error!("Failed to start transaction: {}", err);
                    return Err(err.into());
                }
            };
            // println!("{}", #fn_block);
            // Call the original function and capture the result
            let block = (async move #fn_block);

            let result = block.await;

            if result.is_ok() {
                if let Err(err) = tx.commit().await {
                    tracing::error!("Failed to commit transaction: {}", err);
                    return Err(err.into());
                }
            } else {
                if let Err(err) = tx.rollback().await {
                    tracing::error!("Failed to rollback transaction: {}", err);
                    return Err(err.into());
                }
            }
            tracing::info!("Ending transaction...");
            // Return the original function's result
            result
        }
    };

    gen.into()

    // gen.into()
}
