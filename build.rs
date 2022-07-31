const ENV_FILE: &str = include_str!(".env");

fn main() {
    for line in ENV_FILE.split_whitespace() {
        if line.starts_with('#') {
            continue;
        }
        if line.ends_with('=') {
            println!(
                "cargo:warning={} environment variable is not set",
                line.trim_end_matches('=')
            );
            continue;
        }
        println!("cargo:rustc-env={line}");
    }
}
