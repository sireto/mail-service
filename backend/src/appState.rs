use diesel::r2d2::{ ConnectionManager, Pool };

use diesel::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// use crate::prisma::PrismaClient;

// pub struct AppState {
//     pub db: PrismaClient,
// }

// impl AppState {
//     pub fn new(db: PrismaClient) -> Self {
//         AppState { db }
//     }
// }

// use crate::prisma::PrismaClient;

pub struct AppState {
    pub db_pool: DbPool,
}

impl AppState {
    pub fn new(db_pool: DbPool) -> Self {
        AppState { db_pool }
    }
}