use crate::entry;

pub fn query(_query: String) -> entry::Result {
    let _ = entry::load()?;
    Ok(())
}
