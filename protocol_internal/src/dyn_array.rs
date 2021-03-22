use crate::{ProtocolSupportDeserializer, ProtocolSupportSerializer, RangeValidatedSupport};

pub struct DynArray;

impl DynArray {
    #[inline(always)]
    pub fn calculate_len<T: ProtocolSupportSerializer>(value: &Vec<T>) -> usize {
        value
            .iter()
            .map(<T as ProtocolSupportSerializer>::calculate_len)
            .fold(0, |acc, x| acc + x)
    }

    pub fn deserialize<R: std::io::Read, T: ProtocolSupportDeserializer>(
        src: &mut R,
    ) -> std::io::Result<Vec<T>> {
        let mut buf = Vec::new();

        loop {
            match <T as ProtocolSupportDeserializer>::deserialize(src) {
                Ok(out) => buf.push(out),
                Err(err) => match err.kind() {
                    std::io::ErrorKind::UnexpectedEof => break,
                    _ => Err(err)?,
                },
            }
        }

        Ok(buf)
    }

    pub fn serialize<W: std::io::Write, T: ProtocolSupportSerializer>(
        value: &Vec<T>,
        dst: &mut W,
    ) -> std::io::Result<()> {
        for e in value {
            <T as ProtocolSupportSerializer>::serialize(e, dst)?;
        }

        Ok(())
    }
}

impl<T: ProtocolSupportDeserializer> RangeValidatedSupport<Vec<T>> for DynArray {
    fn deserialize<R: std::io::Read>(
        src: &mut R,
        min: usize,
        max: usize,
    ) -> std::io::Result<Vec<T>> {
        let mut buf = Vec::with_capacity(max);

        loop {
            if max < buf.len() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("dynarray is bigger than max {}", max),
                ));
            }

            match <T as ProtocolSupportDeserializer>::deserialize(src) {
                Ok(out) => buf.push(out),
                Err(err) => match err.kind() {
                    std::io::ErrorKind::UnexpectedEof => break,
                    _ => Err(err)?,
                },
            }
        }

        if min > buf.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("dynarray is smaller than min {}", min),
            ));
        }

        Ok(buf)
    }
}
