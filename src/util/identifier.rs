pub fn identifier(namespace: &str, path: &str) -> String {
    namespace.to_owned() + ":" + &*path
}