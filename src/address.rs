#[derive(Debug)]
pub struct Address {
    foo: String,
}

impl Address {
    pub fn new(foo: String) -> Self {
        Address { foo: foo }
    }
}
