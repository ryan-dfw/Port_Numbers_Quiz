pub fn printlist(pairs: &[(i32, &str, char)]) {
    for pair in pairs {
        let protocol = match pair.2 {
            'T' => "TCP",
            'U' => "UDP",
            _ => "unknown",
        };
        println!("port {} is {} using {}", pair.0, pair.1, protocol);
    }
}
