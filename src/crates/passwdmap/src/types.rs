use std::collections::HashMap;

use intmap::IntMap;
use static_rc::StaticRc;

pub type Full<T> = StaticRc<T, 2, 2>;
pub type Part<T> = StaticRc<T, 1, 2>;

pub type Id = u32;
pub type NameOwned = Full<str>;
pub type Name = Part<str>;
pub type IdToNameMap = IntMap<u32, Name>;
pub type NameToIdMap = HashMap<Name, u32>;

#[derive(Debug)]
pub struct NameMaps {
    pub id_to_name: IdToNameMap,
    pub name_to_id: NameToIdMap,
}

#[derive(Debug)]
pub struct SysMap {
    pub user_maps: NameMaps,
    pub group_maps: NameMaps,
}
