use anchor_lang::prelude::*;

use crate::message::Message;

declare_id!("3wxi1Gg12VhCgYB5onA5mr28xYBK6szpvNdAowkDje8c");

mod message;

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(init, payer = author, space = Message::LEN)]
    pub message: Account<'info, Message>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = System::id())]
    pub system_program: Program<'info, System>,
}

#[program]
pub mod solana_custom {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn send_message(
        ctx: Context<SendMessage>,
        topic: String,
        content: String,
    ) -> ProgramResult {
        let message_account: &mut Account<Message> = &mut ctx.accounts.message;
        let sender: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        message_account.author = *sender.key;
        message_account.timestamp = clock.unix_timestamp;
        message_account.topic = topic;
        message_account.content = content;

        Ok(())
    }
}
