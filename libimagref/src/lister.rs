use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagentrylist::lister::Lister;
use libimagentrylist::listers::core::CoreLister;
use libimagentrylist::result::Result;

use toml::Value;

pub struct RefLister;

impl RefLister {

    pub fn new() -> RefLister {
        RefLister
    }

}

fn list_fn(e: &Entry) -> String {
    let stored_hash = match e.get_header().read("ref.content_hash") {
        Ok(Some(Value::String(s))) => s.clone(),
        _                          => String::from("<Error: Could not read stored hash>"),
    };

    let filepath = match e.get_header().read("ref.path") {
        Ok(Some(Value::String(ref s))) => s.clone(),
        _ => String::from("<Error: Could not read file path>"),
    };

    format!("Ref({} -> {})", stored_hash, filepath)
}

impl Lister for RefLister {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        CoreLister::new(list_fn).list(entries)
    }

}

