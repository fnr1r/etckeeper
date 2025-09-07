use std::io::{BufRead, BufReader, Read};

use crate::types::{Id, NameOwned};

/// NOTE: not fully conformant - ignores everything after uid
fn parse_line(line: String) -> Option<(Id, NameOwned)> {
    let mut parts = line.splitn(4, ':');

    let name = parts.next()?;
    let _ = parts.next()?;
    let id_str = parts.next()?;
    // Ignore the rest
    let _ = parts.next()?;

    let Ok(id) = id_str.parse::<Id>() else {
        return None;
    };

    Some((id, name.into()))
}

fn should_parse(line: &str) -> bool {
    !line.starts_with('#') && !line.trim().is_empty()
}

/// Quick and generic parser for `/etc/passwd` and `/etc/group`
///
/// Ignores any fields except name and id
pub fn quick_parse(buf: impl Read) -> Vec<(Id, NameOwned)> {
    BufReader::new(buf)
        .lines()
        .map_while(Result::ok)
        .filter(|e| should_parse(e))
        .filter_map(parse_line)
        .collect()
}
