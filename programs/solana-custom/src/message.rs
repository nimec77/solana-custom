use anchor_lang::prelude::*;

// Constants defining sizes of account properties
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

#[account]
pub struct Message {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

impl Message {
    // Constant representing the total size of the Message account
    pub const LEN: usize = DISCRIMINATOR_LENGTH
    + PUBLIC_KEY_LENGTH // Author.
    + TIMESTAMP_LENGTH // Timestamp.
    + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
    + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}
