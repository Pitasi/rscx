use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenTree};
use proc_macro2_diagnostics::Diagnostic;
use quote::{quote, quote_spanned, ToTokens};
use rstml::{
    node::{Node, NodeAttribute, NodeElement, NodeName},
    Parser, ParserConfig,
};
use syn::{parse::Parse, spanned::Spanned, ItemStruct};

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    html_inner(tokens, false)
}

#[proc_macro]
pub fn html_ide(tokens: TokenStream) -> TokenStream {
    html_inner(tokens, true)
}

fn html_inner(tokens: TokenStream, ide_helper: bool) -> TokenStream {
    // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
    let empty_elements: HashSet<_> = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]
    .into_iter()
    .collect();
    let config = ParserConfig::new()
        .recover_block(true)
        .always_self_closed_elements(empty_elements.clone());

    let parser = Parser::new(config);
    let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    process_nodes(empty_elements, ide_helper, &nodes, errors).into()
}

fn process_nodes<'n>(
    empty_elements: HashSet<&str>,
    ide_helper: bool,
    nodes: &'n Vec<Node>,
    errors: Vec<Diagnostic>,
) -> proc_macro2::TokenStream {
    let WalkNodesOutput {
        static_format: html_string,
        values,
        collected_elements: elements,
        diagnostics,
    } = walk_nodes(&empty_elements, &nodes);
    let docs = if ide_helper {
        generate_tags_docs(elements)
    } else {
        vec![]
    };
    let errors = errors
        .into_iter()
        .map(|e| e.emit_as_expr_tokens())
        .chain(diagnostics);
    quote! {
        {
            // Make sure that "compile_error!(..);"  can be used in this context.
            #(#errors;)*
            // Make sure that "enum x{};" and "let _x = crate::element;"  can be used in this context
            #(#docs;)*
            format!(#html_string, #(#values),*)
        }
    }
}

fn generate_tags_docs(elements: Vec<&NodeName>) -> Vec<proc_macro2::TokenStream> {
    // Mark some of elements as type,
    // and other as elements as fn in crate::docs,
    // to give an example how to link tag with docs.
    let elements_as_type: HashSet<&'static str> =
        vec!["html", "head", "meta", "link", "body", "div"]
            .into_iter()
            .collect();

    elements
        .into_iter()
        .map(|e| {
            if elements_as_type.contains(&*e.to_string()) {
                let element = quote_spanned!(e.span() => enum);
                quote!({#element X{}})
            } else {
                // let _ = crate::docs::element;
                let element = quote_spanned!(e.span() => element);
                quote!(let _ = crate::docs::#element)
            }
        })
        .collect()
}

#[derive(Default)]
struct WalkNodesOutput<'a> {
    static_format: String,
    // Use proc_macro2::TokenStream instead of syn::Expr
    // to provide more errors to the end user.
    values: Vec<proc_macro2::TokenStream>,
    // Additional diagnostic messages.
    diagnostics: Vec<proc_macro2::TokenStream>,
    // Collect elements to provide semantic highlight based on element tag.
    // No differences between open tag and closed tag.
    // Also multiple tags with same name can be present,
    // because we need to mark each of them.
    collected_elements: Vec<&'a NodeName>,
}
impl<'a> WalkNodesOutput<'a> {
    fn extend(&mut self, other: WalkNodesOutput<'a>) {
        self.static_format.push_str(&other.static_format);
        self.values.extend(other.values);
        self.diagnostics.extend(other.diagnostics);
        self.collected_elements.extend(other.collected_elements);
    }
}

fn walk_nodes<'a>(empty_elements: &HashSet<&str>, nodes: &'a Vec<Node>) -> WalkNodesOutput<'a> {
    let mut out = WalkNodesOutput::default();

    for node in nodes {
        match node {
            Node::Doctype(doctype) => {
                let value = &doctype.value.to_token_stream_string();
                out.static_format.push_str(&format!("<!DOCTYPE {}>", value));
            }
            Node::Element(element) => {
                let name = element.name().to_string();

                if !is_component_tag_name(&name) {
                    out.static_format.push_str(&format!("<{}", name));
                    out.collected_elements.push(&element.open_tag.name);
                    if let Some(e) = &element.close_tag {
                        out.collected_elements.push(&e.name)
                    }

                    // attributes
                    for attribute in element.attributes() {
                        match attribute {
                            NodeAttribute::Block(block) => {
                                // If the nodes parent is an attribute we prefix with whitespace
                                out.static_format.push(' ');
                                out.static_format.push_str("{}");
                                out.values.push(block.to_token_stream());
                            }
                            NodeAttribute::Attribute(attribute) => {
                                out.static_format.push_str(&format!(" {}", attribute.key));
                                if let Some(value) = attribute.value() {
                                    out.static_format.push_str(r#"="{}""#);
                                    out.values.push(value.to_token_stream());
                                }
                            }
                        }
                    }
                    // Ignore childs of special Empty elements
                    if empty_elements.contains(element.open_tag.name.to_string().as_str()) {
                        out.static_format
                            .push_str(&format!("/</{}>", element.open_tag.name));
                        if !element.children.is_empty() {
                            let warning = proc_macro2_diagnostics::Diagnostic::spanned(
                                element.open_tag.name.span(),
                                proc_macro2_diagnostics::Level::Warning,
                                "Element is processed as empty, and cannot have any child",
                            );
                            out.diagnostics.push(warning.emit_as_expr_tokens())
                        }

                        continue;
                    }
                    out.static_format.push('>');

                    // children
                    let other_output = walk_nodes(empty_elements, &element.children);
                    out.extend(other_output);
                    out.static_format.push_str(&format!("</{}>", name));
                } else {
                    // custom elements
                    out.static_format.push_str("{}");
                    out.values
                        .push(CustomElement::new(element).to_token_stream());
                }
            }
            Node::Text(text) => {
                out.static_format.push_str(&text.value_string());
            }
            Node::RawText(text) => {
                out.static_format.push_str("{}");
                let tokens = text.to_string_best();
                let literal = Literal::string(&tokens);

                out.values.push(TokenTree::from(literal).into());
            }
            Node::Fragment(fragment) => {
                let other_output = walk_nodes(empty_elements, &fragment.children);
                out.extend(other_output)
            }
            Node::Comment(comment) => {
                out.static_format.push_str("<!-- {} -->");
                out.values.push(comment.value.to_token_stream());
            }
            Node::Block(block) => {
                out.static_format.push_str("{}");
                out.values.push(block.to_token_stream());
            }
        }
    }

    out
}

fn is_component_tag_name(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase())
}

struct CustomElement<'e> {
    e: &'e NodeElement,
}

