use clap::{Parser, Subcommand};
use diesel::Connection;
use diesel_migrations::{
    EmbeddedMigrations, MigrationHarness, embed_migrations,
};

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("../../common/migrations");

#[derive(Parser)]
struct MigrationApp {
    #[clap(subcommand)]
    command: Cmds,

    /// The URL of the database to migrate.
    #[clap(long, env = "DATABASE_URL")]
    database_url: String,
}

#[derive(Subcommand)]
enum Cmds {
    Run,
    Reset,
}

fn main() {
    let args = MigrationApp::parse();

    let mut conn = diesel::pg::PgConnection::establish(&args.database_url)
        .expect("Failed to connect to database");

    match args.command {
        Cmds::Run => {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("Failed to run migrations");
        }
        Cmds::Reset => {
            conn.revert_all_migrations(MIGRATIONS)
                .expect("Failed to revert migrations");
        }
    }
}
