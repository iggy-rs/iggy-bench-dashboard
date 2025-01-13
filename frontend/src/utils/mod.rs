pub fn get_file_prefix(benchmark_name: &str) -> &'static str {
    if benchmark_name.starts_with("send_") {
        "producer"
    } else {
        "consumer"
    }
}
