use crate::{data, entry, error};

type Result<T = ()> = std::result::Result<T, error::Access>;

pub fn access(path: String) -> Result {
    let now = entry::LastVisit::now()?;
    let access = entry::Access::try_from(path)?;
    let access = entry::Entry::new(access, now);

    let data = data::Data::<true>::new()?;

    let mut entries =
        bincode::decode_from_std_read::<Vec<entry::Entry>, _, _>(&mut data.reader(), entry::CONFIG)
            .ok()
            .unwrap_or_else(Vec::new);
    eprintln!("Loaded   {entries:?}");

    entries.retain(|e| e.days_since(now) < 365);
    eprintln!("Retained {entries:?}");

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

    bincode::encode_into_std_write(entries, &mut data.writer(), entry::CONFIG)
        .map_err(error::Entry::Encode)?;

    Ok(())
}
