use crate::{BustubxError, BustubxResult};

use crate::catalog::Catalog;
use crate::common::TableReference;
use crate::planner::logical_plan::{LogicalPlan, OrderByExpr};

pub struct PlannerContext<'a> {
    pub catalog: &'a Catalog,
}

pub struct LogicalPlanner<'a> {
    pub context: PlannerContext<'a>,
}
impl<'a> LogicalPlanner<'a> {
    pub fn plan(&mut self, stmt: &sqlparser::ast::Statement) -> BustubxResult<LogicalPlan> {
        match stmt {
            sqlparser::ast::Statement::CreateTable { name, columns, .. } => {
                self.plan_create_table(name, columns)
            }
            sqlparser::ast::Statement::CreateIndex {
                name,
                table_name,
                columns,
                ..
            } => self.plan_create_index(name, table_name, columns),
            sqlparser::ast::Statement::Query(query) => self.plan_query(query),
            sqlparser::ast::Statement::Insert {
                table_name,
                columns,
                source,
                ..
            } => self.plan_insert(table_name, columns, source),
            sqlparser::ast::Statement::Update {
                table,
                assignments,
                selection,
                ..
            } => self.plan_update(table, assignments, selection),
            _ => unimplemented!(),
        }
    }

    pub fn bind_order_by_expr(
        &self,
        order_by: &sqlparser::ast::OrderByExpr,
    ) -> BustubxResult<OrderByExpr> {
        let expr = self.bind_expr(&order_by.expr)?;
        Ok(OrderByExpr {
            expr: Box::new(expr),
            asc: order_by.asc.unwrap_or(true),
            nulls_first: order_by.nulls_first.unwrap_or(false),
        })
    }

    pub fn bind_table_name(
        &self,
        table_name: &sqlparser::ast::ObjectName,
    ) -> BustubxResult<TableReference> {
        match table_name.0.as_slice() {
            [table] => Ok(TableReference::bare(table.value.clone())),
            [schema, table] => Ok(TableReference::partial(
                schema.value.clone(),
                table.value.clone(),
            )),
            [catalog, schema, table] => Ok(TableReference::full(
                catalog.value.clone(),
                schema.value.clone(),
                table.value.clone(),
            )),
            _ => Err(BustubxError::Plan(format!(
                "Fail to plan table name: {}",
                table_name
            ))),
        }
    }
}
