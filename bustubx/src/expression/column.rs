use crate::catalog::data_type::DataType;
use crate::catalog::schema::Schema;
use crate::common::scalar::ScalarValue;
use crate::common::table_ref::TableReference;
use crate::error::{BustubxError, BustubxResult};
use crate::expression::ExprTrait;
use crate::storage::tuple::Tuple;

/// A named reference to a qualified field in a schema.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColumnExpr {
    /// relation/table reference.
    pub relation: Option<TableReference>,
    /// field/column name.
    pub name: String,
}

impl ExprTrait for ColumnExpr {
    fn data_type(&self, input_schema: &Schema) -> BustubxResult<DataType> {
        input_schema.get_col_by_name(&self.name).map_or(
            Err(BustubxError::Internal("Failed to get column".to_string())),
            |col| Ok(col.data_type),
        )
    }

    fn evaluate(&self, tuple: &Tuple) -> BustubxResult<ScalarValue> {
        todo!()
    }
}