// This is my first ever procedural macro.
// - Lucas Marta

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, Fields};

/// This procedural macro is used for extracting globals in files such as package manager 
/// configuration files and service manager configuration files. The reason for seperating this into
/// a macro is to make implementing new features for those configruations consistent and simpler.
/// 
/// For more information check out the `package_managers` and `service_managers` direcrory with
/// several lua configuration file examples.
#[proc_macro_derive(FromLuaFile)]
pub fn derive_from_lua_file(input: TokenStream) -> TokenStream {
    // Extract struct tokens
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    
    // Ensure that the deriving struct is indeed a struct and has named fields.
    let fields = match &input.data { 
        Data::Struct(data) => {
            match &data.fields { 
                Fields::Named(fields) => &fields.named,
                _ => panic!("FromLuaFile can only be derived for structs with named fields."),
            }
        },
        _ => panic!("FromLuaFile can only be derived for structs.")
    };
    
    let field_extractions = fields.iter().map(|field| {
        // Get the field identifier...
        let field_name = &field.ident;
        // and the name as a string
        let field_name_str = field_name.as_ref().unwrap().to_string();
        
        // Check if the field type is a `Vec<String>`
        let is_vec_string = if let syn::Type::Path(type_path) = &field.ty {
            let type_str = quote! { #type_path }.to_string();
            type_str.contains("Vec < String >") || type_str.contains("Vec<String>")
        } else {
            false
        };
        
        // Code to write depending if we are extracting a list or a string
        if is_vec_string {
            quote! {
                let mut #field_name: Vec<String> = vec![];
                
                if let Ok(value) = globals.get::<mlua::Value>(#field_name_str) {
                    if let Some(table) = value.as_table() {
                        #field_name = table.sequence_values::<String>()
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|e| anyhow::anyhow!("{}", e))?;
                    }
                }
            }
        } else {
            quote! {
                let #field_name = globals.get(#field_name_str).map_err(|e| anyhow::anyhow!("{}", e))?;
            }
        }
    });
    
    // Create the field assignments
    let field_assignments = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name }
    });
    
    TokenStream::from(quote! {
        impl crate::from_file::FromFile for #name {
            fn from_file(path: &std::path::PathBuf) -> anyhow::Result<Self> {
                let lua = mlua::Lua::new();
                
                if !path.exists() {
                    return Err(anyhow::anyhow!("Configuration file: \"{}\" does not exist", path.display()));
                }
                
                let config_script = std::fs::read_to_string(path)?;
                
                let globals = lua.globals();
                crate::include_custom_runtime!(lua, globals);
                
                lua.load(&config_script).exec().map_err(|e| anyhow::anyhow!("Failed to interpret configuration file: {}", e))?;
                
                #(#field_extractions)*
                
                Ok(#name {
                    #(#field_assignments),*
                })
            }
            
            fn get_binary_name(&self) -> &str {
                &self.binary_name
            }
        }
    })
}