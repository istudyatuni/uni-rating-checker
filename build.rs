const ENV_FILE: &str = include_str!(".env");

fn main() {
    parse_dotenv();
    env_commit_hash();
}

fn parse_dotenv() {
    for line in ENV_FILE.split('\n') {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
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

fn env_commit_hash() {
    #[cfg(feature = "prod")]
    {
        let output = match std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
        {
            Ok(output) => match String::from_utf8(output.stdout) {
                Ok(s) => s,
                Err(e) => {
                    println!("cargo:warning=cannot get output of \"git rev-parse --short HEAD\" command: {e}");
                    return;
                }
            },
            Err(e) => {
                println!("cargo:warning=cannot get git commit hash: {e}");
                return;
            }
        };
        println!("cargo:rustc-env=GIT_HASH={output}")
    }
    #[cfg(not(feature = "prod"))]
    println!("cargo:rustc-env=GIT_HASH=dev")
}
