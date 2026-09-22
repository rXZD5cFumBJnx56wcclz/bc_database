use std::ops::RangeBounds;
use std::path::PathBuf;

use fjall::Slice;
use fjall::{Database, Keyspace, KeyspaceCreateOptions, PersistMode::SyncAll};
use serde::{Deserialize, Serialize};
use serde_json5::{from_slice, to_string};

use crate::prelude::*;

pub struct DatabaseH {
    pub db: Database,
    pub state: Keyspace,
    pub trade_state: Keyspace,
    pub trade_state_executed: Keyspace,
    pub trade_state_cleared: Keyspace,
    pub src: Keyspace,
    pub ind_col: Keyspace,
    pub ind_val: Keyspace,
}

impl DatabaseH {
    pub fn new(path: PathBuf) -> fjall::Result<Self> {
        let db = Database::builder(path).open()?;
        Ok(DatabaseH {
            state: db.keyspace("state", KeyspaceCreateOptions::default)?,
            trade_state: db.keyspace("trade_state", KeyspaceCreateOptions::default)?,
            trade_state_executed: db
                .keyspace("trade_state_executed", KeyspaceCreateOptions::default)?,
            trade_state_cleared: db
                .keyspace("trade_state_cleared", KeyspaceCreateOptions::default)?,
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

    pub fn read<T: for<'a> Deserialize<'a>>(keyspace: &Keyspace, key: &[u8]) -> DbResult<T> {
        if let Some(state) = keyspace.get(key).map_err(|e| DbError::Read(e))? {
            return from_slice(&state).map_err(|e| DbError::ValueParse(e));
        }
        Err(DbError::ValueNotFound)
    }

    pub fn load_history<T: for<'a> Deserialize<'a>, K: AsRef<[u8]>>(
        keyspace: &Keyspace,
        range: impl RangeBounds<K>,
    ) -> DbResult<Vec<(Slice, T)>> {
        keyspace
            .range(range)
            .map(|g| {
                let (k, v) = g.into_inner()?;
                Ok((k, from_slice(&v).map_err(|e| DbError::ValueParse(e))?))
            })
            .collect::<DbResult<_>>()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;

    use super::*;
    use bc_runtime_components::test_state::state::STATE;
    use bc_test_kit::prelude::*;

    #[test]
    fn read_res_1() {
        let db = DatabaseH::new("test1".into()).unwrap();
        let key = SRC_EL[0].to_be_bytes();
        let value = StateDeserialize::from(&STATE());
        DatabaseH::insert(&db.state, &key, &value).unwrap();
        assert_eq!(
            DatabaseH::read::<StateDeserialize>(&db.state, &key).unwrap(),
            value
        );
        remove_dir_all("test1").unwrap()
    }

    #[test]
    fn load_history_res_1() {
        let db = DatabaseH::new("test2".into()).unwrap();
        let key = SRC_EL1[0].to_be_bytes();
        let value = StateDeserialize::from(&STATE());
        DatabaseH::insert(&db.state, &key, &value).unwrap();
        let key2 = SRC_EL[0].to_be_bytes();
        DatabaseH::insert(&db.state, &key2, &value).unwrap();
        let history = DatabaseH::load_history::<StateDeserialize, _>(&db.state, key..).unwrap();
        let test = vec![
            (<[u8; _] as Into<Slice>>::into(key), value.clone()),
            (key2.into(), value.clone()),
        ];
        assert_eq_pr!(history.len(), 2);
        assert_eq_pr!(history, test);
        remove_dir_all("test2").unwrap()
    }
}
