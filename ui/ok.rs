mod foo {
    pub struct Foo;
}

use crate::foo::Foo;

fn main() {
    let _foo = Foo;
}
