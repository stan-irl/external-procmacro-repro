#[derive(uniffi::Object)]
pub struct Foo {}

#[uniffi::export]
impl Foo {
    #[uniffi::constructor]
    pub fn new() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {})
    }

    pub fn foo_thing(&self) {
        println!("did foo thing")
    }
}

uniffi::include_scaffolding!("api");
