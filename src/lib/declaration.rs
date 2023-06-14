pub struct Declaration<'a> {
    version: &'a str,
    encoding: &'a str,
    standalone: Option<&'a str>,
}