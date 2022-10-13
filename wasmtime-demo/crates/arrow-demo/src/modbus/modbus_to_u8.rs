use std::sync::Arc;
use arrow::array::ArrayBuilder;
use arrow::datatypes::Schema;

pub type SharedSchema = Arc(Schema);

/// Columnar Batch buffer that assists creating `RecordBatches`
pub struct MutableRecordBatch {
    arrays: Vec<Box<dyn ArrayBuilder>>,
    schema: Arc<Schema>,
}

fn read_modbus_to_arrow(schema: SharedSchema, records: MutableRecordBatch) {

}