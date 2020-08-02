(
    include_str!(file!()).lines().collect::<Vec<_>>(),
    std::io::BufReader::new(include_bytes!(file!()) as &[u8]),
)
