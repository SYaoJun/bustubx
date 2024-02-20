use crate::catalog::{SchemaRef, EMPTY_SCHEMA_REF};
use crate::common::TableReference;
use crate::planner::logical_plan::OrderByExpr;
use crate::{
    execution::{ExecutionContext, VolcanoExecutor},
    storage::Tuple,
    BustubxResult,
};

#[derive(Debug, derive_new::new)]
pub struct PhysicalCreateIndex {
    pub name: String,
    pub table: TableReference,
    pub table_schema: SchemaRef,
    pub columns: Vec<OrderByExpr>,
}

impl VolcanoExecutor for PhysicalCreateIndex {
    fn next(&self, _context: &mut ExecutionContext) -> BustubxResult<Option<Tuple>> {
        // TODO implement
        Ok(None)
    }
    fn output_schema(&self) -> SchemaRef {
        EMPTY_SCHEMA_REF.clone()
    }
}

impl std::fmt::Display for PhysicalCreateIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CreateIndex: {}", self.name)
    }
}
