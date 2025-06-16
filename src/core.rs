pub fn hello(name: impl AsRef<str>) {
    println!("Hello, {}!", name.as_ref())
}
