use crate::entry;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No matches")]
    NoMatches,
    #[error("Error executing fzf: {0}")]
    Fzf(std::io::Error),
    #[error("Could not communicate with fzf: {0}")]
    Pipe(std::io::Error),
    #[error("Could not write to stdout: {0}")]
    Stdout(std::io::Error),
    #[error(transparent)]
    Entry(#[from] crate::entry::Error),
}

pub fn query<S: AsRef<str>>(query: S) -> Result {
    let query = query.as_ref();
    let entries = entry::load()?;

    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

    let mut scores = entries
        .iter()
        .filter_map(|e| {
            e.path().to_str().and_then(|path| {
                fuzzy_matcher::FuzzyMatcher::fuzzy_match(&matcher, path, query)
                    .map(|score| (score, e.last_visit_secs(), path))
            })
        })
        .collect::<Vec<_>>();

    match scores.len() {
        0 => Err(Error::NoMatches),
        1 => {
            println!("{}", unsafe { scores.get_unchecked(0) }.2);
            Ok(())
        }
        _ => {
            scores.sort_unstable_by(|a, b| match b.0.cmp(&a.0) {
                std::cmp::Ordering::Equal => b.1.cmp(&a.1),
                c => c,
            });

            let mut fzf = pwner::Spawner::spawn_owned(&mut std::process::Command::new("fzf"))
                .map_err(Error::Fzf)?;

            for s in scores {
                std::io::Write::write_all(&mut fzf, s.2.as_bytes()).map_err(Error::Pipe)?;
                std::io::Write::write(&mut fzf, b"\n").map_err(Error::Pipe)?;
            }

            let (status, mut fzf_out, _) = fzf.wait().map_err(Error::Fzf)?;
            if status.success() {
                let mut stdout = std::io::stdout().lock();
                let mut buffer = [0; 1024];
                loop {
                    let bytes =
                        std::io::Read::read(&mut fzf_out, &mut buffer).map_err(Error::Pipe)?;

                    if bytes == 0 {
                        break;
                    }

                    std::io::Write::write(&mut stdout, &buffer[..bytes]).map_err(Error::Stdout)?;
                }
            }
            Ok(())
        }
    }
}
