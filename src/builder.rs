use crate::{Declaration, Element};

/// Represents a builder for constructing an XML document.
pub struct Builder<'a> {
    /// The XML declaration.
    declaration: Declaration<'a>,
    /// The root element of the XML document.
    element: Element<'a>,
}

impl<'a> Builder<'a> {
    /// Creates a new instance of `Builder` with the given declaration and root element.
    ///
    /// # Arguments
    ///
    /// * `declaration` - The XML declaration.
    /// * `element` - The root element of the XML document.
    ///
    /// # Example
    ///
    /// ```
    /// let declaration = Declaration::new("1.0", "UTF-8");
    /// let element = Element::new("root");
    /// let builder = Builder::new(declaration, element);
    /// ```
    pub fn new(declaration: Declaration<'a>, element: Element<'a>) -> Self {
        Builder {
            declaration,
            element,
        }
    }
}

impl<'a> std::fmt::Display for Builder<'a> {
    /// Formats the builder and its content as an XML document string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.declaration.to_string())?;
        write!(f, "{}", self.element.to_string())?;
        Ok(())
    }
}