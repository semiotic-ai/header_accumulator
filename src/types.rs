use alloy_primitives::Uint;
use alloy_primitives::B256;
use ethportal_api::{types::execution::accumulator::HeaderRecord, Header};
use sf_protos::ethereum::r#type::v2::Block;
use sf_protos::ethereum::r#type::v2::BlockHeader;

use crate::errors::EraValidateError;

#[derive(Clone)]
pub struct ExtHeaderRecord {
    pub block_hash: B256,
    pub total_difficulty: Uint<256, 4>,
    pub block_number: u64,
    pub full_header: Option<Header>,
}

impl From<ExtHeaderRecord> for HeaderRecord {
    fn from(
        ExtHeaderRecord {
            block_hash,
            total_difficulty,
            ..
        }: ExtHeaderRecord,
    ) -> Self {
        HeaderRecord {
            block_hash,
            total_difficulty,
        }
    }
}

impl TryFrom<ExtHeaderRecord> for Header {
    type Error = EraValidateError;

    fn try_from(ext: ExtHeaderRecord) -> Result<Self, Self::Error> {
        ext.full_header
            .ok_or(EraValidateError::ExtHeaderRecordError)
    }
}

impl From<&ExtHeaderRecord> for HeaderRecord {
    fn from(ext: &ExtHeaderRecord) -> Self {
        HeaderRecord {
            block_hash: ext.block_hash,
            total_difficulty: ext.total_difficulty,
        }
    }
}

/// Decodes a [`ExtHeaderRecord`] from a [`Block`]. A [`BlockHeader`] must be present in the block,
/// otherwise validating headers won't be possible
impl TryFrom<&Block> for ExtHeaderRecord {
    type Error = EraValidateError;

    fn try_from(block: &Block) -> Result<Self, Self::Error> {
        let header: &BlockHeader = block
            .header
            .as_ref()
            .ok_or(EraValidateError::HeaderDecodeError)?;

        let total_difficulty = header
            .total_difficulty
            .as_ref()
            .ok_or(EraValidateError::HeaderDecodeError)?;

        Ok(ExtHeaderRecord {
            block_number: block.number,
            block_hash: B256::from_slice(&block.hash),
            total_difficulty: Uint::from_be_slice(total_difficulty.bytes.as_slice()),
            full_header: Some(block.try_into()?),
        })
    }
}
