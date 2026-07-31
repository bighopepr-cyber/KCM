use kcm_core::types::KcmError;

pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError>;
    fn decompress(&self, data: &[u8], expected_size: usize) -> Result<Vec<u8>, KcmError>;
}

pub struct ZstdCompressor {
    level: i32,
}

impl ZstdCompressor {
    pub fn new(level: i32) -> Self {
        ZstdCompressor { level }
    }

    pub fn default_level() -> Self {
        ZstdCompressor { level: 3 }
    }
}

impl Compressor for ZstdCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError> {
        zstd::encode_all(data, self.level).map_err(|e| KcmError::Io(e.to_string()))
    }

    fn decompress(&self, data: &[u8], _expected_size: usize) -> Result<Vec<u8>, KcmError> {
        zstd::decode_all(data).map_err(|e| KcmError::Io(e.to_string()))
    }
}

pub struct Lz4Compressor;

impl Lz4Compressor {
    pub fn default_level() -> Self {
        Lz4Compressor
    }
}

impl Compressor for Lz4Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError> {
        lz4::block::compress(data, None, false).map_err(|e| KcmError::Io(e.to_string()))
    }

    fn decompress(&self, data: &[u8], expected_size: usize) -> Result<Vec<u8>, KcmError> {
        lz4::block::decompress(data, Some(expected_size as i32))
            .map_err(|e| KcmError::Io(e.to_string()))
    }
}

pub struct NoopCompressor;

impl Compressor for NoopCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError> {
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8], _expected_size: usize) -> Result<Vec<u8>, KcmError> {
        Ok(data.to_vec())
    }
}

pub struct RleCompressor;

impl Compressor for RleCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, KcmError> {
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

    fn decompress(&self, data: &[u8], _expected_size: usize) -> Result<Vec<u8>, KcmError> {
        let mut result = Vec::new();
        let mut i = 0;
        while i + 5 <= data.len() {
            let value = data[i];
            i += 1;
            let count = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            i += 4;
            for _ in 0..count {
                result.push(value);
            }
        }
        Ok(result)
    }
}

pub fn hash_blake3(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

pub fn hash_blake3_hex(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_compress_decompress() {
        let compressor = ZstdCompressor::default_level();
        let data = b"Hello, World! This is a test of zstd compression in KCM.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_lz4_compress_decompress() {
        let compressor = Lz4Compressor::default_level();
        let data = b"Hello, World! This is a test of lz4 compression in KCM.";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_noop_compress_decompress() {
        let compressor = NoopCompressor;
        let data = b"Hello, World!";
        let compressed = compressor.compress(data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data.to_vec(), decompressed);
        assert_eq!(data.to_vec(), compressed);
    }

    #[test]
    fn test_blake3_hash() {
        let data = b"test data";
        let hash = hash_blake3(data);
        assert_eq!(hash.len(), 32);

        let hash2 = hash_blake3(data);
        assert_eq!(hash, hash2);

        let hash3 = hash_blake3(b"different data");
        assert_ne!(hash, hash3);
    }

    #[test]
    fn test_blake3_hex() {
        let data = b"test";
        let hex = hash_blake3_hex(data);
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_zstd_repetitive_data() {
        let compressor = ZstdCompressor::new(10);
        let data: Vec<u8> = vec![42u8; 10000];
        let compressed = compressor.compress(&data).unwrap();
        assert!(compressed.len() < data.len());
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data, decompressed);
    }
}
