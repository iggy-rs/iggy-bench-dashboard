use anyhow::Result;

pub trait Validatable {
    fn validate(&self) -> Result<()>;
}
