pub struct Declaration<'a> {
    version: &'a str,
    encoding: &'a str,
    standalone: Option<&'a str>,
}

impl Declaration<'_> {
    pub fn new(version: &str, encoding: &str) -> Self {
        Declaration {
            version: version,
            encoding: encoding,
            standalone : None,
        }
    }

    pub fn with_standlone(&self,standalone : &str) -> Self {
        self.standalone = Some(standalone);
        self
    }
}

impl<'a> fmt::Display for Declaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"<?xml version="{}" encoding="{}""#, self.version, self.encoding)?;
        
        if let Some(standalone) = self.standalone {
            write!(f, r#" standalone="{}""#, standalone)?;
        }
        
        write!(f, "?>")?;
        Ok(())
    }
}