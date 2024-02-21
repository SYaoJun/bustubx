use log::debug;
use std::sync::Arc;
use tempfile::TempDir;

use crate::buffer::TABLE_HEAP_BUFFER_POOL_SIZE;
use crate::catalog::load_catalog_data;
use crate::common::util::{pretty_format_logical_plan, pretty_format_physical_plan};
use crate::error::{BustubxError, BustubxResult};
use crate::optimizer::LogicalOptimizer;
use crate::planner::logical_plan::LogicalPlan;
use crate::planner::PhysicalPlanner;
use crate::{
    buffer::BufferPoolManager,
    catalog::Catalog,
    execution::{ExecutionContext, ExecutionEngine},
    planner::{LogicalPlanner, PlannerContext},
    storage::{DiskManager, Tuple},
};

pub struct Database {
    disk_manager: Arc<DiskManager>,
    pub(crate) catalog: Catalog,
    temp_dir: Option<TempDir>,
}
impl Database {
    pub fn new_on_disk(db_path: &str) -> BustubxResult<Self> {
        let disk_manager = Arc::new(DiskManager::try_new(db_path)?);
        let buffer_pool = BufferPoolManager::new(TABLE_HEAP_BUFFER_POOL_SIZE, disk_manager.clone());

        let mut catalog = Catalog::new(buffer_pool);

        let mut db = Self {
            disk_manager,
            catalog,
            temp_dir: None,
        };
        load_catalog_data(&mut db)?;
        Ok(db)
    }

    pub fn new_temp() -> BustubxResult<Self> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().join("test.db");
        let disk_manager =
            Arc::new(DiskManager::try_new(temp_path.to_str().ok_or(
                BustubxError::Internal("Invalid temp path".to_string()),
            )?)?);
        let buffer_pool = BufferPoolManager::new(TABLE_HEAP_BUFFER_POOL_SIZE, disk_manager.clone());

        let mut catalog = Catalog::new(buffer_pool);

        let mut db = Self {
            disk_manager,
            catalog,
            temp_dir: Some(temp_dir),
        };
        load_catalog_data(&mut db)?;
        Ok(db)
    }

    pub fn run(&mut self, sql: &str) -> BustubxResult<Vec<Tuple>> {
        let logical_plan = self.create_logical_plan(sql)?;
        debug!(
            "Logical Plan: \n{}",
            pretty_format_logical_plan(&logical_plan)
        );

        let optimized_logical_plan = LogicalOptimizer::new().optimize(&logical_plan)?;
        debug!(
            "Optimized Logical Plan: \n{}",
            pretty_format_logical_plan(&logical_plan)
        );

        // logical plan -> physical plan
        let physical_plan = PhysicalPlanner::new().create_physical_plan(optimized_logical_plan);
        debug!(
            "Physical Plan: \n{}",
            pretty_format_physical_plan(&physical_plan)
        );

        let execution_ctx = ExecutionContext::new(&mut self.catalog);
        let mut execution_engine = ExecutionEngine {
            context: execution_ctx,
        };
        let tuples = execution_engine.execute(Arc::new(physical_plan))?;
        Ok(tuples)
    }

    pub fn create_logical_plan(&mut self, sql: &str) -> BustubxResult<LogicalPlan> {
        // sql -> ast
        let stmts = crate::parser::parse_sql(sql)?;
        if stmts.len() != 1 {
            return Err(BustubxError::NotSupport(
                "only support one sql statement".to_string(),
            ));
        }
        let stmt = &stmts[0];
        let mut planner = LogicalPlanner {
            context: PlannerContext {
                catalog: &self.catalog,
            },
        };
        // ast -> logical plan
        planner.plan(stmt)
    }

    pub fn flush(&mut self) -> BustubxResult<()> {
        // TODO flush buffer pool
        todo!()
    }
}
