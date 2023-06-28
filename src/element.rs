use crate::{Attribute, Namespace};

/// Represents an XML element.
pub struct Element<'a> {
    /// The name of the element.
    name: &'a str,
    /// The namespaces associated with the element.
    namespaces: Vec<Namespace<'a>>,
    /// The attributes of the element.
    attributes: Vec<Attribute<'a>>,
    /// The child elements of the element.
    children: Vec<Element<'a>>,
}

impl<'a> Element<'a> {
    /// Creates a new instance of `Element` with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the element.
    ///
    /// # Example
    ///
    /// ```
    /// let element = Element::new("root");
    /// ```
    pub fn new(name: &'a str) -> Self {
        Element {
            name,
            namespaces: Vec::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Adds a namespace to the element and returns a modified `Element`.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace to be added.
    ///
    /// # Example
    ///
    /// ```
    /// let element = Element::new("root")
    ///     .add_namespace(Namespace::new("name", "http://example.com"));
    /// ```
    pub fn add_namespace(mut self, namespace: Namespace<'a>) -> Self {
        self.namespaces.push(namespace);
        self
    }

    /// Adds an attribute to the element and returns a modified `Element`.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute to be added.
    ///
    /// # Example
    ///
    /// ```
    /// let element = Element::new("root")
    ///     .add_attribute(Attribute::new("attr1", "value1"));
    /// ```
    pub fn add_attribute(mut self, attribute: Attribute<'a>) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Adds a child element to the element and returns a modified `Element`.
    ///
    /// # Arguments
    ///
    /// * `child` - The child element to be added.
    ///
    /// # Example
    ///
    /// ```
    /// let child = Element::new("child");
    /// let element = Element::new("root")
    ///     .add_child(child);
    /// ```
    pub fn add_child(mut self, child: Element<'a>) -> Self {
        self.children.push(child);
        self
    }
}

impl std::fmt::Display for Element<'_> {
    /// Formats the element and its content as an XML string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>\n", self.name)?;

        for namespace in &self.namespaces {
            write!(f, "    {}\n", namespace.to_string())?;
        }

        for attribute in &self.attributes {
            write!(f, "    {}\n", attribute.to_string())?;
        }
        write!(f,">\n")?;
        for child in &self.children {
            write!(f, "    {}\n", child.to_string())?;
        }

        write!(f, "</{}>", self.name)?;
        Ok(())
    }
}