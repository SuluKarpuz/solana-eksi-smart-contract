use anchor_lang::prelude::*;

declare_id!("5pLRRFDgRaQUHoHUuKNjvj5worF9ZuroqKcyVvMjyNgL");

#[program]
pub mod solana_eksi {
    use super::*;

    pub fn send_entry(ctx: Context<SendEntry>, topic: String, content: String) -> Result<()>{
        let entry: &mut Account<Entry> = &mut ctx.accounts.entry;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        entry.author = *author.key;
        entry.timestamp = clock.unix_timestamp;
        entry.topic = topic;
        entry.content = content;
        entry.upvotes = 0;
        entry.downvotes = 0;

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, entry_id: Pubkey, is_upvote: bool) -> Result<()> {
        let entry: &mut Account<Entry> = &mut ctx.accounts.entry;

        if is_upvote {
            entry.upvotes += 1;
        } else {
            entry.downvotes += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendEntry<'info> {
    #[account(init, payer = author, space = Entry::LEN)]
    pub entry: Account<'info, Entry>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Entry {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
    pub upvotes: i64,
    pub downvotes: i64,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const VOTES_LENGTH: usize = 8;

impl Entry {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // Content.
        + VOTES_LENGTH * 2; // Upvotes and downvotes.
}

