use crate::gettext;
use crate::{software, storage};

pub fn doit() {
    println!("{}", gettext("Web"));
    software::web::doit();
    storage::web::doit();
}
