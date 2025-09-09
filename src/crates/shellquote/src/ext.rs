use easy_ext::ext;

use crate::singlequote_escape;

#[ext(ExtStrEscapeSinglequote)]
pub impl str {
    fn escape_singlequote(&self) -> String {
        singlequote_escape(self)
    }
}
