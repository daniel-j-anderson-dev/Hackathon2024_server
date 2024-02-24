pub mod handlers;
pub mod session;

use std::{sync::Arc, collections::HashMap};
use tokio::{net::TcpListener, sync::Mutex};
use uuid::Uuid;

use crate::{handlers::*, session::*};

pub type Sessions = Arc<Mutex<HashMap<Uuid, TicTacToeSession>>>;
