use anchor_lang::prelude::*;

// declare rs modules
pub mod constant;
pub mod error;
pub mod states;

// import custom rs modules
    // * use crate brings l1-l13 to the 1st scope
use crate::{constant::*, error::*, states::*};

declare_id!("4roLsp9UCWbpm9tgj7yw94eDn6jGWJncvLq2fTbZdeWd");

#[program]
pub mod clever_todo {
    // * use super brings l1-l13 to the clever_todo{} scope
    use super::*;
    
        // * Initialize User
            // - ADD a USER_PROFILE to the blockchain
            // - ADD VALUES to the DEFAULT data
        pub fn initialize_user(
            ctx: Context<InitializeUser>
        ) -> Result<()> {
            // Initialize USER_PROFILE with default data           
        }


    
        // * ADD toDo 
            // - ADD a toDo to the blockchain
        
        // * MARK a toDo
            // - UPDATE the STATE of a toDo in the blockchain
        
        // * DELETE toDo
            // - REMOVE a toDo in the blockchain
}

#[derive(Accounts)]
#[instruction()]
// <'info> -> means it will exist in memory as long it needs to
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG,authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem:size_of::<UserProfile>(),
    )]
    // Box -> place to store memory
    pub user_profile: Box<Account<'info, UserProfile>>,
}