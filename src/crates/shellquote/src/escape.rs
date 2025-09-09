use std::borrow::Cow;

use crate::{consts::SINGLE_QUOTE, eval::shell_eval};

const ESCAPED_SINGLE_QUOTE: &str = r#"'"'"'"#;

pub fn singlequote_escape(txt: &str) -> String {
    format!("'{}'", txt.replace(SINGLE_QUOTE, ESCAPED_SINGLE_QUOTE))
}

pub fn singlequote_unescape(txt: &str) -> Option<Cow<'_, str>> {
    let maybe_res = txt.strip_prefix(SINGLE_QUOTE)?;
    let maybe_res = maybe_res.strip_suffix(SINGLE_QUOTE)?;
    let res = shell_eval(txt).unwrap();
    Some(if maybe_res == res {
        Cow::Borrowed(maybe_res)
    } else {
        Cow::Owned(res)
    })
}
