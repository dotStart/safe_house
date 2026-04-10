/*
 * This file is part of safe_house - a simple E2E encrypted file sharing utility.
 * Copyright (c) 2026 Yuki Donath <https://dotstart.tv> and other contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::store::error::StoreError;
use crate::store::storage::Document;
use rocksdb::{BoundColumnFamily, DBAccess, DBIteratorWithThreadMode, IteratorMode, TransactionDB};
use std::marker::PhantomData;
use std::sync::Arc;

pub trait StoreOps<B: DBAccess, D: Document> {
    fn store(&self, key: &str, document: &D) -> Result<(), StoreError>;
    fn load(&self, key: &str) -> Result<Option<D>, StoreError>;
    fn iterator(&'_ self, mode: IteratorMode) -> Result<Iterator<'_, B, D>, StoreError>;
    fn delete(&self, key: &str) -> Result<(), StoreError>;
}

pub struct Store<D: Document> {
    db: Arc<TransactionDB>,
    family: String,
    phantom: PhantomData<D>,
}

fn get_cf<'a>(
    db: &'a Arc<TransactionDB>,
    family: &str,
) -> Result<Arc<BoundColumnFamily<'a>>, StoreError> {
    match db.cf_handle(family) {
        Some(handle) => Ok(handle),
        None => Err(StoreError::MissingFamily(family.to_string())),
    }
}

impl<D: Document> Store<D> {
    pub fn new(db: &Arc<TransactionDB>, family: &str) -> Self {
        Store::<D> {
            db: Arc::clone(db),
            family: family.to_string(),
            phantom: PhantomData,
        }
    }

    pub fn transaction(&'_ self) -> Result<Transaction<'_, D>, StoreError> {
        let cf = get_cf(&self.db, &self.family)?;

        Ok(Transaction::new(&self.db, cf))
    }
}

impl<D: Document> StoreOps<TransactionDB, D> for Store<D> {
    fn store(&self, key: &str, document: &D) -> Result<(), StoreError> {
        let cf = get_cf(&self.db, self.family.as_str())?;
        let stored = document.store()?;

        self.db
            .put_cf(&cf, key, stored)
            .map_err(|e| StoreError::UpdateFailure(e.to_string()))?;

        Ok(())
    }

    fn load(&self, key: &str) -> Result<Option<D>, StoreError> {
        let cf = get_cf(&self.db, self.family.as_str())?;

        match self
            .db
            .get_cf(&cf, key)
            .map_err(|e| StoreError::ReadFailure(e.to_string()))?
        {
            Some(stored) => Ok(Some(D::load(&stored)?)),
            None => Ok(None),
        }
    }

    fn iterator(&self, mode: IteratorMode) -> Result<Iterator<'_, TransactionDB, D>, StoreError> {
        let cf = get_cf(&self.db, self.family.as_str())?;
        let it = self.db.iterator_cf(&cf, mode);

        Ok(Iterator::new(it))
    }

    fn delete(&self, key: &str) -> Result<(), StoreError> {
        let cf = get_cf(&self.db, self.family.as_str())?;

        self.db
            .delete_cf(&cf, key)
            .map_err(|e| StoreError::UpdateFailure(e.to_string()))?;

        Ok(())
    }
}

impl<D: Document> Clone for Store<D> {
    fn clone(&self) -> Self {
        Store::<D>::new(&self.db, self.family.as_str())
    }
}

pub struct RawStore {
    db: Arc<TransactionDB>,
    family: String,
}

impl RawStore {
    pub fn new(db: &Arc<TransactionDB>, family: &str) -> Self {
        RawStore {
            db: Arc::clone(db),
            family: family.to_string(),
        }
    }

    pub fn load(&self, key: &str) -> Result<Option<Vec<u8>>, StoreError> {
        let cf = get_cf(&self.db, self.family.as_str())?;

        match self
            .db
            .get_cf(&cf, key)
            .map_err(|e| StoreError::ReadFailure(e.to_string()))?
        {
            Some(stored) => Ok(Some(stored)),
            None => Ok(None),
        }
    }

    pub fn store(&self, key: &str, value: &Vec<u8>) -> Result<(), StoreError> {
        let cf = get_cf(&self.db, self.family.as_str())?;

        self.db
            .put_cf(&cf, key, value)
            .map_err(|e| StoreError::UpdateFailure(e.to_string()))?;

        Ok(())
    }

    pub fn transaction(&'_ self) -> Result<RawTransaction<'_>, StoreError> {
        let cf = get_cf(&self.db, &self.family)?;

        Ok(RawTransaction::new(&self.db, cf))
    }
}

pub struct Transaction<'a, D: Document> {
    cf: Arc<BoundColumnFamily<'a>>,
    tx: rocksdb::Transaction<'a, TransactionDB>,
    _phantom: PhantomData<D>,
}

impl<'a, D: Document> Transaction<'a, D> {
    fn new(db: &'a Arc<TransactionDB>, cf: Arc<BoundColumnFamily<'a>>) -> Self {
        let tx = db.transaction();

        Transaction {
            cf,
            tx,
            _phantom: PhantomData,
        }
    }

    pub fn commit(self) -> Result<(), StoreError> {
        self.tx
            .commit()
            .map_err(|e| StoreError::CommitFailure(e.to_string()))?;

        Ok(())
    }

    pub fn rollback(self) -> Result<(), StoreError> {
        self.tx
            .rollback()
            .map_err(|e| StoreError::RollbackFailure(e.to_string()))?;

        Ok(())
    }
}

impl<'a, D: Document> StoreOps<rocksdb::Transaction<'a, TransactionDB>, D> for Transaction<'a, D> {
    fn store(&self, key: &str, document: &D) -> Result<(), StoreError> {
        let stored = document.store()?;

        self.tx
            .put_cf(&self.cf, key, stored)
            .map_err(|e| StoreError::UpdateFailure(e.to_string()))?;

        Ok(())
    }

    fn load(&self, key: &str) -> Result<Option<D>, StoreError> {
        let stored = self
            .tx
            .get_cf(&self.cf, key)
            .map_err(|e| StoreError::ReadFailure(e.to_string()))?;

        match stored {
            Some(stored) => Ok(Some(D::load(&stored)?)),
            None => Ok(None),
        }
    }

    fn iterator(
        &self,
        mode: IteratorMode,
    ) -> Result<Iterator<'_, rocksdb::Transaction<'a, TransactionDB>, D>, StoreError> {
        let it = self.tx.iterator_cf(&self.cf, mode);
        Ok(Iterator::new(it))
    }

    fn delete(&self, key: &str) -> Result<(), StoreError> {
        self.tx
            .delete_cf(&self.cf, key)
            .map_err(|e| StoreError::UpdateFailure(e.to_string()))?;

        Ok(())
    }
}

pub struct RawTransaction<'a> {
    cf: Arc<BoundColumnFamily<'a>>,
    tx: rocksdb::Transaction<'a, TransactionDB>,
}

impl<'a> RawTransaction<'a> {
    fn new(db: &'a Arc<TransactionDB>, cf: Arc<BoundColumnFamily<'a>>) -> Self {
        let tx = db.transaction();

        RawTransaction { cf, tx }
    }

    pub fn load(&self, key: &str) -> Result<Option<Vec<u8>>, StoreError> {
        match self
            .tx
            .get_cf(&self.cf, key)
            .map_err(|e| StoreError::ReadFailure(e.to_string()))?
        {
            Some(stored) => Ok(Some(stored)),
            None => Ok(None),
        }
    }

    pub fn store(&self, key: &str, value: &Vec<u8>) -> Result<(), StoreError> {
        self.tx
            .put_cf(&self.cf, key, value)
            .map_err(|e| StoreError::UpdateFailure(e.to_string()))?;

        Ok(())
    }

    pub fn commit(self) -> Result<(), StoreError> {
        self.tx
            .commit()
            .map_err(|e| StoreError::CommitFailure(e.to_string()))?;

        Ok(())
    }

    pub fn rollback(self) -> Result<(), StoreError> {
        self.tx
            .rollback()
            .map_err(|e| StoreError::RollbackFailure(e.to_string()))?;

        Ok(())
    }
}

pub struct Iterator<'a, B: DBAccess, D: Document> {
    it: DBIteratorWithThreadMode<'a, B>,
    _phantom: PhantomData<D>,
}

impl<'a, B: DBAccess, D: Document> Iterator<'a, B, D> {
    fn new(it: DBIteratorWithThreadMode<'a, B>) -> Self {
        Iterator {
            it,
            _phantom: PhantomData,
        }
    }
}

impl<'a, B: DBAccess, D: Document> std::iter::Iterator for Iterator<'a, B, D> {
    type Item = Result<KV<D>, StoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        let stored = match self.it.next() {
            Some(Ok(kv)) => kv,
            Some(Err(e)) => return Some(Err(StoreError::ReadFailure(e.to_string()))),
            None => return None,
        };

        let key = match str::from_utf8(stored.0.as_ref()) {
            Ok(key) => key.to_string(),
            Err(e) => return Some(Err(StoreError::ReadFailure(e.to_string()))),
        };
        let value = match D::load(stored.1.to_vec().as_ref()) {
            Ok(stored) => stored,
            Err(e) => return Some(Err(e)),
        };

        Some(Ok((Box::new(key), Box::new(value))))
    }
}

pub type KV<D> = (Box<String>, Box<D>);
