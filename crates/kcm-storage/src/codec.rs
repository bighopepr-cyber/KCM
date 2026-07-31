use kcm_core::types::*;

pub trait Codec<T: Copy> {
    fn encode(&self, data: &[T]) -> Result<Vec<u8>, KcmError>;
    fn decode(&self, data: &[u8], count: usize) -> Result<Vec<T>, KcmError>;
}

pub struct DeltaCodec;

impl Codec<i64> for DeltaCodec {
    fn encode(&self, data: &[i64]) -> Result<Vec<u8>, KcmError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let mut deltas = Vec::with_capacity(data.len());
        deltas.push(data[0]);

        for i in 1..data.len() {
            deltas.push(data[i] - data[i - 1]);
        }

        let mut result = Vec::with_capacity(deltas.len() * 8);
        for delta in &deltas {
            result.extend_from_slice(&delta.to_le_bytes());
        }

        Ok(result)
    }

    fn decode(&self, data: &[u8], count: usize) -> Result<Vec<i64>, KcmError> {
        if count == 0 || data.is_empty() {
            return Ok(vec![]);
        }

        let mut deltas = Vec::with_capacity(count);
        let mut i = 0;
        while i + 8 <= data.len() && deltas.len() < count {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&data[i..i + 8]);
            deltas.push(i64::from_le_bytes(bytes));
            i += 8;
        }

        let mut result = Vec::with_capacity(count);
        let mut current = 0i64;

        for delta in deltas {
            current += delta;
            result.push(current);
        }

        Ok(result)
    }
}

pub struct RleCodec;

impl Codec<u8> for RleCodec {
    fn encode(&self, data: &[u8]) -> Result<Vec<u8>, KcmError> {
        let mut result = Vec::new();

        let mut i = 0;
        while i < data.len() {
            let value = data[i];
            let mut count = 1u32;

            while i + (count as usize) < data.len()
                && data[i + count as usize] == value
                && count < u32::MAX
            {
                count += 1;
            }

            result.push(value);
            result.extend_from_slice(&count.to_le_bytes());
            i += count as usize;
        }

        Ok(result)
    }

    fn decode(&self, data: &[u8], _count: usize) -> Result<Vec<u8>, KcmError> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let value = data[i];
            i += 1;

            if i + 4 > data.len() {
                return Err(KcmError::Corrupted("Incomplete RLE entry".to_string()));
            }

            let count = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            i += 4;

            for _ in 0..count {
                result.push(value);
            }
        }

        Ok(result)
    }
}

pub struct GorillaCodec;

impl Codec<f64> for GorillaCodec {
    fn encode(&self, data: &[f64]) -> Result<Vec<u8>, KcmError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let mut result = Vec::new();
        let mut prev_bits = data[0].to_bits();
        result.extend_from_slice(&data[0].to_le_bytes());

        for &value in &data[1..] {
            let bits = value.to_bits();
            let xor = bits ^ prev_bits;

            result.extend_from_slice(&xor.to_le_bytes());
            prev_bits = bits;
        }

        Ok(result)
    }

    fn decode(&self, data: &[u8], count: usize) -> Result<Vec<f64>, KcmError> {
        if count == 0 {
            return Ok(vec![]);
        }

        let mut result = Vec::with_capacity(count);

        if data.len() < 8 {
            return Err(KcmError::Corrupted("Incomplete Gorilla data".to_string()));
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[0..8]);
        let mut prev_bits = u64::from_le_bytes(bytes);
        result.push(f64::from_bits(prev_bits));

        let mut i = 8;
        while result.len() < count && i + 8 <= data.len() {
            bytes.copy_from_slice(&data[i..i + 8]);
            let xor = u64::from_le_bytes(bytes);
            prev_bits ^= xor;
            result.push(f64::from_bits(prev_bits));
            i += 8;
        }

        Ok(result)
    }
}
