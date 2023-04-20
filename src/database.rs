use std::marker::PhantomData;

use rbatis::rbdc::datetime::FastDateTime;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::{crud, Rbatis};
use rbdc::uuid::Uuid;
// use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;
use serde::{Deserialize, Serialize};

use crate::agent::Agent;

pub trait InteractionState {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithAgent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithoutAgent;

impl InteractionState for WithAgent {}
impl InteractionState for WithoutAgent {}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Interaction<State = WithoutAgent>
where
    State: InteractionState,
{
    pub id: Uuid,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,

    pub user_name: String,

    pub long_term_memory: String,
    pub short_term_memory: Option<String>,

    pub short_term_memory_size: usize,

    pub state: PhantomData<State>,

    #[serde(skip_serializing, skip_deserializing)]
    agent: Option<Box<Agent>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: Uuid,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,

    pub default_interaction: Option<Uuid>,
}

crud!(Interaction {});
crud!(Interaction<WithAgent> {});
crud!(Meta {});

#[derive(Debug, Clone)]
pub struct MemoryEngine {
    rb: Rbatis,
}

impl Interaction {
    pub fn new(
        user_name: String,
        long_term_memory_init: String,
        short_term_memory_size: usize,
    ) -> Self {
        Self {
            user_name,
            long_term_memory: long_term_memory_init,
            short_term_memory_size,
            ..Default::default()
        }
    }

    pub fn new_with_agent(
        user_name: String,
        long_term_memory_init: String,
        short_term_memory_size: usize,
        agent: Agent,
    ) -> Interaction<WithAgent> {
        Interaction::<WithAgent> {
            user_name,
            long_term_memory: long_term_memory_init,
            short_term_memory_size,
            agent: Some(Box::new(agent)),
            state: PhantomData,
            ..Default::default()
        }
    }