impl<'e> CustomElement<'e> {
    fn new(e: &'e NodeElement) -> Self {
        CustomElement { e }
    }
}

impl<'e> ToTokens for CustomElement<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = self.e.name();
        let props_struct_name = syn::Ident::new(
            &format!("{}Props", name.to_string()),
            proc_macro2::Span::call_site(),
        );

        let mut chain = vec![quote! {
            #props_struct_name::builder()
        }];

        let children = &self.e.children;
        if !children.is_empty() {
            // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
            let empty_elements: HashSet<_> = [
                "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
                "param", "source", "track", "wbr",
            ]
            .into_iter()
            .collect();
            let c = process_nodes(empty_elements, false, children, vec![]);
            chain.push(quote! { .children(#c) });
        }

        chain.push({
            self.e
                .attributes()
                .iter()
                .map(|a| match a {
                    NodeAttribute::Block(block) => {
                        quote! {
                            .push_attr(
                                #[allow(unused_braces)]
                                #block
                            )
                        }
                    }
                    NodeAttribute::Attribute(attribute) => {
                        let key = &attribute.key;
                        let value = attribute.value().unwrap();
                        quote! { .#key(#value) }
                    }
                })
                .collect::<proc_macro2::TokenStream>()
        });

        chain.push(quote! { .build() });

        tokens.extend(quote! {
            #name(#(#chain)*).await
        });
    }
}

#[proc_macro_attribute]
pub fn props(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let props = syn::parse_macro_input!(input as PropsStruct);
    quote! { #props }.to_token_stream().into()
}

struct PropsStruct {
    name: syn::Ident,
    item: ItemStruct,
}

impl Parse for PropsStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<ItemStruct>()?;
        let name = item.ident.clone();

        Ok(PropsStruct { name, item })
    }
}

impl ToTokens for PropsStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let item = &self.item;

        let builder_name =
            syn::Ident::new(&format!("{}Builder", name), proc_macro2::Span::call_site());

        let setters = item
            .fields
            .iter()
            .filter(|field| field.ident.as_ref().unwrap() != "attributes")
            .map(|field| {
                let name = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                quote! {
                    pub fn #name(mut self, #name: #ty) -> Self {
                        self.props.#name = #name;
                        self
                    }
                }
            });

        tokens.extend(quote! {
            #[derive(Default)]
            #item

            impl #name {
                pub fn builder() -> #builder_name {
                    #builder_name::default()
                }
            }

            #[derive(Default)]
            pub struct #builder_name {
                props: #name,
            }

            impl #builder_name {
                #(#setters)*

                pub fn build(self) -> #name {
                    self.props
                }
            }
        });

        let has_attributes = item
            .fields
            .iter()
            .any(|field| field.ident.as_ref().unwrap().to_string() == "attributes");

        if has_attributes {
            tokens.extend(quote! {
                impl #builder_name {
                    pub fn push_attr<A: std::fmt::Display>(mut self, attr: A) -> Self {
                        self.props.attributes.push_str(&format!("{} ", attr));
                        self
                    }
                }
            });
        }
    }
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let comp = syn::parse_macro_input!(input as ComponentFn);
    quote! { #comp }.to_token_stream().into()
}

struct ComponentFn {
    item: syn::ItemFn,
}

impl Parse for ComponentFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<syn::ItemFn>()?;
        Ok(ComponentFn { item })
    }
}

impl ToTokens for ComponentFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let item = &self.item;
        let name = &item.sig.ident;

        let (defs, args) = match item.sig.inputs.len() {
            0 => {
                // generate empty props
                let props_name =
                    syn::Ident::new(&format!("{}Props", name), proc_macro2::Span::call_site());
                (
                    quote! {
                        #[props]
                        pub struct #props_name{}
                    },
                    quote! { _props: #props_name },
                )
            }
            // match if there is a single arg of type #nameProps
            1 if matches!(item.sig.inputs.first().unwrap(), syn::FnArg::Typed(arg) if matches!(arg.ty.as_ref(), syn::Type::Path(p) if p.path.segments.last().unwrap().ident.to_string() == format!("{}Props", name))) =>
            {
                let props = item.sig.inputs.first().unwrap();
                (quote! {}, props.to_token_stream())
            }
            // TODO: generate #nameProps here
            _ => {
                panic!("wrong props")
            }
        };

        let body = &item.block;
        let output = &item.sig.output;

        tokens.extend(quote! {
            #defs
            #[allow(non_snake_case)]
            pub async fn #name(#args) #output {
                #body
            }
        });
    }
}
