mod abspath;
mod cleanpath;
mod relpath;
pub mod utils;

mod std_path_as_path {
    pub use std::path::{Path, PathBuf};

    pub use camino::{Utf8Path, Utf8PathBuf};
}

pub use abspath::{abspath, abspath_utf8};
pub use cleanpath::{cleanpath, cleanpath_utf8};
pub use relpath::{relpath, relpath_cwd, relpath_utf8, relpath_utf8_cwd};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
