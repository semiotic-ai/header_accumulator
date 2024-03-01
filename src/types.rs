use ethereum_types::{H256, U256};
use ethportal_api::types::execution::accumulator::HeaderRecord;

#[derive(Clone)]
pub struct ExtHeaderRecord {
    pub block_hash: H256,
    pub total_difficulty: U256,
    pub block_number: u64,
}

impl From<ExtHeaderRecord> for HeaderRecord {
    fn from(ext: ExtHeaderRecord) -> Self {
        HeaderRecord {
            block_hash: ext.block_hash,
            total_difficulty: ext.total_difficulty,
        }
    }
}

impl From<HeaderRecord> for ExtHeaderRecord {
    fn from(hr: HeaderRecord) -> Self {
        ExtHeaderRecord {
            block_hash: hr.block_hash,
            total_difficulty: hr.total_difficulty,
            block_number: 0, // Default value or decide based on context
        }
    }
}