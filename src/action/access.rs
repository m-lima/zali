use crate::{data, entry, error};

type Result<T = ()> = std::result::Result<T, error::Access>;

pub fn access(path: String) -> Result {
    let now = entry::LastVisit::now()?;
    let access = entry::Access::try_from(path)?;
    let access = entry::Entry::new(access, now);

    let data = data::Writer::new()?;

    let (data, mut entries) = data.read(|mut r| {
        bincode::decode_from_std_read::<Vec<entry::Entry>, _, _>(&mut r, entry::CONFIG)
            .ok()
            .unwrap_or_else(Vec::new)
    });
    eprintln!("Loaded   {entries:?}");

    entries.retain(|e| e.days_since(now) < 365);
    eprintln!("Retained {entries:?}");

    std::thread::sleep(std::time::Duration::from_secs(10));

    // #[cfg(feature = "check_on_load")]
    {
        entries.sort_unstable();
        eprintln!("Sorted   {entries:?}");
        entries.dedup_by(entry::Entry::same_path);
        eprintln!("Deduped  {entries:?}");
    }

    match entries.binary_search_by(access.cmp_path()) {
        Ok(idx) => unsafe { *entries.get_unchecked_mut(idx) = access },
        Err(idx) => entries.insert(idx, access),
    }
    eprintln!("Pushed   {entries:?}");

    data.write(|mut w| {
        bincode::encode_into_std_write(entries, &mut w, entry::CONFIG).map_err(error::Entry::Encode)
    })??;

    Ok(())
}
