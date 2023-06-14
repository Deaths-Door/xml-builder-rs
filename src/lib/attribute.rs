pub struct Attribute<'a> {
    name: &'a str,
    value: &'a str,
    namespace: Option<Namespace>,
}

impl Attribute {
    pub fn new(name: &str, value: &str) -> Self {
        Attribute {
            name,
            value,
            None,
        }
    }
    pub fn with_namespace(namespace: Namespace) -> Self {
        self.namespace = Some(namespace);
        self
    }
}

impl fmt::Display for Attribute<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(namespace) = self.namespace {
            write!(f, "{}:", namespace.prefix)?;
        };

        write!(f, "{}", self.name)?;
        
        write!(f, "=\"{}\"", self.value)?;
        Ok(())
    }
}