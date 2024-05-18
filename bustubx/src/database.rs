use comfy_table::Table;
use std::sync::Arc;
use tempfile::TempDir;

use crate::buffer::TABLE_HEAP_BUFFER_POOL_SIZE;
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
    catalog: Catalog,
    temp_dir: Option<TempDir>,
}
impl Database {
    pub fn new_on_disk(db_path: &str) -> BustubxResult<Self> {
        let disk_manager = Arc::new(DiskManager::try_new(&db_path)?);
        let buffer_pool_manager =
            BufferPoolManager::new(TABLE_HEAP_BUFFER_POOL_SIZE, disk_manager.clone(), 2);
        // TODO load catalog from disk
        let catalog = Catalog::new(buffer_pool_manager);
        Ok(Self {
            disk_manager,
            catalog,
            temp_dir: None,
        })
    }

    pub fn new_temp() -> BustubxResult<Self> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().join("test.db");
        let disk_manager =
            Arc::new(DiskManager::try_new(temp_path.to_str().ok_or(
                BustubxError::Internal("Invalid temp path".to_string()),
            )?)?);
        let buffer_pool_manager =
            BufferPoolManager::new(TABLE_HEAP_BUFFER_POOL_SIZE, disk_manager.clone(), 2);
        let catalog = Catalog::new(buffer_pool_manager);
        Ok(Self {
            disk_manager,
            catalog,
            temp_dir: Some(temp_dir),
        })
    }

    pub fn run(&mut self, sql: &str) -> BustubxResult<Vec<Tuple>> {
        // 把sql转换为逻辑计划
        let v: Vec<Tuple> = vec![];
        if sql == "\\dt" {
            // show_all_tables();
            // 1. 获取系统表中的所有表
            let table_names = self.catalog.get_table_names();
            // 2. 打印header
            println!("oid | name | schema");
            // 3. 遍历表
            for table_name in table_names {
                if let Some(table_info) = self.catalog.get_table_by_name(table_name.as_str()) {
                    println!(
                        "{} | {} | {}",
                        table_info.oid,
                        table_info.name,
                        table_info.schema.to_string()
                    );
                }
            }
            return Ok(v);
        } else if sql == "\\di" {
            // TODO: 功能还不完善
            let table_names = self.catalog.get_table_names();
            // 2. 打印header
            println!("table_name     | index_oid    | index_name    | index_cols");
            // 3. 遍历表
            for table_name in table_names {
                // 4. 遍历索引
                let index_infos = self.catalog.get_table_indexes(table_name.as_str());
                for index_info in index_infos {
                    println!(
                        "{} | {} | {} | {}",
                        table_name,
                        index_info.oid,
                        index_info.name,
                        index_info.key_schema.to_string()
                    );
                }
                // if let Some(table_info) = self.catalog.get_table_by_name(table_name.as_str()) {

                // }
            }
            return Ok(v);
        }
        // 逻辑计划做了什么事情哟？

        let logical_plan = self.create_logical_plan(sql)?;
        println!(
            "Logical Plan: \n{}",
            pretty_format_logical_plan(&logical_plan)
        );
        // 直接就到优化器了呀
        let optimized_logical_plan = LogicalOptimizer::new().optimize(&logical_plan)?;

        // logical plan -> physical plan
        let physical_plan = PhysicalPlanner::new().create_physical_plan(optimized_logical_plan);
        println!(
            "Physical Plan: \n{}",
            pretty_format_physical_plan(&physical_plan)
        );
        // 执行器
        let execution_ctx = ExecutionContext::new(&mut self.catalog);
        let mut execution_engine = ExecutionEngine {
            context: execution_ctx,
        };
        // 开始执行
        let tuples = execution_engine.execute(Arc::new(physical_plan))?;
        // println!("execution result: {:?}", tuples);
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
        planner.plan(&stmt)
    }
}
