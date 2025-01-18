use crate::prisma::PrismaClient;

pub struct AppState {
    pub db: PrismaClient,
}

impl AppState {
    pub fn new(db: PrismaClient) -> Self {
        AppState { db }
    }
}