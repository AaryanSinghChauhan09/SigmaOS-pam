use ::pam::{pam, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = pam::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.start().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.stop().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-pam operational");
    Ok(())
}
