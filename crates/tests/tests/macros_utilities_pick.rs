// A proc-macro crate can only export macros, so its expansion modules are compiled into this
// test target from source; only part of each module is exercised here.
#![allow(dead_code)]

#[path = "../../macros/src/field_type.rs"]
mod field_type;
#[path = "../../macros/src/helpers.rs"]
mod helpers;
#[path = "../../macros/src/models/mod.rs"]
mod models;
#[path = "../../macros/src/utilities/mod.rs"]
mod utilities;

mod tests {
    use crate::utilities::pick::*;
    use quote::{ToTokens, format_ident, quote};

    #[test]
    fn test_parse_single_without_generics() {
        let input = quote! {
            PickedPerson, Person as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 0);

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson {
                name: <Person as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_without_generics_and_trait_paths() {
        let input = quote! {
            PickedPerson, Person as crate :: person::Pickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 0);

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson {
                name: <Person as crate::person::Pickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string(),);
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime() {
        let input = quote! {
            PickedPerson, Person<'a> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 1);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a> {
                name: <Person<'a> as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_with_phantom_data() {
        let input = quote! {
            PickedPerson, Person<'a, _> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_with_phantom_data_and_another_lifetime() {
        let input = quote! {
            PickedPerson, Person<'a, _, 'b> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "'b",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, 'b > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_with_phantom_data_and_another_lifetime_and_const() {
        let input = quote! {
            PickedPerson, Person<'a, _, 'b, _> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 4);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "'b", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, 'b, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_multiple_lifetimes_only() {
        let input = quote! {
            PickedPerson, Person<'a, 'b> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));

        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec!["name".to_string(), "age".to_string()]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "'b",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<'a, 'b> as PersonPickable>::name,
                age: <Person<'a, 'b> as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_multiple_lifetimes_and_skipped_at_beginning() {
        let input = quote! {
            PickedPerson, Person<_, 'a, 'b> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "'a", "'b",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, 'b> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, 'a, 'b> as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, 'a, 'b> as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_multiple_lifetimes_and_skipped_at_end() {
        let input = quote! {
            PickedPerson, Person<'a, 'b, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "'b", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse_single_lifetime_and_type_param() {
        let input = quote! {
            PickedPerson, Person<'a, T> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "T",]
        );
        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, T> {
                name: <Person<'a, T> as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_parse() {
        let input = quote! {
            PickedPerson, Person<'a, _, U, _> as PersonPickable, [name, age, some, another]
        };

        // let picked_meta = PickedMeta::parse(input.into()).unwrap();
        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .len(),
            4
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![
                String::from("name"),
                String::from("age"),
                String::from("some"),
                String::from("another"),
            ]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 4);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, U> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
                some: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::some,
                another: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::another,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_single_type_params_generics() {
        let input = quote! {
            PickedPerson, Person<T> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 1);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_single_type_params_generics_with_phantom_data() {
        let input = quote! {
            PickedPerson, Person<T, _> as PersonPickable, [name]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            1
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T> {
                name: <Person<T, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics() {
        let input = quote! {
            PickedPerson, Person<T, U> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 2);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T", "U",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<T, U> as PersonPickable>::name,
                age: <Person<T, U> as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_end() {
        let input = quote! {
            PickedPerson, Person<T, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["T", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning() {
        let input = quote! {
            PickedPerson, Person<_, T, U> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 3);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "T", "U",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end() {
        let input = quote! {
            PickedPerson, Person<_, T, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 4);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "T", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end_and_middle() {
        let input = quote! {
            PickedPerson, Person<_, T, _, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 5);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["_", "T", "_", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };

        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end_and_middle_and_lifetime()
     {
        let input = quote! {
            PickedPerson, Person<'a, _, T, _, U, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 6);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "T", "_", "U", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, T, U> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_type_params_generics_with_phantom_data_at_the_beginning_and_end_and_middle_and_lifetime_and_const()
     {
        let input = quote! {
            PickedPerson, Person<'a, _, T, _, U, _, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        assert_eq!(picked_meta.new_struct, format_ident!("PickedPerson"));
        assert_eq!(picked_meta.old_struct, format_ident!("Person"));
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>()
                .len(),
            2
        );
        assert_eq!(
            picked_meta
                .field_names
                .iter()
                .map(|field| field.name.to_string())
                .collect::<Vec<String>>(),
            vec![String::from("name"), String::from("age"),]
        );
        assert_eq!(picked_meta.generics_without_bounds.params.len(), 7);
        assert_eq!(
            picked_meta
                .generics_without_bounds
                .params
                .iter()
                .map(|param| param.to_token_stream().to_string())
                .collect::<Vec<String>>(),
            vec!["'a", "_", "T", "_", "U", "_", "_",]
        );

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<'a, T, U> {
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_skips_at_beginning_and_end() {
        let input = quote! {
            PickedPerson, Person<_, _, _, U, _, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_skips_at_beginning_and_end_and_middle() {
        let input = quote! {
            PickedPerson, Person<_, _, T, _, U, _, _> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T, U> {
                name: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any>, T, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any>, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };
        assert_eq!(tokenstream, expected.to_string());
        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn test_multiple_skips_at_beginning_all() {
        let input = quote! {
            PickedPerson, Person<_, _, _, _, _, _, T> as PersonPickable, [name, age]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");
        let tokenstream = picked_meta.to_token_stream().to_string();
        let expected = quote! {
            pub struct PickedPerson<T> {
                name: <Person
        <::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        T >
        as PersonPickable>::name,

                age: <Person
        <::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        ::std::marker::PhantomData<dyn ::std::any::Any>,
        T >
        as PersonPickable>::age,
            }
        };

        assert_eq!(tokenstream, expected.to_string());

        insta::assert_snapshot!(tokenstream);
    }

    #[test]
    fn can_add_container_struct_and_field_attributes() {
        let input = quote! {
            #[derive(Debug)]
            PickedPerson,
            Person<'a, _, U, _> as PersonPickable,
            [
                #[serde(rename = "goodie")]
                name,
                age
            ]
        };

        let picked_meta = syn::parse2::<PickedMeta>(input).expect("failed to parse");

        let tokenstream = picked_meta.to_token_stream().to_string();

        let expected = quote! {
            #[derive(Debug)]
            pub struct PickedPerson<'a, U> {
                #[serde(rename = "goodie")]
                name: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::name,
                age: <Person<'a, ::std::marker::PhantomData<dyn ::std::any::Any>, U, ::std::marker::PhantomData<dyn ::std::any::Any> > as PersonPickable>::age,
            }
        };

        assert_eq!(tokenstream, expected.to_string());
    }
}
