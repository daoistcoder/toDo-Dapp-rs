use anchor_lang::prelude::*;

declare_id!("4roLsp9UCWbpm9tgj7yw94eDn6jGWJncvLq2fTbZdeWd");

#[program]
pub mod to_do_d_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
