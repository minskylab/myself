use std::{marker::PhantomData, str::FromStr};

use chrono::Utc;
use sqlx::{postgres::PgPool, query};
use uuid::Uuid;

use crate::{
    // agent::{Agent, DefaultInteraction},
    backend::core::AgentBackend,
    sdk::agent::{Agent, DefaultInteraction},
    sdk::interactions::{
        Interaction, InteractionBlock, InteractionBlockRole, Meta, WithAgent, WithoutAgent,
    },
};

use super::{engine::new_postgres_pool, models::migrate_database_with_pg_pool};

#[derive(Debug, Clone)]
pub struct MemoryEngine<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    pool: PgPool,
    phantom: PhantomData<Backend>,
}

impl<Backend> MemoryEngine<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub async fn new(database_url: String) -> Self {
        let pool = new_postgres_pool(database_url).await;

        migrate_database_with_pg_pool(&pool).await;

        Self {
            pool,
            phantom: PhantomData,
        }
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
    ) -> Interaction<Backend, WithoutAgent> {
        let interaction = Interaction::<Backend>::new(user_name, constitution, memory_size);
        let res = query!(
            r#"
            INSERT INTO interactions (id, created_at, updated_at, user_name, default_long_term_memory_size, constitution, short_term_memory)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at, updated_at, user_name, default_long_term_memory_size, constitution, short_term_memory
            "#,
            interaction.id,
            interaction.created_at.naive_utc(),
            interaction.updated_at.naive_utc(),
            interaction.user_name,
            interaction.long_term_memory_size as i32,
            interaction.constitution,
            interaction.short_term_memory,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Interaction::<Backend, WithoutAgent> {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            user_name: res.user_name,
            long_term_memory_size: res.default_long_term_memory_size as usize,
            short_term_memory: res.short_term_memory,
            constitution: res.constitution,
            state: PhantomData,
            agent: None,
        }
    }

    pub async fn new_interaction_with_agent(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
        agent: &Agent<Backend>,
    ) -> Interaction<Backend, WithAgent> {
        let mut interaction = self
            .new_interaction(user_name, constitution, memory_size)
            .await;

        interaction.with_agent(agent.to_owned()) // TODO: Check if it can be optimized
    }

    pub async fn update_constitution(
        &mut self,
        _id: Uuid,
        _constitution: String,
    ) -> Interaction<Backend> {
        todo!()
    }

    pub async fn append_to_long_term_memory(
        &mut self,
        interaction_id: Uuid,
        interaction_block: &InteractionBlock,
    ) -> InteractionBlock {
        query!(
            r#"
            INSERT INTO interaction_blocks (id, created_at, updated_at, interaction_id, role, content, name)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at, updated_at, interaction_id, role, content, name
            "#,
            interaction_block.id,
            interaction_block.created_at.naive_utc(),
            interaction_block.updated_at.naive_utc(),
            interaction_id,
            interaction_block.role.as_str(),
            interaction_block.content,
            interaction_block.name,
        )
        .fetch_one(&self.pool)
        .await
        .map(|res| InteractionBlock {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            interaction_id: res.interaction_id,
            role: InteractionBlockRole::from_str(&res.role).unwrap(),
            content: res.content,
            name: res.name,
        })
        .unwrap()
    }

    pub async fn set_short_term_memory(
        &mut self,
        _interaction_id: Uuid,
        _memory: String,
    ) -> Interaction<Backend> {
        todo!()
    }

    pub async fn get_meta_with_agent(&mut self, agent: &mut Agent<Backend>) -> Meta {
        let meta_exists = query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM meta)
            "#
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .exists
        .unwrap();

        if !meta_exists {
            let default_interaction = self
                .new_interaction_with_agent(
                    agent.default_interaction.user_name.clone(),
                    agent.default_interaction.constitution.clone(),
                    agent.default_interaction.memory_size,
                    agent,
                )
                .await;

            query!(
                r#"
                INSERT INTO meta (id, created_at, updated_at, default_interaction_id)
                VALUES ($1, $2, $3, $4) RETURNING id, created_at, updated_at, default_interaction_id
                "#,
                Uuid::new_v4(),
                Utc::now().naive_utc(),
                Utc::now().naive_utc(),
                default_interaction.id,
            )
            .fetch_one(&self.pool)
            .await
            .map(|res| Meta {
                id: res.id,
                created_at: res.created_at.and_local_timezone(Utc).unwrap(),
                updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
                default_interaction_id: res.default_interaction_id.unwrap(),
            })
            .unwrap()
        } else {
            query!(
                r#"
                SELECT id, created_at, updated_at, default_interaction_id
                FROM meta
                "#
            )
            .fetch_one(&self.pool)
            .await
            .map(|res| Meta {
                id: res.id,
                created_at: res.created_at.and_local_timezone(Utc).unwrap(),
                updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
                default_interaction_id: res.default_interaction_id.unwrap(),
            })
            .unwrap()
        }
    }

    pub async fn get_meta(&mut self) -> Meta {
        query!(
            r#"
            SELECT id, created_at, updated_at, default_interaction_id
            FROM meta
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map(|res| Meta {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            default_interaction_id: res.default_interaction_id.unwrap(),
        })
        .unwrap()
    }

    pub async fn get_interaction(
        &mut self,
        id: Uuid,
    ) -> Option<Interaction<Backend, WithoutAgent>> {
        let res = query!(
            r#"
            SELECT id, created_at, updated_at, user_name, default_long_term_memory_size, constitution, short_term_memory
            FROM interactions
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap();

        res.map(|res| Interaction::<Backend, WithoutAgent> {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            user_name: res.user_name,
            short_term_memory: res.short_term_memory,
            long_term_memory_size: res.default_long_term_memory_size as usize,
            constitution: res.constitution,
            state: PhantomData,
            agent: None,
        })
    }

    pub async fn set_default_interaction(&mut self, id: Uuid) -> Meta {
        query!(
            r#"
            UPDATE meta
            SET default_interaction_id = $1
            WHERE id = $2
            RETURNING id, created_at, updated_at, default_interaction_id
            "#,
            id,
            self.get_meta().await.id,
        )
        .fetch_one(&self.pool)
        .await
        .map(|res| Meta {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            default_interaction_id: res.default_interaction_id.unwrap(),
        })
        .unwrap()
    }

    pub async fn get_or_create_default_interaction(
        &mut self,
        agent: &mut Agent<Backend>,
    ) -> Interaction<Backend, WithAgent> {
        let interaction_id = self.get_meta_with_agent(agent).await.default_interaction_id;

        self.get_interaction(interaction_id)
            .await
            .unwrap()
            .with_agent(agent.clone())
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction<Backend, WithoutAgent>> {
        todo!()
    }

    pub async fn new_agent(
        &mut self,
        name: String,
        default_user_name: String,
        default_constitution: String,
        default_memory_size: usize,
        llm_engine: Backend,
        memory_engine: MemoryEngine<Backend>,
    ) -> Agent<Backend>
    where
        Backend: AgentBackend + Sized + Default + Clone,
    {
        let new_id = Uuid::new_v4();
        let res = query!(
            r#"
            INSERT INTO agents (id, name, default_interaction_user_name, default_interaction_constitution, default_interaction_memory_size)
            VALUES ($1, $2, $3, $4, $5) RETURNING id, name, default_interaction_user_name, default_interaction_constitution, default_interaction_memory_size
            "#,
            new_id,
            name,
            default_user_name,
            default_constitution,
            default_memory_size as i32,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Agent::new(
            res.id,
            res.name,
            DefaultInteraction {
                user_name: res.default_interaction_user_name,
                constitution: res.default_interaction_constitution,
                memory_size: res.default_interaction_memory_size as usize,
            },
            llm_engine,
            memory_engine,
        )
    }

    pub async fn get_interaction_long_term_memory(
        &self,
        interaction_id: Uuid,
        limit: usize,
    ) -> Vec<InteractionBlock> {
        query!(
            r#"
            SELECT id, created_at, updated_at, name, interaction_id, role, content
            FROM interaction_blocks
            WHERE interaction_id = $1
            ORDER BY created_at ASC
            LIMIT $2
            "#,
            interaction_id,
            limit as i64,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
        .iter()
        .map(|res| InteractionBlock {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            name: res.name.clone(),
            interaction_id: res.interaction_id,
            role: InteractionBlockRole::from_str(res.role.as_str()).unwrap(),
            content: res.content.clone(),
        })
        .collect()
    }

    pub async fn flush_interaction_long_term_memory(&self, interaction_id: Uuid) {
        query!(
            r#"
            DELETE FROM interaction_blocks
            WHERE interaction_id = $1
            "#,
            interaction_id,
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
