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
            let user_profile = &mut ctx.accounts.user_profile; // declared user_profile var as the val from InitializeUser struct

            user_profile.authority = ctx.accounts.authority.key(); 
            user_profile.last_todo = 0;
            user_profile.todo_count = 0;

            Ok(())
        }
    
        // * ADD toDo 
            // - ADD a toDo to the blockchain
        pub fn add_todo(
            ctx: Context<AddTodo>,
            _content: String,
        ) -> Result<()> {
            // Initialize variables
            
            let todo_account = &mut ctx.accounts.todo_account; 
            let user_profile = &mut ctx.accounts.user_profile;

            // Fill the toDo_account struct w/ proper values
            todo_account.authority = ctx.accounts.authority.key();
            todo_account.idx = user_profile.last_todo;
            todo_account.content = _content;
            todo_account.marked = false;

            // Increase toDo idx for PDA
            user_profile.last_todo = user_profile.last_todo
            .checked_add(1) // check if it can be incremented by 1
            .unwrap();

            // Increase the total_todo count
            user_profile.todo_count = user_profile.todo_count
            .checked_add(1)
            .unwrap();

            Ok(())
             
        }
        
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
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    // Box -> place to store memory
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

// AddTodo Struct
#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}