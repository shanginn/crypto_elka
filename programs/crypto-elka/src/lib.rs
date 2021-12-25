use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("P67cjVRgibTH2q6vrfvPPnLkmxjgrLHM4QJGBxFAJwh");

#[error]
pub enum ErrorCode {
    #[msg("The provided message should be 280 characters long maximum.")]
    MessageTooLong,
}

#[program]
pub mod crypto_elka {
    use super::*;
    pub fn place_ball(
        ctx: Context<PlaceBall>,
        place: u8,
        message: String,
    ) -> ProgramResult {
        let ball: &mut Account<Ball> = &mut ctx.accounts.ball;
        let creator: &Signer = &ctx.accounts.creator;
        let ball_nft: &Signer = &ctx.accounts.ball_nft;
        let clock: Clock = Clock::get().unwrap();

        if message.chars().count() > 280 {
            return Err(ErrorCode::MessageTooLong.into());
        }

        ball.creator = *creator.key;
        ball.timestamp = clock.unix_timestamp;
        ball.message = message;
        ball.place = place;
        ball.ball = *ball_nft.key;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlaceBall<'info> {
    #[account(init, payer = creator, space = Ball::LEN)]
    pub ball: Account<'info, Ball>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub ball_nft: Signer<'info>,
    pub system_program: Program<'info, System>,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const MAX_CONTENT_LENGTH: usize = 280 * 4;
const PLACE_SIZE: usize = 1;

#[account]
pub struct Ball {
    pub creator: Pubkey,
    pub ball: Pubkey,
    pub message: String,
    pub place: u8,
    pub timestamp: i64,
}

impl Ball {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Creator
        + PUBLIC_KEY_LENGTH // Ball NFT
        + TIMESTAMP_LENGTH // Timestamp.
        + PLACE_SIZE // Place in tree
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}