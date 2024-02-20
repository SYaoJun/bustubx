use log::debug;
use std::sync::{atomic::AtomicU32, Arc, Mutex};

use crate::catalog::SchemaRef;
use crate::expression::ExprTrait;
use crate::planner::logical_plan::OrderByExpr;
use crate::{
    execution::{ExecutionContext, VolcanoExecutor},
    storage::Tuple,
    BustubxError, BustubxResult,
};

use super::PhysicalPlan;

#[derive(Debug)]
pub struct PhysicalSort {
    pub order_bys: Vec<OrderByExpr>,
    pub input: Arc<PhysicalPlan>,

    all_tuples: Mutex<Vec<Tuple>>,
    cursor: AtomicU32,
}
impl PhysicalSort {
    pub fn new(order_bys: Vec<OrderByExpr>, input: Arc<PhysicalPlan>) -> Self {
        PhysicalSort {
            order_bys,
            input,
            all_tuples: Mutex::new(Vec::new()),
            cursor: AtomicU32::new(0),
        }
    }
}
impl VolcanoExecutor for PhysicalSort {
    fn init(&self, context: &mut ExecutionContext) -> BustubxResult<()> {
        debug!("init sort executor");
        self.input.init(context)?;
        // TODO move to next method
        // load all tuples from input
        let mut all_tuples = Vec::new();
        loop {
            let next_tuple = self.input.next(context)?;
            if next_tuple.is_none() {
                break;
            }
            all_tuples.push(next_tuple.unwrap());
        }

        // TODO handle error during sorting
        // sort all tuples
        all_tuples.sort_by(|a, b| {
            let mut ordering = std::cmp::Ordering::Equal;
            let mut index = 0;
            while ordering == std::cmp::Ordering::Equal && index < self.order_bys.len() {
                let a_value = self.order_bys[index].expr.evaluate(a).unwrap();
                let b_value = self.order_bys[index].expr.evaluate(b).unwrap();
                ordering = if self.order_bys[index].asc {
                    a_value.partial_cmp(&b_value)
                } else {
                    b_value.partial_cmp(&a_value)
                }
                .ok_or(BustubxError::Execution(format!(
                    "Can not compare {} and {}",
                    a_value, b_value
                )))
                .unwrap();
                index += 1;
            }
            ordering
        });
        *self.all_tuples.lock().unwrap() = all_tuples;
        self.cursor.store(0, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    fn next(&self, context: &mut ExecutionContext) -> BustubxResult<Option<Tuple>> {
        let cursor = self
            .cursor
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst) as usize;
        if cursor >= self.all_tuples.lock().unwrap().len() {
            return Ok(None);
        }
        return Ok(self
            .all_tuples
            .lock()
            .unwrap()
            .get(cursor)
            .map(|t| t.clone()));
    }

    fn output_schema(&self) -> SchemaRef {
        self.input.output_schema()
    }
}

impl std::fmt::Display for PhysicalSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sort")
    }
}
