// TODO. Export here

#[macro_export]
macro_rules! create_greeting {
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}
