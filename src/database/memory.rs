use std::marker::PhantomData;

use chrono::Utc;
use sqlx::{postgres::PgPool, query};
use uuid::Uuid;

use crate::{
    agent::Agent,
    sdk::interactions::{Interaction, Meta, WithAgent, WithoutAgent},
};

use super::{engine::new_postgres_pool, models::migrate_database_with_pg_pool};

#[derive(Debug, Clone)]
pub struct MemoryEngine {
    pool: PgPool,
}

impl MemoryEngine {
    pub async fn new(database_url: String) -> Self {
        let pool = new_postgres_pool(database_url).await;

        migrate_database_with_pg_pool(&pool).await;

        Self { pool }
    }

    pub async fn new_defaults() -> Self {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string());

        Self::new(database_url).await
    }

    pub async fn new_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        let interaction = Interaction::new(user_name, constitution, memory_size);

        let res = query!(
            r#"
            INSERT INTO interactions (id, created_at, updated_at, user_name, short_term_memory_size)
            VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, updated_at, user_name, short_term_memory_size
            "#,
            interaction.id,
            interaction.created_at.naive_utc(),
            interaction.updated_at.naive_utc(),
            interaction.user_name,
            interaction.short_term_memory_size as i32,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Interaction::<WithoutAgent> {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            user_name: res.user_name,
            short_term_memory_size: res.short_term_memory_size as usize,
            short_term_memory: None,
            state: PhantomData,
            agent: None,
        }
    }

    pub async fn new_interaction_with_agent(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
        agent: &Agent,
    ) -> Interaction<WithAgent> {
        let mut interaction = self
            .new_interaction(user_name, constitution, memory_size)
            .await;

        interaction.set_agent(agent.to_owned()) // TODO: Check if it can be optimized
    }

    pub async fn update_constitution(&mut self, id: Uuid, constitution: String) -> Interaction {
        todo!()
    }

    pub async fn append_to_dynamic_memory(
        &mut self,
        id: Uuid,
        new_interaction: String,
    ) -> Interaction {
        todo!()
    }

    pub async fn set_short_term_memory(
        &mut self,
        interaction_id: Uuid,
        memory: String,
    ) -> Interaction {
        todo!()
    }

    pub async fn get_meta(&mut self) -> Option<Meta> {
        todo!()
    }

    pub async fn get_interaction(&mut self, id: Uuid) -> Option<Interaction> {
        todo!()
    }

    pub async fn set_default_interaction(&mut self, id: Uuid) -> Meta {
        todo!()
    }

    pub async fn get_default_interaction(&mut self) -> Option<Interaction> {
        todo!()
    }

    pub async fn get_or_create_default_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        todo!()
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction> {
        todo!()
    }
}
