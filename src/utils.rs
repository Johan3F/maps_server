pub fn to_anyhow<T>(error: T) -> anyhow::Error
where
    T: ToString,
{
    anyhow::anyhow!(error.to_string())
}
