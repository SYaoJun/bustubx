use crate::catalog::SchemaRef;
use crate::expression::Expr;
use crate::planner::logical_plan_v2::LogicalPlanV2;
use std::sync::Arc;

#[derive(derive_new::new, Debug, Clone)]
pub struct Join {
    /// Left input
    pub left: Arc<LogicalPlanV2>,
    /// Right input
    pub right: Arc<LogicalPlanV2>,
    pub join_type: JoinType,
    pub condition: Option<Expr>,
    pub schema: SchemaRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    // select * from x inner join y on ...
    Inner,
    // select * from x left (outer) join y on ...
    LeftOuter,
    // select * from x right (outer) join y on ...
    RightOuter,
    // select * from x full (outer) join y on ...
    FullOuter,
    // select * from x, y
    // select * from x cross join y
    CrossJoin,
}
