#![warn(clippy::pedantic)]

#[cfg(not(target_family = "unix"))]
compile_error!("This crate is only compatible with Unix targets");

mod action;
mod entry;
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
            action::help(stderr);
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
        "q" => {
            let query = args.next().ok_or(error::Args::Missing)?;
            action::query(query)?;
        }
        "a" => {
            let path = args.next().ok_or(error::Args::Missing)?;
            action::access(path)?;
        }
        "h" | "-h" => {
            action::help(std::io::stdout().lock());
        }
        _ => return Err(error::Error::Args(error::Args::UnknownAction(action))),
    }

    Ok(())
}
