pub struct Bar {
    pub bar: String,
}

impl Bar {
    pub fn do_nothing(&self) {
        // do nothing
    }
}

pub struct Foo {
    pub foo: String,
}

impl Foo {
    pub fn do_nothing(&self) -> u8 {
        // slightly separate signature. If .do_nothing is called in the following
        // `impl_for` block, this method is suggested and go to implementation will end up here,
        // even though the `do_nothing` method called is actually on Bar.
        0
    }
}

#[attribute_example::impl_for(Bar)]
impl Foo {
    pub fn bar(&self) {
        self.do_nothing();
        // the RustRover autocomplete thinks this is a method of Foo, when it should be seeing the macro expansion and showing
        // completions for Bar. This impacts the documentation intention and go to definition, among other things.
        // rust-analyzer correctly shows the completions for Bar.
    }
}
