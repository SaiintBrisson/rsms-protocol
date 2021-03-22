use uuid::Uuid;

use crate::{ProtocolSupportDeserializer, ProtocolSupportSerializer};

impl ProtocolSupportSerializer for Uuid {
    fn calculate_len(&self) -> usize {
        16
    }

    fn serialize<W: std::io::Write>(&self, dst: &mut W) -> std::io::Result<()> {
        self.as_u128().serialize(dst)
    }
}

impl ProtocolSupportDeserializer for Uuid {
    fn deserialize<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        Ok(Uuid::from_u128(ProtocolSupportDeserializer::deserialize(
            src,
        )?))
    }
}
