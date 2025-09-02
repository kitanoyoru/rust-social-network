use anyhow::Result;

pub struct Cache;

impl Cache {
    pub async fn new(_redis_url: &str) -> Result<Self> {
        println!("Cache initialized (dummy)");
        Ok(Self)
    }

    pub async fn get<T>(&mut self, _key: &str) -> Result<Option<T>> {
        println!("Cache get (dummy)");
        Ok(None)
    }

    pub async fn set<T>(&mut self, _key: &str, _value: &T, _ttl: Option<std::time::Duration>) -> Result<()> {
        println!("Cache set (dummy)");
        Ok(())
    }

    pub async fn delete(&mut self, _key: &str) -> Result<()> {
        println!("Cache delete (dummy)");
        Ok(())
    }

    pub async fn exists(&mut self, _key: &str) -> Result<bool> {
        println!("Cache exists (dummy)");
        Ok(false)
    }

    pub async fn flush_all(&mut self) -> Result<()> {
        println!("Cache flush (dummy)");
        Ok(())
    }
}
