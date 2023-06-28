pub struct Builder<'a> {
    declaration: Declaration,
    root_element: Element<'a>,
}

impl Builder<'_> {
    pub fn new(declaration : Declaration)
}

impl<'a> std::fmt::Display for Declaration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.declaration.to_string());
        write!(f,"{}",self.root_element.to_string());
        Ok(())
    }
}
