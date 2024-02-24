pub mod handlers;
pub mod session;

use std::{sync::Arc, collections::HashMap};
use tokio::sync::Mutex;
use uuid::Uuid;

pub use crate::{handlers::*, session::*};

pub type Sessions = Arc<Mutex<HashMap<Uuid, TicTacToeSession>>>;
