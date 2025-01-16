use cache_lru::cache::Cache;

#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let mut cache = Cache::<String, String>::new(2);

    
        cache.put("key1".to_string(), "value1".to_string());
        let value = cache.get(&"key1".to_string());
        assert_eq!(value, Some(&"value1".to_string()));
    }

    #[test]
    fn test_deletion() {
        let mut cache = Cache::new(2);

        cache.put("key1".to_string(), "value1".to_string());
        cache.put("key2".to_string(), "value2".to_string());

        assert_eq!(cache.get(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), Some(&"value2".to_string()));

        cache.put("key3".to_string(), "value3".to_string());

        assert_eq!(cache.get(&"key2".to_string()), Some(&"value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()), Some(&"value3".to_string()));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }
}

