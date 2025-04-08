const APP_NAME: &str = env!("CARGO_PKG_NAME");

mod path {
    pub fn data() -> Option<&'static std::path::PathBuf> {
        static PATH: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        PATH.get_or_init(|| dirs::data_local_dir().map(|p| p.join(crate::APP_NAME)))
            .as_ref()
    }
}

fn main() -> std::process::ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(query) = args.next() else {
        return std::process::ExitCode::FAILURE;
    };

    let Ok(a) = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
    else {
        return std::process::ExitCode::FAILURE;
    };

    print!("{a:?} {query}");

    std::process::ExitCode::SUCCESS
}
