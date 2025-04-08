#![warn(clippy::pedantic)]

#[cfg(not(target_family = "unix"))]
compile_error!("This crate is only compatible with Unix targets");

mod entries;
mod error;

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
    let mut args = std::env::args_os().skip(1);

    let action = args.next().ok_or(error::Args::NoAction)?;

    match action.as_encoded_bytes() {
        b"q" => query(args),
        b"a" => access(args),
        b"h" => {
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
