use std::{ffi::OsString, io::Result as IoResult};

use camino::Utf8Path;

type VecXattrs = Vec<(OsString, Vec<u8>)>;
//type MapXattrs = BTreeMap<OsString, Vec<u8>>;

pub struct FileXattrs(pub VecXattrs);

fn get_xattr(path: &Utf8Path, attr: OsString) -> Option<IoResult<(OsString, Vec<u8>)>> {
    let value = xattr::get(path, &attr).transpose()?;
    Some(value.map(|e| (attr, e)))
}

impl FileXattrs {
    pub fn load(path: &Utf8Path) -> IoResult<Self> {
        let xattrs = xattr::list(path)?;
        let mapfunc = |e| get_xattr(path, e);
        let mut inner = xattrs
            .into_iter()
            .filter_map(mapfunc)
            .collect::<Result<Vec<_>, _>>()?;
        inner.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(Self(inner))
    }
}

fn debug_xattr(m: &mut std::fmt::DebugMap<'_, '_>, v: &[u8]) {
    if let Ok(v) = std::str::from_utf8(v) {
        m.value(&v);
    } else {
        m.value(&v);
    }
}

impl std::fmt::Debug for FileXattrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        for (k, v) in &self.0 {
            m.key(&k);
            debug_xattr(&mut m, v);
        }
        m.finish()
    }
}
