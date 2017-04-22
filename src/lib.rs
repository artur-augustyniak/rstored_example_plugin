#[no_mangle]
pub fn run_probe() -> String {
    //your stuff, result must be encoded in json string
    let ret = r#"{ "key":"value", "len":31, "foo":"bar"}"#.to_string();
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
