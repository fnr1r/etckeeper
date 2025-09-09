#[cfg(test)]
use crate::escape::{singlequote_escape, singlequote_unescape};

#[test]
fn test_1() {
    let txt = "test test";
    let escaped = singlequote_escape(txt);
    let unescaped = singlequote_unescape(&escaped);
    assert_eq!(txt, unescaped.unwrap());
}

#[test]
fn test_2() {
    let txt = r#"abc123'aaa'bbbb"ccc"ddd##;"#;
    let escaped = singlequote_escape(txt);
    let unescaped = singlequote_unescape(&escaped);
    assert_eq!(txt, unescaped.unwrap());
}
