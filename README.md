# einfach-xml-builder-rs
 A lightweight and intuitive library for generating XML documents in Rust. With an easy-to-use API, it allows you to create well-formed XML structures programmatically. Add elements, attributes, namespaces, and CDATA sections effortlessly. 

## Features

- Create XML declarations with specified version, encoding, and standalone attributes.
- Build XML elements with namespaces, attributes, and child elements.
- Generate XML strings from the constructed XML documents.

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
einfach_xml_builder = "0.1.0"
```

## Usage

Here's an example of how to use XML Builder to construct an XML document:

```rust
use xml_builder::{Declaration, Element, Attribute, Namespace};

fn main() {
    // Create an XML declaration
    let declaration = Declaration::new("1.0", "UTF-8").with_standalone(true);

    // Create an XML element
    let element = Element::new("root")
        .add_namespace(Namespace::new("android", "http://example.com"))
        .add_attribute(Attribute::new("attr1", "value1"))
        .add_attribute(Attribute::new("attr2", "value2"))
        .add_child(Element::new("child1"))
        .add_child(Element::new("child2"));

    // Create a builder with the declaration and element
    let builder = Builder::new(declaration, element);

    // Print the XML document
    println!("{}", builder.to_string());
}
```

Please note that the examples provided here are simplified and serve as a starting point. For comprehensive documentation of the crate, please visit the [crate documentation](https://docs.rs/xml-builder) for a better understanding of the crate's functionalities and APIs.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
