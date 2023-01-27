use anchor_lang::prelude::*;

declare_id!("5pLRRFDgRaQUHoHUuKNjvj5worF9ZuroqKcyVvMjyNgL");

#[program]
pub mod solana_eksi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
