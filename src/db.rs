use std::path::PathBuf;

use bc_runtime_components::state::State;
use bc_utils_lg::prelude::*;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode::SyncAll};
use serde::{Deserialize, Serialize};
use serde_json5::{from_slice, to_string};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct StateDeserialize {
    pub indications: MAP<String, f64>,
    pub signals_train: MAP<String, f64>,
    pub signals: MAP<String, Signal>,
    pub utils_state: MAP<String, f64>,
    pub orders: MAP<String, OrderWrap>,
    pub orders_filtered: MAP<String, (bool, bool)>,
}

impl<'a> From<&State<'a>> for StateDeserialize {
    fn from(value: &State) -> Self {
        StateDeserialize {
            indications: value
                .indications
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            signals_train: value
                .signals_train
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            signals: value
                .signals
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            utils_state: value
                .utils_state
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
            orders: value
                .orders
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
            orders_filtered: value
                .orders_filtered
                .iter()
                .map(|(k, v)| (k.to_string(), (v.0.is_some(), v.1)))
                .collect(),
        }
    }
}

pub struct DatabaseH {
    pub db: Database,
    pub state: Keyspace,
    pub src: Keyspace,
    pub ind_col: Keyspace,
    pub ind_val: Keyspace,
}

impl DatabaseH {
    pub fn new(path: PathBuf) -> fjall::Result<Self> {
        let db = Database::builder(path).open()?;
        Ok(DatabaseH {
            state: db.keyspace("state", KeyspaceCreateOptions::default)?,
            src: db.keyspace("src", KeyspaceCreateOptions::default)?,
            ind_col: db.keyspace("ind_col", KeyspaceCreateOptions::default)?,
            ind_val: db.keyspace("ind_val", KeyspaceCreateOptions::default)?,
            db,
        })
    }

    pub fn write(&self) -> fjall::Result<()> {
        self.db.persist(SyncAll)
    }

    pub fn insert<T: Serialize>(keyspace: &Keyspace, key: &[u8], value: &T) -> fjall::Result<()> {
        keyspace.insert(
            key,
            &to_string(value).map_err(|_| fjall::Error::KeyspaceDeleted)?,
        )
    }

    pub fn read<T: for<'a> Deserialize<'a>>(
        keyspace: &Keyspace,
        key: &[u8],
    ) -> fjall::Result<Option<T>> {
        if let Some(state) = keyspace.get(key)? {
            return Ok(from_slice(&state).map_err(|_| fjall::Error::KeyspaceDeleted)?);
        }
        Err(fjall::Error::KeyspaceDeleted)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;

    use super::*;
    use bc_runtime_components::test_state::state::STATE;
    use bc_test_kit::src::SRC_EL;

    #[test]
    fn read_res_1() {
        let db = DatabaseH::new("test1".into()).unwrap();
        let key = SRC_EL[0].to_be_bytes();
        let value = StateDeserialize::from(&STATE());
        DatabaseH::insert(&db.state, &key, &value).unwrap();
        assert_eq!(
            DatabaseH::read::<StateDeserialize>(&db.state, &key)
                .unwrap()
                .unwrap(),
            value
        );
        remove_dir_all("test1").unwrap()
    }
}
