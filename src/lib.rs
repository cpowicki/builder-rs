extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field,
};

#[proc_macro_derive(Builder)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let target_name = input.ident.to_string();
    let builder_name = format!("{}Builder", target_name);

    let mut attr = "".to_owned();
    let mut setters = "".to_owned();
    let mut builder = "".to_owned();
    let mut initializer = "".to_owned();

    if let Data::Struct(s) = input.data {
        attr = builder_attributes(&s);
        setters = builder_setters(builder_name.clone(), &s);
        builder = build_method(target_name.clone(), &s);
        initializer = builder_initializer(&s);
    }

    let builder_impl = format!(r#"
        pub struct {} {{
            {}
        }}

        impl {} {{
            {}
            {}
        }}"#,
        builder_name, attr, builder_name, setters, builder
    );

    let target_impl = format!(r#"
        impl {} {{
            pub fn builder() -> {} {{
                {} {{
                    {}
                }}
            }}
        }}
    "#, target_name, builder_name, builder_name, initializer);

    format!("{}\n{}", target_impl,builder_impl).parse().unwrap()
}

fn builder_attributes(s: &DataStruct) -> String {
    s.fields
        .iter()
        .map(|f| to_option(f))
        .collect::<Vec<String>>()
        .join(",\n")
}

fn to_option(f: &Field) -> String {
    let name = f
        .clone()
        .ident
        .map(|i| i.to_string())
        .unwrap_or("_0".to_string());

    let t = match f.ty {
        syn::Type::Path(ref path) => path.path.get_ident().unwrap().to_string(),
        _ => "String".to_owned(),
    };
    format!("{}: Option<{}>", name, t)
}

fn builder_setters(name: String, s: &DataStruct) -> String {
    s.fields
        .iter()
        .map(|f| to_setter(name.clone(), f))
        .collect::<Vec<String>>()
        .join("\n")
}

fn to_setter(builder: String, f: &Field) -> String {
    let name = f
        .clone()
        .ident
        .map(|i| i.to_string())
        .unwrap_or("_0".to_string());

    let t = match f.ty {
        syn::Type::Path(ref path) => path.path.get_ident().unwrap().to_string(),
        _ => "String".to_owned(),
    };

    format!(
        r#"
        pub fn {}(mut self, value: {}) -> {} {{
            self.{} = Some(value);
            self
        }}
    "#,
        name, t, builder, name
    )
}

fn build_method(target: String, s: &DataStruct) -> String {
    let assignments = s
        .fields
        .iter()
        .map(to_assignment)
        .collect::<Vec<String>>()
        .join(",\n");

    format!(
        r#"
        pub fn build(mut self) -> {} {{
            {} {{
                {}
            }}
        }}
    "#,
        target, target, assignments
    )
}

fn to_assignment(f: &Field) -> String {
    let name = f
        .clone()
        .ident
        .map(|i| i.to_string())
        .unwrap_or("_0".to_string());

    format!("{} : self.{}.expect(\"{} is required\")", name, name, name)
}

fn builder_initializer(s: &DataStruct) -> String {
    s.fields.iter()
        .map(|f| {
            let field_name = f
            .clone()
            .ident
            .map(|i| i.to_string())
            .unwrap_or("_0".to_string());
            format!("{}: None", field_name)
        }).collect::<Vec<String>>()
        .join(",\n")
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
