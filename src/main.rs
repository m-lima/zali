#![warn(clippy::pedantic)]

mod entries {
    use crate::{Result, error};

    fn data_path() -> Result<&'static std::path::PathBuf> {
        static PATH: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        PATH.get_or_init(|| {
            dirs::data_local_dir().map(|p| p.join(env!("CARGO_PKG_NAME")).join("entries"))
        })
        .as_ref()
        .ok_or(error::Error::DataPath)
    }

    pub struct Entry {
        score: u64,
        path: std::ffi::OsString,
    }

    impl Entry {
        fn from_bytes(bytes: &[u8]) {
            // if bytes.len() > std::mem::size_of::<u64>() + std::mem::size_of::<u8>() {
            //     let mut score = [u8; std::mem::size_of::<u64>()];
            // }
            todo!()
        }
    }

    pub struct Access(std::path::PathBuf);

    impl Access {
        pub fn register(self) -> Result {
            // let path = data_path()?;
            // let bytes = {
            //     let mut file = std::fs::OpenOptions::new()
            //         .read(true)
            //         .write(true)
            //         .create(true)
            //         .open(path)?;
            //
            //     fs2::FileExt::lock_exclusive(&file);
            //
            //     let size = file.metadata().map(|m| m.len() as usize).ok();
            //     let mut bytes = Vec::new();
            //     bytes.try_reserve_exact(size.unwrap_or(0))?;
            //     std::io::Read::read_to_end(&mut file, &mut bytes);
            //     bytes
            // };
            todo!()
        }
    }

    impl TryFrom<String> for Access {
        type Error = error::Error;

        fn try_from(value: String) -> Result<Self> {
            let path = std::path::PathBuf::from(value);

            let path = path
                .canonicalize()
                .map_err(|_| error::Args::InvalidPath(path))?;

            if path.is_dir() {
                Ok(Self(path))
            } else {
                Err(error::Error::Args(error::Args::InvalidPath(path)))
            }
        }
    }

    pub struct Entries(Vec<u8>);

    impl Entries {
        pub fn new() -> Result<Self> {
            let path = data_path()?;
            let bytes = {
                let mut file = std::fs::OpenOptions::new().read(true).open(path)?;

                fs2::FileExt::lock_shared(&file);

                let size = file.metadata().map(|m| m.len() as usize).ok();
                let mut bytes = Vec::new();
                bytes.try_reserve_exact(size.unwrap_or(0))?;
                std::io::Read::read_to_end(&mut file, &mut bytes);
                bytes
            };

            Ok(Self(bytes))
        }
    }
}

mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Args(#[from] Args),
        #[error("Could not establish data dir")]
        DataPath,
        #[error("Failed to access data: {0}")]
        Io(#[from] std::io::Error),
        #[error("Failed to allocate: {0}")]
        Allocation(#[from] std::collections::TryReserveError),
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Args {
        #[error("No action provided")]
        NoAction,
        #[error("Unknown action provided: `{0}`")]
        UnknownAction(String),
        #[error("No query provided")]
        NoQuery,
        #[error("No path provided")]
        NoPath,
        #[error("Invalid path: {0:?}")]
        InvalidPath(std::path::PathBuf),
    }
}

type Result<T = ()> = std::result::Result<T, error::Error>;

fn main() -> std::process::ExitCode {
    match fallible_main() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error::Error::Args(err)) => {
            use std::io::Write;

            let mut stderr = std::io::stderr().lock();
            let _ = writeln!(&mut stderr, "{err}");
            let _ = writeln!(&mut stderr);
            let _ = show_help(stderr);
            std::process::ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn fallible_main() -> Result {
    let mut args = std::env::args().skip(1);

    let action = args.next().ok_or(error::Args::NoAction)?;

    match action.as_str() {
        "q" => query(args),
        "a" => access(args),
        "h" => {
            let _ = show_help(std::io::stdout().lock());
            Ok(())
        }
        _ => Err(error::Error::Args(error::Args::UnknownAction(action))),
    }
}

fn query<A: Iterator<Item = String>>(mut args: A) -> Result {
    let query = args.next().ok_or(error::Args::NoQuery)?;
    let entries = entries::Entries::new()?;

    println!(
        "Hello, nix world! {query} PKG: {} APP: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_BIN_NAME")
    );

    Ok(())
}

fn access<A: Iterator<Item = String>>(mut args: A) -> Result {
    let access = args
        .next()
        .ok_or(error::Error::Args(error::Args::NoPath))
        .and_then(entries::Access::try_from)?;

    Ok(())
}

fn show_help<W: std::io::Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "Usage: zali [ACTION] [args...]

A directory classifier

Commands:
  q <query>  Query the matching directory entries
  a <path>   Registers an access to the given path
  h          Show this help message

Examples:
  zali q co/ru/z  Queries zali for the matches of `co/ru/z` and their scores

Output:
  If there's a single match, just the match is returned:
    $ zali q co/ru/z
    /home/user/code/rust/zali

  If there are multiple matches, each will take a line preceded by their score:
    $ zali q co/ru/z
    100 /home/user/code/rust/zali/
    60 /homu/user/records/carl/utils/lazy"
    )
}
