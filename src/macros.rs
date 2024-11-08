/// Creates a [`PathBuf`](std::path::PathBuf) from the provided pathbuf list
#[macro_export]
macro_rules! pathbuf {
    ($($path: expr),*) => {{
        let mut pathbuf = ::std::path::PathBuf::new();
        $(pathbuf.push($path);)*
        pathbuf
    }}
}
