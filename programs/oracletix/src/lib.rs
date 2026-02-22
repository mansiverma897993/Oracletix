use anchor_lang::prelude::*;

declare_id!("ReplaceWithYourProgramID");

#[program]
pub mod oracletix {
    use super::*;

    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        name: String,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;

        require!(name.len() <= Event::MAX_NAME_LEN, ErrorCode::NameTooLong);
        require!(start_time < end_time, ErrorCode::InvalidTimeRange);

        event.organizer = ctx.accounts.organizer.key();
        event.name = name;
        event.start_time = start_time;
        event.end_time = end_time;
        event.state = EventState::Created as u8;
        event.mint = Pubkey::default();
        event.attendance = 0;
        event.bump = ctx.bumps.event;

        Ok(())
    }

    pub fn attach_mint(ctx: Context<AttachMint>, mint: Pubkey) -> Result<()> {
        let event = &mut ctx.accounts.event;

        require_keys_eq!(
            event.organizer,
            ctx.accounts.organizer.key(),
            ErrorCode::Unauthorized
        );

        event.mint = mint;

        Ok(())
    }

    pub fn update_state(
        ctx: Context<UpdateState>,
        next_state: u8,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;

        require!(next_state <= EventState::Completed as u8, ErrorCode::InvalidState);
        require!(next_state >= event.state, ErrorCode::InvalidTransition);

        event.state = next_state;

        Ok(())
    }

    pub fn mark_attendance(ctx: Context<MarkAttendance>) -> Result<()> {
        let event = &mut ctx.accounts.event;

        require!(
            event.state == EventState::Live as u8,
            ErrorCode::EventNotLive
        );

        event.attendance = event
            .attendance
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(
        init,
        payer = organizer,
        space = 8 + Event::SIZE,
        seeds = [b"event", organizer.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,

    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AttachMint<'info> {
    #[account(mut, has_one = organizer)]
    pub event: Account<'info, Event>,

    pub organizer: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub event: Account<'info, Event>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MarkAttendance<'info> {
    #[account(mut)]
    pub event: Account<'info, Event>,

    pub attendee: Signer<'info>,
}

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub name: String,
    pub start_time: i64,
    pub end_time: i64,
    pub state: u8,
    pub mint: Pubkey,
    pub attendance: u32,
    pub bump: u8,
}

impl Event {
    pub const MAX_NAME_LEN: usize = 64;

    pub const SIZE: usize =
        32 +
        4 + Self::MAX_NAME_LEN +
        8 +
        8 +
        1 +
        32 +
        4 +
        1;
}

#[repr(u8)]
pub enum EventState {
    Created = 0,
    Live = 1,
    Completed = 2,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Invalid time")]
    InvalidTimeRange,
    #[msg("Invalid state")]
    InvalidState,
    #[msg("Invalid transition")]
    InvalidTransition,
    #[msg("Not live")]
    EventNotLive,
    #[msg("Overflow")]
    Overflow,
}
