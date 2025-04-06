use bon::Builder;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
pub struct Config {
    pub serializer_channel_buffer_size: usize,
    pub deserializer_channel_buffer_size: usize,
    pub serializer_num_threads: u8,
    pub deserializer_num_threads: u8,
    pub request_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            serializer_channel_buffer_size: 16,
            deserializer_channel_buffer_size: 256,
            serializer_num_threads: 4,
            deserializer_num_threads: 32,
            request_timeout: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config_build() {
        let config = Config::builder()
            .serializer_channel_buffer_size(512)
            .deserializer_channel_buffer_size(512)
            .serializer_num_threads(16)
            .deserializer_num_threads(16)
            .request_timeout(60)
            .build();
        assert_eq!(config.serializer_channel_buffer_size, 512);
        assert_eq!(config.deserializer_channel_buffer_size, 512);
    }
}
