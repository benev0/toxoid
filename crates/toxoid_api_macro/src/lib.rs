#![recursion_limit = "256"]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{
    parse::{Parse, ParseStream, Parser}, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, FieldsNamed, Ident, ItemFn, Type, Stmt, Token
};

#[repr(u8)]
enum FieldType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Pointer
}

// The input to the macro will be a list of field names and types.
struct ComponentStruct {
    name: Ident,
    fields: FieldsNamed,
}
 
// Implement the parsing functionality.
impl Parse for ComponentStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let fields = input.parse()?;
        Ok(ComponentStruct { name, fields })
    }
}
 
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let components = Punctuated::<ComponentStruct, Comma>::parse_terminated
        .parse(input)
        .unwrap();
    let expanded = components
        .into_iter()
        .map(|component| {
            let ComponentStruct { name, fields } = component;
            let fields: Vec<_> = fields.named.iter().collect();

            let field_names = fields.iter().map(|f| &f.ident);
            let field_types = fields.iter().map(|f| &f.ty);

            fn align_offset(offset: u32, align: u32) -> u32 {
                (offset + align - 1) & !(align - 1)
            }
            let mut current_offset = 0;
            let fields_offsets = fields.iter().map(
                |field| {
                    let field_type = &field.ty;
                    let size = get_type_size(field_type);
                    let align = get_type_alignment(field_type);
                    current_offset = align_offset(current_offset, align);
                    let offset = current_offset;
                    current_offset += size;
                    offset
                }
            );

            let getters_and_setters =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .zip(fields_offsets)
                    .map(|((field_name, field_type), field_offset)| {
                        let getter_name = Ident::new(
                            &format!("get_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        let setter_name = Ident::new(
                            &format!("set_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        let field_type_str = format!("{}", quote!(#field_type));
                        match () {
                            _ if field_type_str == "u8" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u8(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: u8) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u8(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u16" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u16(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: u16) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u16(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u32(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: u32) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u32(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: u64) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i8" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_i8(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: i8) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_i8(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i16" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_i16(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: i16) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_i16(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_i32(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: i32) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_i32(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_i64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: i64) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_i64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "f32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_f32(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: f32) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_f32(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "f64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_f64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: f64) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_f64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "bool" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_bool(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: bool) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_bool(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "PointerT" => {
                                quote! {
                                    pub fn #getter_name(&self) -> PointerT {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: EcsEntityT) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "EcsEntityT" => {
                                quote! {
                                    pub fn #getter_name(&self) -> EcsEntityT {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u64(#field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&self, value: u64) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "String" => {
                                quote! {
                                    pub fn #getter_name(&self) -> String {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_string(#field_offset)
                                        }
                                    }
                                    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                                    pub fn #setter_name(&self, value: String) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_string(#field_offset, value.as_str());
                                        }
                                    }
                                    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
                                    pub fn #setter_name(&self, value: String) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_string(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "Vec :: < u8 >" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Vec<u8> {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u8list(#field_offset)
                                        }
                                    }
                                    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                                    pub fn #setter_name(&self, value: Vec<u8>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u8list(#field_offset, value.as_slice());
                                        }
                                    }
                                    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
                                    pub fn #setter_name(&self, value: Vec<u8>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u8list(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "Vec :: < u64 >" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Vec<u64> {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u64list(#field_offset)
                                        }
                                    }
                                    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                                    pub fn #setter_name(&self, value: Vec<u64>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64list(#field_offset, value.as_slice());
                                        }
                                    }
                                    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
                                    pub fn #setter_name(&self, value: Vec<u64>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64list(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "Vec :: < PointerT >" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Vec<PointerT> {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u64list(#field_offset)
                                        }
                                    }
                                    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                                    pub fn #setter_name(&self, value: Vec<PointerT>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64list(#field_offset, value.as_slice());
                                        }
                                    }
                                    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
                                    pub fn #setter_name(&self, value: Vec<PointerT>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64list(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "Vec :: < EcsEntityT >" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Vec<EcsEntityT> {
                                        unsafe {
                                            self.component.as_ref().unwrap().get_member_u64list(#field_offset)
                                        }
                                    }
                                    #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
                                    pub fn #setter_name(&self, value: Vec<EcsEntityT>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64list(#field_offset, value.as_slice());
                                        }
                                    }
                                    #[cfg(any(not(target_arch = "wasm32"), target_os = "emscripten"))]
                                    pub fn #setter_name(&self, value: Vec<EcsEntityT>) {
                                        unsafe {
                                            self.component.as_mut().unwrap().set_member_u64list(#field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ => {
                                println!("Unsupported field type: {}", quote!(#field_type));
                                panic!("Unsupported field type for getter/setter, {}", quote!(#field_type));
                            }
                        }
                    });

            let struct_fields =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        quote! {
                            pub #field_name: #field_type,
                        }
                        // let field_type_str = format!("{}", quote!(#field_type));
                        // match field_type_str.as_str() {
                        //     "Pointer" | "* mut c_void" | "F32Array" => {
                        //         quote! {
                        //             #[serde(skip)]
                        //             pub #field_name: i64,
                        //         }
                        //     },
                        //     _ => {
                        //         quote! {
                        //             pub #field_name: #field_type,
                        //         }
                        //     }
                        // }
                    });

            let default_body =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        quote! {
                            #field_name: #field_type::default(),
                        }
                    });

            let default_impl = quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self {
                            entity_added: 0,
                            component_type: 0,
                            component: std::ptr::null_mut(),
                            singleton: false,
                            id: 0,
                            #(#default_body)*
                        }
                    }
                }
            };
            
            // Create the struct name string.
            let struct_name_str = name.to_string();

            // Create the register component tokens.
            let field_names_str = field_names.clone().map(|f| f.clone().unwrap().to_string());
            let field_types_code = field_types.clone().map(|f| get_type_code(f));

            // Create the register implementation.
            let register_fn = quote! {
                fn register() -> u64 {
                    register_component(#struct_name_str, vec![#(#field_names_str.to_string()),*], vec![#(#field_types_code),*])
                }
            };
            
            let type_name = struct_name_str.as_str();
            let type_name_fn = quote! {
                fn get_name() -> &'static str {
                    #type_name
                }
            };
            let type_get_id_fn = quote! {
                fn get_id() -> u64 {
                    get_component_id(#struct_name_str)
                }
            };
            quote! {
                // #[derive(Clone, PartialEq, Serialize, Deserialize)]
                #[repr(C)]
                pub struct #name {
                    // #[serde(skip)]
                    entity_added: EcsEntityT,
                    component_type: EcsEntityT,
                    component: *mut ToxoidComponent,
                    singleton: bool,
                    id: EcsEntityT,
                    #(#struct_fields)*
                }

                #default_impl

                impl #name {
                    #(#getters_and_setters)*
                }

                impl ComponentType for #name {
                    // Static methods
                    #register_fn
                    #type_name_fn
                    #type_get_id_fn
                }

                impl Component for #name {
                    fn set_component(&mut self, component: ToxoidComponent) {
                        // TODO: Remove this boxed pointer for host (and possibly guest)
                        self.component = Box::into_raw(Box::new(component));
                    }
                    fn set_entity_added(&mut self, entity_id: EcsEntityT) {
                        self.entity_added = entity_id;
                    }
                    fn set_component_type(&mut self, component_type_id: EcsEntityT) {
                        self.component_type = component_type_id;
                    }
                }

                impl Drop for #name {
                    fn drop(&mut self) {
                        // Reconstruct the Box and drop it properly
                        unsafe {
                            if !self.component.is_null() {
                                let _ = Box::from_raw(self.component);
                            }
                        }
                    }
                }
            }
            
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        #(#expanded)*
    })
}

