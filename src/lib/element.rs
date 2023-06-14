pub struct Element<'a> {
    name: &'a str,
    namespaces: Vec<&'a str>,
    attributes: Vec<Attribute<'a>>,
    children: Vec<Element<'a>>,
    text: Option<&'a str>,
}