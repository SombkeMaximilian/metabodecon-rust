macro_rules! extract_capture {
    ($re:expr, $text:expr, $name:ident) => {
        $re.captures($text)
            .unwrap()
            .name(stringify!($name))
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    };
}
