use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::result::Result as RResult;

use store::Result;
use store::Store;
use storeid::IntoStoreId;
use storeid::StoreId;

#[derive(Debug, Clone)]
pub struct EntryStoreId(StoreId);

impl EntryStoreId {

    pub fn new(store: &Store, si: StoreId) -> Result<EntryStoreId> {
        si.unstorified(store).map(|si| EntryStoreId(si))
    }

}

impl Into<StoreId> for EntryStoreId {

    fn into(self) -> StoreId {
        self.0
    }

}

impl Deref for EntryStoreId {
    type Target = StoreId;

    fn deref(&self) -> &StoreId {
        &self.0
    }
}

impl IntoStoreId for EntryStoreId {

    fn into_storeid(self) -> StoreId {
        self.into()
    }

}

impl From<PathBuf> for EntryStoreId {

    fn from(pb: PathBuf) -> EntryStoreId {
        EntryStoreId(StoreId::from(pb))
    }

}

impl Display for EntryStoreId {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "{}", self.0)
    }

}

