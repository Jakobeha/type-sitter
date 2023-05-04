use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr};
use crate::node_types::{Children, NodeName, NodeType, NodeTypeKind, Type};

impl NodeType {
    pub fn print(&self) -> TokenStream {
        let NodeName { rust_type_name, rust_method_name: _, sexp_name, is_implicit } = &self.name;
        let ident = Ident::new(rust_type_name, Span::call_site());
        let sexp_name_literal = LitStr::new(sexp_name, Span::call_site());
        // let named = self.named;
        match &self.kind {
            NodeTypeKind::Supertype { subtypes } => {
                if !is_implicit {
                    panic!("Node types with subtypes must be implicit (start with \"_\")")
                }

                let variants = subtypes.iter().map(Type::print_variant);
                let from_cases = subtypes.iter().map(Type::print_from_case);
                let node_cases = subtypes.iter().map(Type::print_node_case);
                quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[doc = concat!("Typed node `", #sexp_name, "`")]
                    pub enum #ident<'tree> {
                        #variants
                    }

                    impl<'tree> TryFrom<TSNode<'tree>> for #ident<'tree> {
                        type Error = tree_sitter_lib::IncorrectType<'tree>;

                        fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
                            match node.kind() {
                                #cases
                                _ => Err(tree_sitter_lib::IncorrectType {
                                    kind: #sexp_name_literal,
                                    actual: node.kind()
                                })
                            }
                        }
                    }

                    impl<'tree> tree_sitter_lib::TypedNode<'tree> for #ident<'tree> {
                        fn node(&self) -> &tree_sitter::Node<'tree> {
                            match self {
                                #node_cases
                            }
                        }
                    }
                }
            }
            NodeTypeKind::Regular { fields, children } => {
                if is_implicit {
                    panic!("Node types without subtypes must not be implicit (not start with \"_\")")
                }
                let field_accessors = fields.iter().map(|(name, field)| {
                    let name_sexp_literal = LitStr::new(name, Span::call_site());
                    field.print(
                        (
                            format!("{}s", name),
                            quote!(concat!("Get the field `", #name_sexp_literal, "`")),
                            quote! { self.0.children_by_field_name(#name_sexp_literal, &mut c) }
                        ),
                        (
                            name.clone(),
                            quote!(concat!("Get the field `", #name_sexp_literal, "`")),
                            quote! { self.0.child_by_field_name(#name_sexp_literal) }
                        ),
                        None
                    )
                });
                let child_accessor = children.print(
                    name.as_str(),
                    (
                        "children",
                        quote!(concat!("Get the node's children")),
                        quote! { self.0.children(&mut c) }
                    ),
                    (
                        "child",
                        quote!(concat!("Get the node's child")),
                        quote! { self.0.child(0) }
                    ),
                    Some(|i| (
                        format!("child_{}", i),
                        quote!(concat!("Get the node's child #", stringify!(i))),
                        quote! { self.0.child(i) }
                    ))
                );
                quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[doc = concat!("Typed node `", #sexp_name, "`")]
                    pub struct #ident<'tree>(tree_sitter::Node<'tree>);

                    impl<'tree> TryFrom<TSNode<'tree>> for #ident<'tree> {
                        type Error = tree_sitter_lib::IncorrectType<'tree>;

                        fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
                            if node.kind() == #sexp_name_literal {
                                Ok(Self(node))
                            } else {
                                Err(tree_sitter_lib::IncorrectType {
                                    kind: #sexp_name_literal,
                                    actual: node.kind()
                                })
                            }
                        }
                    }

                    impl<'tree> tree_sitter_lib::TypedNode<'tree> for #ident<'tree> {
                        fn node(&self) -> &tree_sitter::Node<'tree> {
                            &self.0
                        }
                    }

                    impl<'tree> #ident<'tree> {
                        #field_accessors
                        #child_accessor
                    }
                }
            }
        }
        quote! {
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body): (&str, TokenStream, TokenStream),
        (child_name, child_doc, child_body): (&str, TokenStream, TokenStream),
        child_i: Option<impl Fn(usize) -> (String, TokenStream, TokenStream)>
    ) -> TokenStream {
        if self.multiple {
            todo!()
        }
        let ident = Ident::new(name, Span::call_site());
        let (field_type, field_ctor) = field.print(
            quote! { self.0.children_by_field_name(#name_sexp_literal, &mut c) },
            quote! { self.0.child_by_field_name(#name_sexp_literal) }
        );
        quote! {
                        #[doc = concat!("Get the field `", #name_sexp_literal, "`")]
                        pub fn #ident(&self) -> #field_type {
                            #field_ctor
                        }
                    }
    }
}

impl Type {
    fn print_variant(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        quote! {
            #ident(#ident<'tree>),
        }
    }

    fn print_from_case(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        let sexp_name_literal = LitStr::new(&self.name.sexp_name, Span::call_site());
        quote! {
            #sexp_name_literal => Ok(Self::#ident(#ident(node))),
        }
    }

    fn print_node_case(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        quote! {
            Self::#ident(x) => x.node(),
        }
    }
}