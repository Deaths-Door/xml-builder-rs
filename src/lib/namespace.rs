pub struct Namespace<'a> {
    prefix: &'a str,
    uri: &'a str,
}

impl<'a> Namespace<'a> {
    pub fn new(prefix: &'a str, uri: &'a str) -> Self {
        Namespace { prefix, uri }
    }
}

impl<'a> std::fmt::Display for Namespace<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xmlns:{}=\"{}\"", self.prefix, self.uri)
    }
}