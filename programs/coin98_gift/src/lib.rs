use anchor_lang::prelude::*;

pub mod constant;
pub mod context;
pub mod error;
pub mod event;
pub mod state;

use crate::constant::*;
use crate::context::*;
use crate::error::*;
use crate::event::*;
use crate::state::*;

#[cfg(feature = "localhost")]
declare_id!("7KZpqbt3uTD6DZ2gJpZRjTxVT6NULXVHYPcVw12ivBvE");

#[cfg(all(not(feature = "localhost")))]
declare_id!("MT2ok2hRsikpmQPJjjFVh73tgh1bkvSMnea1ujEE1Kp");

#[program]
mod coin98_gift {
  use super::*;
}

