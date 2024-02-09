use anchor_lang::prelude::*;

declare_id!("C4m1216wrXuhEQ7FqoN7ySF5qeZLfrEyYtxfYCEYCkKU");

#[program]
pub mod multi_level_squads {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
