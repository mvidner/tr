use std::io::Error;
use gettextrs::gettext;

mod software;
mod storage;
mod web;

fn main() -> Result<(), Error> {
    println!("{}", gettext("Main"));
    web::doit();
    Ok(())
}
