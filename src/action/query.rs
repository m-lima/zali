use crate::entry;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No matches")]
    NoMatches,
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
            let mut stdout = std::io::stdout().lock();
            scores.sort_unstable_by(|a, b| match b.0.cmp(&a.0) {
                std::cmp::Ordering::Equal => b.1.cmp(&a.1),
                c => c,
            });
            for s in scores {
                use std::io::Write;
                writeln!(stdout, "{} {}", s.0, s.2).map_err(Error::Stdout)?;
            }
            Ok(())
        }
    }
}
