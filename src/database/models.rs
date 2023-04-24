use sqlx::{postgres::PgPool, Executor};

const MIGRATION_DATABASE_SQL: &str = "
BEGIN;

CREATE TABLE IF NOT EXISTS agents (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    name TEXT NOT NULL,

    default_interaction_user_name TEXT NOT NULL,
    default_interaction_constitution TEXT NOT NULL,
    default_interaction_memory_size INTEGER NOT NULL
);


CREATE TABLE IF NOT EXISTS interactions (
    id UUID PRIMARY KEY,
    
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    user_name TEXT NOT NULL,
    constitution TEXT NOT NULL,
    
    short_term_memory TEXT NOT NULL,
    default_long_term_memory_size INTEGER NOT NULL,
    
    agent_id UUID REFERENCES agents(id)
);


CREATE TABLE IF NOT EXISTS interaction_blocks (
    id UUID PRIMARY KEY,
    
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    
    interaction_id UUID NOT NULL REFERENCES interactions(id),
    
    role TEXT NOT NULL,
    content TEXT NOT NULL,

    name TEXT
);


CREATE TABLE IF NOT EXISTS meta (
    id UUID PRIMARY KEY,

    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    default_interaction_id UUID NOT NULL REFERENCES interactions(id)
);

COMMIT;
";

pub async fn migrate_database_with_pg_pool(pool: &PgPool) {
    pool.execute(MIGRATION_DATABASE_SQL).await.unwrap();
    // query(MIGRATION_DATABASE_SQL).execute(pool).await.unwrap();
}
