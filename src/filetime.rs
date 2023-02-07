use std::time::{Duration, SystemTime};

const NTFS_UNIX_EPOCH: Duration = Duration::from_secs(11644473600);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTime(u64);

impl Into<u64> for FileTime {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<SystemTime> for FileTime {
    fn into(self) -> SystemTime {
        self.to_system_time()
    }
}

impl From<SystemTime> for FileTime {
    fn from(value: SystemTime) -> Self {
        Self::from_system_time(value)
    }
}

impl From<i64> for FileTime {
    #[inline]
    fn from(value: i64) -> Self {
        Self::from_raw(value as u64)
    }
}

impl From<u64> for FileTime {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl FileTime {
    #[inline]
    pub(crate) fn from_raw(ntfs: u64) -> Self {
        FileTime(ntfs)
    }

    pub(crate) fn raw(self) -> u64 {
        self.0
    }

    #[inline]
    fn to_system_time(self) -> SystemTime {
        let ntfs_epoch = SystemTime::UNIX_EPOCH - NTFS_UNIX_EPOCH;
        let nanos = Duration::from_nanos(self.0 * 100);
        let time = ntfs_epoch + nanos;
        time
    }

    #[inline]
    fn from_system_time(time: SystemTime) -> Self {
        let ntfs_epoch = SystemTime::UNIX_EPOCH - NTFS_UNIX_EPOCH;
        let time = time.duration_since(ntfs_epoch).unwrap_or_default();
        let nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
        if nanos == 0 {
            return FileTime(0);
        }
        FileTime(nanos / 100)
    }
}
