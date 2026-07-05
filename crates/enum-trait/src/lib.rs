//! This crate contains the [`Enum`] trait, a marker trait that is
//! used to restrict its Supertraits to being applied to enums.
//!
//! While you can manually implement the [`Enum`] trait, you'll
//! receive compile-time validation of a type being an enum if
//! you derive the trait instead.
//!
//! This trait has no methods because a type implementing it is
//! enough on its own.
//!
//! ```
//! #[derive(enum_trait::Enum)]
//! enum ExampleEnum { A, B, C }
//!
//! trait ExampleTrait: enum_trait::Enum {}
//!
//! impl ExampleTrait for ExampleEnum {}
//! ```
//! ```compile_fail
//! # #[derive(enum_trait::Enum)]
//! # enum ExampleEnum { A, B, C }
//! # trait ExampleTrait: enum_trait::Enum {}
//! #[derive(Enum)] // fails
//! struct ExampleStruct;
//! ```
//! ```compile_fail
//! # #[derive(enum_trait::Enum)]
//! # enum ExampleEnum { A, B, C }
//! # trait ExampleTrait: enum_trait::Enum {}
//! # struct ExampleStruct;
//! impl ExampleTrait for ExampleStruct {}
//! ```
//!

#![no_std]
#![forbid(unsafe_code)]
#![feature(associated_type_defaults)]

pub use enum_trait_derive::Enum;

pub mod base {
    mod private {
        pub trait Sealed {}
    }

    pub struct Yes;
    pub struct No;

    trait Certainty: private::Sealed {}
    impl private::Sealed for Yes {}
    impl private::Sealed for No {}
    impl Certainty for Yes {}
    impl Certainty for No {}

    #[allow(private_bounds)]
    pub trait EnumBase {
        type AreYouSure: Certainty = No;
    }
}

/// This is a derive-only trait that throws when used on non-enums
pub trait Enum: base::EnumBase<AreYouSure = base::Yes> {}