fn get_type_code(ty: &Type) -> u8 {
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => FieldType::U8 as u8,
        Type::Path(tp) if tp.path.is_ident("u16") => FieldType::U16 as u8,
        Type::Path(tp) if tp.path.is_ident("u32") => FieldType::U32 as u8,
        Type::Path(tp) if tp.path.is_ident("u64") => FieldType::U64 as u8,
        Type::Path(tp) if tp.path.is_ident("i8") => FieldType::I8 as u8,
        Type::Path(tp) if tp.path.is_ident("i16") => FieldType::I16 as u8,
        Type::Path(tp) if tp.path.is_ident("i32") => FieldType::I32 as u8,
        Type::Path(tp) if tp.path.is_ident("i64") => FieldType::I64 as u8,
        Type::Path(tp) if tp.path.is_ident("f32") => FieldType::F32 as u8,
        Type::Path(tp) if tp.path.is_ident("f64") => FieldType::F64 as u8,
        Type::Path(tp) if tp.path.is_ident("bool") => FieldType::Bool as u8,
        Type::Path(tp) if tp.path.is_ident("PointerT") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("EcsEntityT") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("String") => FieldType::String as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u8>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u32>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u64>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<i8>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<i16>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<i32>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<i64>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<f32>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<f64>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<PointerT>") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<EcsEntityT>") => FieldType::Pointer as u8,
        Type::Path(tp) => {
            let segment = match tp.path.segments.last() {
                Some(seg) => seg,
                None => {
                    println!("Invalid type path: {}", quote!(#ty));
                    panic!("Unsupported type code");
                }
            };
        
            // If it's not a Vec, we don't need to process further
            if segment.ident != "Vec" {
                println!("Unexpected type: {}", quote!(#ty));
                panic!("Unsupported type code");
            }
        
            // Get the generic type argument
            let inner_type = match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    match args.args.first() {
                        Some(syn::GenericArgument::Type(Type::Path(inner_ty))) => inner_ty,
                        _ => {
                            println!("Invalid Vec generic argument: {}", quote!(#ty));
                            panic!("Unsupported type code");
                        }
                    }
                },
                _ => {
                    println!("Invalid Vec arguments: {}", quote!(#ty));
                    panic!("Unsupported type code");
                }
            };
        
            // Check if the inner type is supported
            if inner_type.path.is_ident("u8") || 
               inner_type.path.is_ident("u16") || 
               inner_type.path.is_ident("u32") || 
               inner_type.path.is_ident("u64") || 
               inner_type.path.is_ident("i8") || 
               inner_type.path.is_ident("i16") || 
               inner_type.path.is_ident("i32") || 
               inner_type.path.is_ident("i64") || 
               inner_type.path.is_ident("f32") || 
               inner_type.path.is_ident("f64") ||
               inner_type.path.is_ident("PointerT") ||
               inner_type.path.is_ident("EcsEntityT")
            {
                return FieldType::Pointer as u8;
            }
        
            println!("Unsupported Vec type: {}", quote!(#ty));
            panic!("Unsupported type code");
        }
        Type::Ptr(_) => FieldType::Pointer as u8,
        _ => {
            println!("Unsupported type: {}", quote!(#ty));
            panic!("Unsupported type code")
        }
    }
}

fn get_type_size(ty: &Type) -> u32 {
    let target = std::env::var("TARGET").unwrap_or("".to_string());
    let pointer_size = if target.contains("emscripten") { 4 } else { 8 };
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => 1,
        Type::Path(tp) if tp.path.is_ident("u16") => 2,
        Type::Path(tp) if tp.path.is_ident("u32") => 4,
        Type::Path(tp) if tp.path.is_ident("u64") => 8,
        Type::Path(tp) if tp.path.is_ident("i8") => 1,
        Type::Path(tp) if tp.path.is_ident("i16") => 2,
        Type::Path(tp) if tp.path.is_ident("i32") => 4,
        Type::Path(tp) if tp.path.is_ident("i64") => 8,
        Type::Path(tp) if tp.path.is_ident("f32") => 4,
        Type::Path(tp) if tp.path.is_ident("f64") => 8,
        Type::Path(tp) if tp.path.is_ident("bool") => 1,
        Type::Path(tp) if tp.path.is_ident("PointerT") => 8,
        Type::Path(tp) if tp.path.is_ident("EcsEntityT") => 8,
        Type::Path(tp) if tp.path.is_ident("String") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u8>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u16>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u32>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u64>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i8>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i16>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i32>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i64>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<f32>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<f64>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<PointerT>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<EcsEntityT>") => pointer_size,
        Type::Ptr(_) => pointer_size,
        Type::Path(tp) => {
            let segment = match tp.path.segments.last() {
                Some(seg) => seg,
                None => {
                    println!("Invalid type path: {}", quote!(#ty));
                    panic!("Unsupported type code");
                }
            };
        
            // If it's not a Vec, we don't need to process further
            if segment.ident != "Vec" {
                println!("Unexpected type: {}", quote!(#ty));
                panic!("Unsupported type code");
            }
        
            // Get the generic type argument
            let inner_type = match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    match args.args.first() {
                        Some(syn::GenericArgument::Type(Type::Path(inner_ty))) => inner_ty,
                        _ => {
                            println!("Invalid Vec generic argument: {}", quote!(#ty));
                            panic!("Unsupported type code");
                        }
                    }
                },
                _ => {
                    println!("Invalid Vec arguments: {}", quote!(#ty));
                    panic!("Unsupported type code");
                }
            };
        
            // Check if the inner type is supported
            if inner_type.path.is_ident("u8") || 
               inner_type.path.is_ident("u16") || 
               inner_type.path.is_ident("u32") || 
               inner_type.path.is_ident("u64") || 
               inner_type.path.is_ident("i8") || 
               inner_type.path.is_ident("i16") || 
               inner_type.path.is_ident("i32") || 
               inner_type.path.is_ident("i64") || 
               inner_type.path.is_ident("f32") || 
               inner_type.path.is_ident("f64") ||
               inner_type.path.is_ident("PointerT") ||
               inner_type.path.is_ident("EcsEntityT")
            {
                return pointer_size;
            }
        
            println!("Unsupported Vec type: {}", quote!(#ty));
            panic!("Unsupported type code");
        }
        _ => {
            println!("Unsupported field type: {}", quote!(#ty));
            panic!("Unsupported field type")
        }
    }
}  

fn get_type_alignment(ty: &Type) -> u32 {
    let target = std::env::var("TARGET").unwrap_or("".to_string());
    let pointer_size = if target.contains("emscripten") { 4 } else { 8 };
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => 1,
        Type::Path(tp) if tp.path.is_ident("u16") => 2,
        Type::Path(tp) if tp.path.is_ident("u32") => 4,
        Type::Path(tp) if tp.path.is_ident("u64") => 8,
        Type::Path(tp) if tp.path.is_ident("i8") => 1,
        Type::Path(tp) if tp.path.is_ident("i16") => 2,
        Type::Path(tp) if tp.path.is_ident("i32") => 4,
        Type::Path(tp) if tp.path.is_ident("i64") => 8,
        Type::Path(tp) if tp.path.is_ident("f32") => 4,
        Type::Path(tp) if tp.path.is_ident("f64") => 8,
        Type::Path(tp) if tp.path.is_ident("bool") => 1,
        Type::Path(tp) if tp.path.is_ident("PointerT") => 8,
        Type::Path(tp) if tp.path.is_ident("EcsEntityT") => 8,
        Type::Path(tp) if tp.path.is_ident("String") => pointer_size, // Assuming String is a pointer
        Type::Path(tp) if tp.path.is_ident("Vec<u8>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u16>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u32>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<u64>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i8>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i16>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i32>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<i64>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<f32>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<f64>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<PointerT>") => pointer_size,
        Type::Path(tp) if tp.path.is_ident("Vec<EcsEntityT>") => pointer_size,
        Type::Ptr(_) => pointer_size, // Pointers are 8 bytes in a 64-bit context
        Type::Path(tp) => {
            let segment = match tp.path.segments.last() {
                Some(seg) => seg,
                None => {
                    println!("Invalid type path: {}", quote!(#ty));
                    panic!("Unsupported type code");
                }
            };
        
            // If it's not a Vec, we don't need to process further
            if segment.ident != "Vec" {
                println!("Unexpected type: {}", quote!(#ty));
                panic!("Unsupported type code");
            }
        
            // Get the generic type argument
            let inner_type = match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    match args.args.first() {
                        Some(syn::GenericArgument::Type(Type::Path(inner_ty))) => inner_ty,
                        _ => {
                            println!("Invalid Vec generic argument: {}", quote!(#ty));
                            panic!("Unsupported type code");
                        }
                    }
                },
                _ => {
                    println!("Invalid Vec arguments: {}", quote!(#ty));
                    panic!("Unsupported type code");
                }
            };
        
            // Check if the inner type is supported
            if inner_type.path.is_ident("u8") || 
               inner_type.path.is_ident("u16") || 
               inner_type.path.is_ident("u32") || 
               inner_type.path.is_ident("u64") || 
               inner_type.path.is_ident("i8") || 
               inner_type.path.is_ident("i16") || 
               inner_type.path.is_ident("i32") || 
               inner_type.path.is_ident("i64") || 
               inner_type.path.is_ident("f32") || 
               inner_type.path.is_ident("f64") ||
               inner_type.path.is_ident("PointerT") ||
               inner_type.path.is_ident("EcsEntityT")
            {
                return pointer_size;
            }
        
            println!("Unsupported Vec type: {}", quote!(#ty));
            panic!("Unsupported type code");
        }
        _ => {
            println!("Unsupported field type: {}", quote!(#ty));
            panic!("Unsupported field type")
        }
    }
}

struct ComponentTuple(Vec<Option<Type>>);

impl Parse for ComponentTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut types = Vec::new();
        
        while !input.is_empty() {
            if input.peek(Token![_]) {
                input.parse::<Token![_]>()?;
                types.push(None);
            } else {
                types.push(Some(input.parse::<Type>()?));
            }
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ComponentTuple(types))
    }
}

#[proc_macro_attribute]
pub fn components(args: TokenStream, input: TokenStream) -> TokenStream {
    let ComponentTuple(types) = parse_macro_input!(args as ComponentTuple);
    let mut func = parse_macro_input!(input as ItemFn);

    let component_vars: Vec<_> = types.iter().enumerate().filter_map(|(index, typ)| {
        typ.as_ref().map(|t| {
            let ident = make_variable_name(t);
            // Using 0-based indexing as per your current implementation
            let term_index = index as i8;
            let stmt: Stmt = syn::parse_quote! {
                let #ident = iter.components::<#t>(#term_index);
            };
            (stmt, ident)
        })
    }).collect();

    let iter_name = format_ident!("components");
    let zip_expr = component_vars.iter()
        .map(|(_, ident)| quote! { #ident.iter() })
        .reduce(|acc, next| quote! { #acc.zip(#next) })
        .unwrap_or_else(|| quote! { std::iter::empty() });

    let tuple_idents: Vec<_> = component_vars.iter().map(|(_, ident)| ident).collect();

    let map_expr = match tuple_idents.len() {
        1 => quote! { |#(#tuple_idents),*| (#(#tuple_idents),*) },
        2 => quote! { |((a, b))| (a, b) },
        3 => quote! { |(((a, b), c))| (a, b, c) },
        4 => quote! { |((((a, b), c), d))| (a, b, c, d) },
        5 => quote! { |(((((a, b), c), d), e))| (a, b, c, d, e) },
        6 => quote! { |((((((a, b), c), d), e), f))| (a, b, c, d, e, f) },
        7 => quote! { |(((((((a, b), c), d), e), f), g))| (a, b, c, d, e, f, g) },
        8 => quote! { |((((((((a, b), c), d), e), f), g), h))| (a, b, c, d, e, f, g, h) },
        9 => quote! { |(((((((((a, b), c), d), e), f), g), h), i))| (a, b, c, d, e, f, g, h, i) },
        10 => quote! { |((((((((((a, b), c), d), e), f), g), h), i), j))| (a, b, c, d, e, f, g, h, i, j) },
        11 => quote! { |(((((((((((a, b), c), d), e), f), g), h), i), j), k))| (a, b, c, d, e, f, g, h, i, j, k) },
        12 => quote! { |((((((((((((a, b), c), d), e), f), g), h), i), j), k), l))| (a, b, c, d, e, f, g, h, i, j, k, l) },
        13 => quote! { |(((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m))| (a, b, c, d, e, f, g, h, i, j, k, l, m) },
        14 => quote! { |((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n) },
        15 => quote! { |(((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o) },
        16 => quote! { |((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) },
        17 => quote! { |(((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q) },
        18 => quote! { |((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r) },
        19 => quote! { |(((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s) },
        20 => quote! { |((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t) },
        21 => quote! { |(((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u) },
        22 => quote! { |((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v) },
        23 => quote! { |(((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w) },
        24 => quote! { |((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x) },
        25 => quote! { |(((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y) },
        26 => quote! { |((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z) },
        27 => quote! { |(((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z), aa))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa) },
        28 => quote! { |((((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z), aa), ab))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab) },
        29 => quote! { |(((((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z), aa), ab), ac))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac) },
        30 => quote! { |((((((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z), aa), ab), ac), ad))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad) },
        31 => quote! { |(((((((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z), aa), ab), ac), ad), ae))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae) },
        32 => quote! { |((((((((((((((((((((((((((((((((a, b), c), d), e), f), g), h), i), j), k), l), m), n), o), p), q), r), s), t), u), v), w), x), y), z), aa), ab), ac), ad), ae), af))| (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa, ab, ac, ad, ae, af) },
        _ => panic!("Unsupported number of components (maximum is 32)"),
    };

    let iter_statement = syn::parse_quote! {
        let #iter_name = #zip_expr.map(#map_expr);
    };

    // Inject component retrieval and iterator creation at the start of the function's block
    let mut stmts = std::mem::take(&mut func.block.stmts);
    for (stmt, _) in component_vars.clone().into_iter().rev() {
        stmts.insert(0, stmt);
    }
    stmts.insert(component_vars.len(), iter_statement);
    func.block.stmts = stmts;

    let output = quote! {
        #func
    };

    TokenStream::from(output)
}

fn make_variable_name(t: &Type) -> Ident {
    let type_str = match t {
        Type::Path(type_path) if type_path.qself.is_none() => {
            // Extract the last segment as the type name
            type_path.path.segments.last().unwrap().ident.to_string()
        },
        _ => panic!("Unsupported type in `components` macro")
    };

    // Convert CamelCase to snake_case
    let mut snake_case = String::new();
    for (i, ch) in type_str.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            snake_case.push('_');
        }
        snake_case.push(ch.to_lowercase().next().unwrap());
    }

    format_ident!("{}", snake_case)
}