use async_trait::async_trait;
use crate::Database;

#[async_trait]
pub trait Seeder: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run(&self, db: &Database) -> anyhow::Result<()>;
}

pub struct SeedRunner {
    seeds: Vec<Box<dyn Seeder>>,
}

impl SeedRunner {
    pub fn new() -> Self {
        Self { seeds: Vec::new() }
    }

    pub fn add(&mut self, seed: Box<dyn Seeder>) {
        self.seeds.push(seed);
    }

    pub async fn run_all(&self, db: &Database) -> anyhow::Result<()> {
        for seed in &self.seeds {
            seed.run(db).await?;
        }
        Ok(())
    }
}

impl Default for SeedRunner {
    fn default() -> Self {
        Self::new()
    }
}
