use anchor_lang::prelude::*;

declare_id!("AxT6afmENm2MjEkqXgDC2UMgLFebTatvM1eWFSkNJL72");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    // Get a reference to the account and increment total_gifs.  
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    
    // Build the struct.
    let item = ItemStruct {
        gif_link: gif_link.to_string(),
        user_address: *user.to_account_info().key,
    };

    // Add it to the gift_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
      Ok(())
  }

  pub fn remove_gif(ctx: Context<RemoveGif>, remove_gif_link: String) -> Result <()> {
    // Get a reference to the account and increment total_gifs.  
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    
    let mut vec: Vec<ItemStruct> = Vec::new();
    let mut a: u64 = 0;
    for i in base_account.gif_list.iter_mut() {
      if (i.gif_link!=remove_gif_link.to_string() && i.user_address== *user.to_account_info().key) || i.user_address!= *user.to_account_info().key
      {
        vec.push(i.clone());
        a +=1;
      }
    }
    base_account.gif_list=vec;
    base_account.total_gifs=a;
    Ok(())
  }
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}