use std::{borrow::BorrowMut, fs::{self, File}, io::Read};

use quote::quote;
use syn::Item;

fn main()->anyhow::Result<()> {

    // dbg!(tokens);
    // println!("{:?}", tokens);
    let mut file = File::open("/Users/kingzcheung/rust/bdzer/src/lib.rs")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut file = syn::parse_file(&content)?;
    parse_items(&mut file.items);
    let new_file = syn::File{
        shebang: file.shebang,
        attrs: file.attrs,
        items: file.items,
    };
    
    let output = quote! {
        #new_file
    };

    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    
    let dest_path = "./output/out.rs";
    fs::write(dest_path, formatted).unwrap();
    Ok(())
}


fn parse_items(items: &mut [syn::Item]) {
    items.iter_mut().for_each(|item:&mut Item| {
        if let syn::Item::Struct(is) = item.borrow_mut() {
            is.fields.iter_mut().for_each(|f| {
                f.attrs.push(syn::parse_quote! {
                    #[serde(rename = "id")]
                });
            });
        }
        
    });

}