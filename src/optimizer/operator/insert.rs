use crate::catalog::{column::Column, schema::Schema};

#[derive(Debug)]
pub struct PhysicalInsertOperator {
    pub table_name: String,
    pub columns: Vec<Column>,
}
impl PhysicalInsertOperator {
    pub fn new(table_name: String, columns: Vec<Column>) -> Self {
        Self {
            table_name,
            columns,
        }
    }
    pub fn output_schema(&self) -> Schema {
        Schema::new(self.columns.clone())
    }
}