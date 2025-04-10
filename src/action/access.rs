use crate::entry;

pub fn access(path: String) -> entry::Result {
    entry::insert(path)
}
