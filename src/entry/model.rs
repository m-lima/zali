use super::{Error, Result};

#[derive(Debug, Eq, PartialEq, bincode::Encode, bincode::Decode)]
pub struct Entry {
    pub(super) path: std::path::PathBuf,
    pub(super) last_visit: u64,
}

impl Entry {
    pub(super) fn new(access: Access, last_visit: LastVisit) -> Self {
        Self {
            path: access.0,
            last_visit: last_visit.0,
        }
    }

    pub(super) fn days_since(&self, reference: LastVisit) -> u64 {
        const SECONDS_IN_DAY: u64 = 60 * 60 * 24;
        reference.0.saturating_sub(self.last_visit) / SECONDS_IN_DAY
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.path.cmp(&other.path) {
            std::cmp::Ordering::Equal => other.last_visit.cmp(&self.last_visit),
            o => o,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Access(std::path::PathBuf);

impl TryFrom<String> for Access {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let path = std::path::PathBuf::from(value);

        let path = path.canonicalize().map_err(|_| Error::InvalidPath(path))?;

        if !path.is_dir() {
            return Err(Error::NotDir(path));
        }

        if let Some(home) = dirs::home_dir() {
            if let Ok(stripped) = path.strip_prefix(home) {
                return Ok(Self(std::path::Path::new("~").join(stripped)));
            }
        }

        Ok(Self(path))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(super) struct LastVisit(u64);

impl LastVisit {
    pub fn now() -> Result<Self> {
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| Self(d.as_secs()))
            .map_err(Error::SystemTime)
    }
}
