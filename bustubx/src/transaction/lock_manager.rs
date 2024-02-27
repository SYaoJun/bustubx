use crate::common::TableReference;
use crate::storage::RecordId;
use crate::transaction::{Transaction, TransactionId};
use std::collections::HashMap;

#[derive(Debug)]
pub enum LockMode {
    Shared,
    Exclusive,
    IntentionShared,
    IntentionExclusive,
    SharedIntentionExclusive,
}

pub struct LockRequest {
    txn_id: TransactionId,
    lock_mod: LockMode,
    table_ref: TableReference,
    rid: Option<RecordId>,
    granted: bool,
}

pub struct LockManager {
    table_lock_map: HashMap<TableReference, Vec<LockRequest>>,
    row_lock_map: HashMap<RecordId, Vec<LockRequest>>,
}

impl LockManager {
    pub fn lock_table(&self, txn: Transaction, mode: LockMode, table_ref: TableReference) -> bool {
        todo!()
    }

    pub fn unlock_table(&self, txn: Transaction, table_ref: TableReference) -> bool {
        todo!()
    }

    pub fn lock_row(
        &self,
        txn: Transaction,
        mode: LockMode,
        table_ref: TableReference,
        rid: RecordId,
    ) -> bool {
        todo!()
    }

    pub fn unlock_row(
        &self,
        txn: Transaction,
        table_ref: TableReference,
        rid: RecordId,
        force: bool,
    ) -> bool {
        todo!()
    }

    pub fn unlock_all(&self) {
        todo!()
    }
}
