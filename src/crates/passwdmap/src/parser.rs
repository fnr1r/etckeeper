use std::{
    fs::File,
    io::{Read, Result as IoResult},
    path::Path,
};

use easy_ext::ext;
use intmap::{Entry, IntKey, IntMap};

use crate::types::{Full, IdToNameMap, NameMaps, NameOwned, NameToIdMap, SysMap};

mod quick;

pub use quick::quick_parse;

#[ext]
impl<K: IntKey, V> IntMap<K, V> {
    fn try_insert(&mut self, k: K, v: V) -> Option<V> {
        if let Entry::Vacant(entry) = self.entry(k) {
            entry.insert(v);
            None
        } else {
            Some(v)
        }
    }
}

impl NameMaps {
    pub fn parse(file: impl Read) -> Self {
        let user_info = quick_parse(file);
        let mut id_to_name = IdToNameMap::with_capacity(user_info.len());
        let mut name_to_id = NameToIdMap::with_capacity(user_info.len());
        for (id, name) in user_info {
            let (name_1, name_2) = Full::split::<1, 1>(name);
            if let Some(name_1) = id_to_name.try_insert(id, name_1) {
                let name = Full::join(name_1, name_2);
                eprintln!("WARN: duplicate id {} name {}", id, name);
                continue;
            }
            name_to_id.insert(name_2, id);
        }
        NameMaps {
            id_to_name,
            name_to_id,
        }
    }
    pub fn load(path: impl AsRef<Path>) -> IoResult<Self> {
        File::open(path.as_ref()).map(Self::parse)
    }
}

impl Drop for NameMaps {
    fn drop(&mut self) {
        for (k1, v) in self.name_to_id.drain() {
            // SAFETY: k1 and k2 MUST appear in both maps
            let k2 = unsafe { self.id_to_name.remove(v).unwrap_unchecked() };
            NameOwned::join(k1, k2);
        }
    }
}

impl SysMap {
    pub fn load() -> IoResult<Self> {
        let user_maps = NameMaps::load("/etc/passwd")?;
        let group_maps = NameMaps::load("/etc/group")?;
        Ok(Self {
            user_maps,
            group_maps,
        })
    }
}
