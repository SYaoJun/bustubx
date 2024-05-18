use crate::error::BustubxError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt64,
    // VarlenType,
    // DecimalType
    // NumericType
}

impl DataType {
    // 实现了一个DataType中的函数
    pub fn type_size(&self) -> usize {
        match self {
            DataType::Boolean => 1,
            DataType::Int8 => 1,
            DataType::Int16 => 2,
            DataType::Int32 => 4,
            DataType::Int64 => 8,
            DataType::UInt64 => 8,
        }
    }
}
// 有一个TryFrom特种，DataType去实现了这个特征
impl TryFrom<&sqlparser::ast::DataType> for DataType {
    type Error = BustubxError;

    fn try_from(value: &sqlparser::ast::DataType) -> Result<Self, Self::Error> {
        match value {
            sqlparser::ast::DataType::Boolean => Ok(DataType::Boolean),
            sqlparser::ast::DataType::TinyInt(_) => Ok(DataType::Int8),
            sqlparser::ast::DataType::SmallInt(_) => Ok(DataType::Int16),
            sqlparser::ast::DataType::Int(_) => Ok(DataType::Int32),
            sqlparser::ast::DataType::BigInt(_) => Ok(DataType::Int64),
            sqlparser::ast::DataType::UnsignedBigInt(_) => Ok(DataType::UInt64),
            _ => Err(BustubxError::NotSupport(format!(
                "unsupported datatype: {}",
                value
            ))),
        }
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
