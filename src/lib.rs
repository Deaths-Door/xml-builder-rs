//! # xml-builder-rs
//!  A lightweight and intuitive library for generating XML documents in Rust. With an easy-to-use API, it allows you to create well-formed XML structures programmatically. Add elements, attributes, namespaces, and CDATA sections effortlessly. 
//! 
//! ## Features
//! 
//! - Create XML declarations with specified version, encoding, and standalone attributes.
//! - Build XML elements with namespaces, attributes, and child elements.
//! - Generate XML strings from the constructed XML documents.
//! 
//! ## Installation
//! 
//! Add the following line to your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! xml_builder = "0.1.0"
//! ```
//! 
//! ## Usage
//! Here's an example of how to use XML Builder to construct an XML document:

//! ```rust
//! use xml_builder::{Declaration, Element, Attribute, Namespace};
//! 
//! fn main() {
//!     // Create an XML declaration
//!     let declaration = Declaration::new("1.0", "UTF-8").with_standalone(true);
//! 
//!     // Create an XML element
//!     let element = Element::new("root")
//!         .add_namespace(Namespace::new("android", "http://example.com"))
//!         .add_attribute(Attribute::new("attr1", "value1"))
//!         .add_attribute(Attribute::new("attr2", "value2"))
//!         .add_child(Element::new("child1"))
//!         .add_child(Element::new("child2"));
//! 
//!     // Create a builder with the declaration and element
//!     let builder = Builder::new(declaration, element);
//! 
//!     // Print the XML document
//!     println!("{}", builder.to_string());
//! }
//! ```
mod declaration;
mod attribute;
mod element;
mod namespace;
mod builder;

pub use self::declaration::*;
pub use self::attribute::*;
pub use self::element::*;
pub use self::namespace::*;
pub use self::builder::*;