    pub fn set_agent(&mut self, agent: Agent) -> Interaction<WithAgent> {
        self.agent = Some(Box::new(agent));
        Interaction::<WithAgent> {
            id: self.id.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_name: self.user_name.clone(),
            long_term_memory: self.long_term_memory.clone(),
            short_term_memory: self.short_term_memory.clone(),
            short_term_memory_size: self.short_term_memory_size,
            agent: self.agent.clone(),
            state: PhantomData,
        }
    }
}
impl Interaction<WithAgent> {
    pub async fn interact(&mut self, message: &String) -> Option<String> {
        self.agent
            .clone()
            .unwrap()
            .interact(self.id.clone(), message)
            .await
    }
}

impl<State> Default for Interaction<State>
where
    State: InteractionState,
{
    fn default() -> Self {
        Self {
            id: Uuid::new(),
            created_at: FastDateTime::now(),
            updated_at: FastDateTime::now(),

            user_name: "".to_string(),

            long_term_memory: "".to_string(),
            short_term_memory: None,
            short_term_memory_size: 0,
            agent: None,
            state: PhantomData,
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            id: Uuid::new(),
            created_at: FastDateTime::now(),
            updated_at: FastDateTime::now(),

            default_interaction: None,
        }
    }
}

impl MemoryEngine {
    pub async fn new(database_url: String) -> Self {
        let mut rb = Rbatis::new();

        if database_url.starts_with("postgresql://") || database_url.starts_with("postgres://") {
            todo!("PostgreSQL is not supported yet. Sorry :(")
            // rb.init(PgDriver {}, &database_url).unwrap();
        } else {
            rb.init(SqliteDriver {}, &database_url).unwrap();
            let s = SqliteTableSync::default();

            s.sync(
                rb.acquire().await.unwrap(),
                to_value!(Interaction::new("user".to_string(), "".to_string(), 1)),
                "interaction",
            )
            .await
            .unwrap();

            s.sync(
                rb.acquire().await.unwrap(),
                to_value!(Meta::default()),
                "meta",
            )
            .await
            .unwrap();
        }

        let meta = Meta::select_all(&mut rb)
            .await
            .unwrap()
            .first()
            .map(|m| m.to_owned());

        if meta.is_none() {
            let meta = Meta::default();

            Meta::insert(&mut rb, &meta).await.unwrap();
        }

        Self { rb }
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

        Interaction::<WithoutAgent>::insert(&mut self.rb, &interaction)
            .await
            .unwrap();

        // println!("data response: {:?}", data);

        Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", interaction.id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }

    pub async fn new_interaction_with_agent(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
        agent: &Agent,
    ) -> Interaction<WithAgent> {
        let interaction = Interaction::new(user_name, constitution, memory_size);

        let data = Interaction::<WithoutAgent>::insert(&mut self.rb, &interaction)
            .await
            .unwrap();

        println!("data response: {:?}", data);
        println!("interaction id response: {:?}", interaction.id);

        Interaction::<WithoutAgent>::select_by_column(
            &mut self.rb,
            "id",
            interaction.id.to_string(),
        )
        .await
        .unwrap()
        .first()
        .unwrap()
        .to_owned()
        .set_agent(agent.to_owned())
    }

    pub async fn update_constitution(&mut self, id: Uuid, constitution: String) -> Interaction {
        let mut interaction = Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        interaction.updated_at = FastDateTime::now();
        interaction.long_term_memory = constitution;

        Interaction::<WithoutAgent>::update_by_column(&mut self.rb, &interaction, "id")
            .await
            .unwrap();

        Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", interaction.id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }

    pub async fn append_to_dynamic_memory(
        &mut self,
        id: Uuid,
        new_interaction: String,
    ) -> Interaction {
        let mut interaction = Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        let memory = match interaction.short_term_memory {
            Some(last_memory) => format!("{}\n{}", last_memory, new_interaction),
            None => new_interaction,
        };

        let lines = memory.split('\n').collect::<Vec<&str>>();
        let max_lines = interaction.short_term_memory_size;

        let memory = if lines.len() > max_lines {
            lines[lines.len() - max_lines..].join("\n")
        } else {
            memory
        };

        interaction.updated_at = FastDateTime::now();
        interaction.short_term_memory = Some(memory);

        Interaction::<WithoutAgent>::update_by_column(&mut self.rb, &interaction, "id")
            .await
            .unwrap();

        Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", interaction.id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }

    pub async fn set_short_term_memory(
        &mut self,
        interaction_id: Uuid,
        memory: String,
    ) -> Interaction {
        let mut interaction =
            Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", interaction_id)
                .await
                .unwrap()
                .first()
                .unwrap()
                .to_owned();

        interaction.updated_at = FastDateTime::now();
        interaction.short_term_memory = Some(memory);

        Interaction::<WithoutAgent>::update_by_column(&mut self.rb, &interaction, "id")
            .await
            .unwrap();

        Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", interaction.id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }

    pub async fn get_meta(&mut self) -> Option<Meta> {
        Meta::select_all(&mut self.rb)
            .await
            .unwrap()
            .first()
            .map(|m| m.to_owned())
    }

    pub async fn get_interaction(&mut self, id: Uuid) -> Option<Interaction> {
        Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .map(|m| m.to_owned())
    }

    pub async fn set_default_interaction(&mut self, id: Uuid) -> Meta {
        let mut meta = Meta::select_all(&mut self.rb)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        meta.updated_at = FastDateTime::now();
        meta.default_interaction = Some(id);

        Meta::update_by_column(&mut self.rb, &meta, "id")
            .await
            .unwrap();

        Meta::select_by_column(&mut self.rb, "id", meta.id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }

    pub async fn get_default_interaction(&mut self) -> Option<Interaction> {
        let meta = Meta::select_all(&mut self.rb)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        match meta.default_interaction {
            Some(id) => Some(
                Interaction::<WithoutAgent>::select_by_column(&mut self.rb, "id", id)
                    .await
                    .unwrap()
                    .first()
                    // TODO: handle this better
                    .unwrap()
                    .to_owned(),
            ),
            None => None,
        }
    }

    pub async fn get_or_create_default_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        match self.get_default_interaction().await {
            Some(interaction) => interaction,
            None => {
                let new_default = self
                    .new_interaction(user_name, constitution, memory_size)
                    .await;

                self.set_default_interaction(new_default.id.clone()).await;

                new_default
            }
        }
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction> {
        Interaction::<WithoutAgent>::select_all(&mut self.rb)
            .await
            .unwrap()
            .iter()
            .map(|i| i.to_owned())
            .collect()
    }
}
