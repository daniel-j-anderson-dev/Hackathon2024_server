pub mod handlers;
pub mod session;
use crate::session::*;

use tokio::sync::Mutex;
use uuid::Uuid;
use std::{collections::HashMap, sync::Arc};

pub type Sessions = Arc<Mutex<HashMap<Uuid, TicTacToeSession>>>;

