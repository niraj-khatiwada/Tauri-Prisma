use std::sync::Arc;
use tauri::State;

use crate::prisma::PrismaClient;

pub type DbState<'a> = State<'a, Arc<PrismaClient>>;
