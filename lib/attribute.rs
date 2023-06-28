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