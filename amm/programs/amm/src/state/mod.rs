use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Config {
    pub seed
}
