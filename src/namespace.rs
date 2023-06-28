use crate::Attribute;

/// Represents a namespace in XML.
pub struct Namespace<'a>(Attribute<'a>);

impl<'a> Namespace<'a> {
    /// Creates a new instance of `Namespace` with the given prefix and URI.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix for the namespace.
    /// * `uri` - The URI associated with the namespace.
    ///
    /// # Example
    ///
    /// ```
    /// let namespace = Namespace::new("xmlns", "http://example.com");
    /// ```
    pub fn new(prefix: &'a str, uri: &'a str) -> Self {
        Namespace(Attribute::new(prefix, uri))
    }
}

impl std::fmt::Display for Namespace<'_> {
    /// Formats the namespace as a string in the format `xmlns:prefix="uri"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xmlns:{}", self.0.to_string())?;
        Ok(())
    }
}
