use crate::buffer::{PageId, BUSTUBX_PAGE_SIZE};
use crate::catalog::SchemaRef;
use crate::common::rid::Rid;
use crate::common::TransactionId;
use crate::storage::codec::{TablePageHeaderCodec, TablePageHeaderTupleInfoCodec, TupleCodec};
use crate::{BustubxError, BustubxResult, Tuple};

lazy_static::lazy_static! {
    pub static ref EMPTY_TUPLE_INFO: TupleInfo = TupleInfo {
        offset: 0,
        size: 0,
        meta: TupleMeta {
            insert_txn_id: 0,
            delete_txn_id: 0,
            is_deleted: false,
        }
    };
}

/**
 * Slotted page format:
 *  ---------------------------------------------------------
 *  | HEADER | ... FREE SPACE ... | ... INSERTED TUPLES ... |
 *  ---------------------------------------------------------
 *                                ^
 *                                free space pointer
 *
 *  Header format (size in bytes):
 *  ----------------------------------------------------------------------------
 *  | NextPageId (4)| NumTuples(2) | NumDeletedTuples(2) |
 *  ----------------------------------------------------------------------------
 *  ----------------------------------------------------------------
 *  | Tuple_1 offset+size + TupleMeta | Tuple_2 offset+size + TupleMeta | ... |
 *  ----------------------------------------------------------------
 *
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TablePage {
    pub schema: SchemaRef,
    pub header: TablePageHeader,
    // 整个页原始数据
    pub data: [u8; BUSTUBX_PAGE_SIZE],
}

// TODO do we need pre_page_id?
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TablePageHeader {
    pub next_page_id: PageId,
    pub num_tuples: u16,
    pub num_deleted_tuples: u16,
    pub tuple_infos: Vec<TupleInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TupleInfo {
    pub offset: u16,
    pub size: u16,
    pub meta: TupleMeta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TupleMeta {
    pub insert_txn_id: TransactionId,
    pub delete_txn_id: TransactionId,
    pub is_deleted: bool,
}

impl TablePage {
    pub fn new(schema: SchemaRef, next_page_id: PageId) -> Self {
        Self {
            schema,
            header: TablePageHeader {
                next_page_id,
                num_tuples: 0,
                num_deleted_tuples: 0,
                tuple_infos: Vec::new(),
            },
            data: [0; BUSTUBX_PAGE_SIZE],
        }
    }

    // Get the offset for the next tuple insertion.
    pub fn next_tuple_offset(&self, tuple: &Tuple) -> BustubxResult<usize> {
        // Get the ending offset of the current slot. If there are inserted tuples,
        // get the offset of the previous inserted tuple; otherwise, set it to the size of the page.
        let slot_end_offset = if self.header.num_tuples > 0 {
            self.header.tuple_infos[self.header.num_tuples as usize - 1].offset as usize
        } else {
            BUSTUBX_PAGE_SIZE
        };

        // Check if the current slot has enough space for the new tuple. Return None if not.
        if slot_end_offset < TupleCodec::encode(tuple).len() {
            return Err(BustubxError::Storage(
                "No enough space to store tuple".to_string(),
            ));
        }

        // Calculate the insertion offset for the new tuple by subtracting its data length
        // from the ending offset of the current slot.
        let tuple_offset = slot_end_offset - TupleCodec::encode(tuple).len();

        // Calculate the minimum valid tuple insertion offset, including the table page header size,
        // the total size of each tuple info (existing tuple infos and newly added tuple info).
        let min_tuple_offset = TablePageHeaderCodec::encode(&self.header).len()
            + TablePageHeaderTupleInfoCodec::encode(&EMPTY_TUPLE_INFO).len();
        if tuple_offset < min_tuple_offset {
            return Err(BustubxError::Storage(
                "No enough space to store tuple".to_string(),
            ));
        }

        // Return the calculated insertion offset for the new tuple.
        Ok(tuple_offset)
    }

    pub fn insert_tuple(&mut self, meta: &TupleMeta, tuple: &Tuple) -> BustubxResult<u16> {
        // Get the offset for the next tuple insertion.
        let tuple_offset = self.next_tuple_offset(tuple)?;
        let tuple_id = self.header.num_tuples;

        // Store tuple information including offset, length, and metadata.
        self.header.tuple_infos.push(TupleInfo {
            offset: tuple_offset as u16,
            size: TupleCodec::encode(tuple).len() as u16,
            meta: *meta,
        });

        // only check
        assert_eq!(tuple_id, self.header.tuple_infos.len() as u16 - 1);

        self.header.num_tuples += 1;
        if meta.is_deleted {
            self.header.num_deleted_tuples += 1;
        }

        // Copy the tuple's data into the appropriate position within the page's data buffer.
        self.data[tuple_offset..tuple_offset + TupleCodec::encode(tuple).len()]
            .copy_from_slice(&TupleCodec::encode(tuple));
        Ok(tuple_id)
    }

    pub fn update_tuple_meta(&mut self, meta: &TupleMeta, slot_num: u16) -> BustubxResult<()> {
        if slot_num >= self.header.num_tuples {
            return Err(BustubxError::Storage(format!(
                "tuple_id {} out of range",
                slot_num
            )));
        }
        if meta.is_deleted && !self.header.tuple_infos[slot_num as usize].meta.is_deleted {
            self.header.num_deleted_tuples += 1;
        }

        self.header.tuple_infos[slot_num as usize].meta = *meta;
        Ok(())
    }

    pub fn tuple(&self, slot_num: u16) -> BustubxResult<(TupleMeta, Tuple)> {
        if slot_num >= self.header.num_tuples {
            return Err(BustubxError::Storage(format!(
                "tuple_id {} out of range",
                slot_num
            )));
        }

        let offset = self.header.tuple_infos[slot_num as usize].offset;
        let size = self.header.tuple_infos[slot_num as usize].size;
        let meta = self.header.tuple_infos[slot_num as usize].meta;
        let (tuple, _) = TupleCodec::decode(
            &self.data[offset as usize..(offset + size) as usize],
            self.schema.clone(),
        )?;

        Ok((meta, tuple))
    }

    pub fn tuple_meta(&self, slot_num: u16) -> BustubxResult<TupleMeta> {
        if slot_num >= self.header.num_tuples {
            return Err(BustubxError::Storage(format!(
                "tuple_id {} out of range",
                slot_num
            )));
        }

        Ok(self.header.tuple_infos[slot_num as usize].meta)
    }

    pub fn get_next_rid(&self, rid: &Rid) -> Option<Rid> {
        // TODO 忽略删除的tuple
        let tuple_id = rid.slot_num;
        if tuple_id + 1 >= self.header.num_tuples as u32 {
            return None;
        }

        Some(Rid::new(rid.page_id, tuple_id + 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::catalog::{Column, DataType, Schema};
    use crate::storage::Tuple;
    use std::sync::Arc;

    #[test]
    pub fn test_table_page_get_tuple() {
        let schema = Arc::new(Schema::new(vec![
            Column::new("a", DataType::Int8, false),
            Column::new("b", DataType::Int16, false),
        ]));
        let mut table_page = super::TablePage::new(schema.clone(), 0);
        let meta = super::TupleMeta {
            insert_txn_id: 0,
            delete_txn_id: 0,
            is_deleted: false,
        };
        let tuple_id = table_page
            .insert_tuple(
                &meta,
                &Tuple::new(schema.clone(), vec![1i8.into(), 1i16.into()]),
            )
            .unwrap();
        assert_eq!(tuple_id, 0);
        let tuple_id = table_page
            .insert_tuple(
                &meta,
                &Tuple::new(schema.clone(), vec![2i8.into(), 2i16.into()]),
            )
            .unwrap();
        assert_eq!(tuple_id, 1);
        let tuple_id = table_page
            .insert_tuple(
                &meta,
                &Tuple::new(schema.clone(), vec![3i8.into(), 3i16.into()]),
            )
            .unwrap();
        assert_eq!(tuple_id, 2);

        let (tuple_meta, tuple) = table_page.tuple(0).unwrap();
        assert_eq!(tuple_meta, meta);
        assert_eq!(tuple.data, vec![1i8.into(), 1i16.into()]);
        let (_tuple_meta, tuple) = table_page.tuple(1).unwrap();
        assert_eq!(tuple.data, vec![2i8.into(), 2i16.into()]);
        let (_tuple_meta, tuple) = table_page.tuple(2).unwrap();
        assert_eq!(tuple.data, vec![3i8.into(), 3i16.into()]);
    }

    #[test]
    pub fn test_table_page_update_tuple_meta() {
        let schema = Arc::new(Schema::new(vec![
            Column::new("a", DataType::Int8, false),
            Column::new("b", DataType::Int16, false),
        ]));
        let mut table_page = super::TablePage::new(schema.clone(), 0);
        let meta = super::TupleMeta {
            insert_txn_id: 0,
            delete_txn_id: 0,
            is_deleted: false,
        };
        let _tuple_id = table_page
            .insert_tuple(
                &meta,
                &Tuple::new(schema.clone(), vec![1i8.into(), 1i16.into()]),
            )
            .unwrap();
        let _tuple_id = table_page
            .insert_tuple(
                &meta,
                &Tuple::new(schema.clone(), vec![2i8.into(), 2i16.into()]),
            )
            .unwrap();
        let _tuple_id = table_page
            .insert_tuple(
                &meta,
                &Tuple::new(schema.clone(), vec![3i8.into(), 3i16.into()]),
            )
            .unwrap();

        let mut tuple_meta = table_page.tuple_meta(0).unwrap();
        tuple_meta.is_deleted = true;
        tuple_meta.delete_txn_id = 1;
        tuple_meta.insert_txn_id = 2;

        table_page.update_tuple_meta(&tuple_meta, 0).unwrap();
        let tuple_meta = table_page.tuple_meta(0).unwrap();
        assert!(tuple_meta.is_deleted);
        assert_eq!(tuple_meta.delete_txn_id, 1);
        assert_eq!(tuple_meta.insert_txn_id, 2);
    }
}