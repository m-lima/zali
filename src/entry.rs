use crate::error;

type Result<T = ()> = std::result::Result<T, error::Entry>;

pub const CONFIG: bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
> = bincode::config::standard().with_fixed_int_encoding();

#[derive(Debug, Eq, PartialEq, bincode::Encode, bincode::Decode)]
pub struct Entry {
    path: std::path::PathBuf,
    last_visit: u64,
}

impl Entry {
    pub fn new(access: Access, last_visit: LastVisit) -> Self {
        Self {
            path: access.0,
            last_visit: last_visit.0,
        }
    }

    pub fn cmp_path(&self) -> impl Fn(&Self) -> std::cmp::Ordering {
        |other| other.path.cmp(&self.path)
    }

    pub fn same_path(a: &mut Self, b: &mut Self) -> bool {
        a.path == b.path
    }

    pub fn days_since(&self, reference: LastVisit) -> u64 {
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
    type Error = error::Entry;

    fn try_from(value: String) -> Result<Self> {
        let path = std::path::PathBuf::from(value);

        let path = path
            .canonicalize()
            .map_err(|_| error::Entry::InvalidPath(path))?;

        if !path.is_dir() {
            return Err(error::Entry::NotDir(path));
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
pub struct LastVisit(u64);

impl LastVisit {
    pub fn now() -> Result<Self> {
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| Self(d.as_secs()))
            .map_err(error::Entry::SystemTime)
    }
}
