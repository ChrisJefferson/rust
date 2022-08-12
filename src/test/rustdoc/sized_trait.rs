#![crate_name = "foo"]

// @has foo/struct.Bar.html
// @!hasraw - '//*[@id="impl-Sized"]'
pub struct Bar {
    a: u16,
}

// @has foo/struct.Foo.html
// @!hasraw - '//*[@id="impl-Sized"]'
pub struct Foo<T: ?Sized>(T);

// @has foo/struct.Unsized.html
// @has - '//*[@id="impl-Sized-for-Unsized"]//h3[@class="code-header in-band"]' 'impl !Sized for Unsized'
pub struct Unsized {
    data: [u8],
}
