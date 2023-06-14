pub struct Attribute<'a> {
    name: &'a str,
    value: &'a str,
    namespace: Option<&'a str>,
}