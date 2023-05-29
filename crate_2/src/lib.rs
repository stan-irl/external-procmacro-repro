#[derive(uniffi::Object)]
pub struct Bar {
    foo: std::sync::Arc<crate_1::Foo>,
}

#[uniffi::export]
impl Bar {
    #[uniffi::constructor]
    pub fn new(foo: std::sync::Arc<crate_1::Foo>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self { foo })
    }
    fn bar_thing(&self) {
        println!("doing bar thing");
        self.foo.foo_thing();
        println!("did bar thing");
    }
}

uniffi::include_scaffolding!("api");
