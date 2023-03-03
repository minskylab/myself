use rbatis::rbdc::datetime::FastDateTime;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::{crud, Rbatis};
use rbdc::uuid::Uuid;
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Uuid,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,

    pub user_name: String,

    pub long_term_memory: String,
    pub short_term_memory: Option<String>,

    pub short_term_memory_size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: Uuid,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,

    pub default_interaction: Option<Uuid>,
}

crud!(Interaction {});
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
            id: Uuid::new(),
            created_at: FastDateTime::now(),
            updated_at: FastDateTime::now(),

            user_name,

            long_term_memory: long_term_memory_init,
            short_term_memory: None,
            short_term_memory_size,
        }
    }
}

impl Meta {
    pub fn new() -> Self {
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
        rb.init(SqliteDriver {}, &database_url).unwrap();

        let s = SqliteTableSync::default();

        s.sync(
            rb.acquire().await.unwrap(),
            to_value!(Interaction::new("user".to_string(), "".to_string(), 1)),
            "interaction",
        )
        .await
        .unwrap();

        s.sync(rb.acquire().await.unwrap(), to_value!(Meta::new()), "meta")
            .await
            .unwrap();

        let meta = Meta::select_all(&mut rb)
            .await
            .unwrap()
            .first()
            .map(|m| m.to_owned());

        if meta.is_none() {
            let meta = Meta::new();

            Meta::insert(&mut rb, &meta).await.unwrap();
        }

        Self { rb }
    }

    pub async fn new_with_defaults() -> Self {
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

        Interaction::insert(&mut self.rb, &interaction)
            .await
            .unwrap();

        // println!("data response: {:?}", data);

        Interaction::select_by_column(&mut self.rb, "id", interaction.id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }

    pub async fn update_constitution(&mut self, id: Uuid, constitution: String) -> Interaction {
        let interaction = Interaction::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        let interaction = Interaction {
            id: interaction.id.clone(),
            created_at: interaction.created_at,
            updated_at: FastDateTime::now(),

            user_name: interaction.user_name,

            long_term_memory: constitution,
            short_term_memory: interaction.short_term_memory,
            short_term_memory_size: interaction.short_term_memory_size,
        };

        Interaction::update_by_column(&mut self.rb, &interaction, "id")
            .await
            .unwrap();

        // println!("data response: {:?}", data);

        Interaction::select_by_column(&mut self.rb, "id", interaction.id)
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
        let interaction = Interaction::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        let memory = match interaction.short_term_memory {
            Some(last_memory) => format!("{}\n{}", last_memory, new_interaction),
            None => new_interaction,
        };

        let lines = memory.split("\n").collect::<Vec<&str>>();
        let max_lines = interaction.short_term_memory_size as usize;

        let memory = if lines.len() > max_lines {
            lines[lines.len() - max_lines..].join("\n")
        } else {
            memory
        };

        let interaction = Interaction {
            id: interaction.id.clone(),
            created_at: interaction.created_at,
            updated_at: FastDateTime::now(),

            user_name: interaction.user_name,

            long_term_memory: interaction.long_term_memory,
            short_term_memory: Some(memory),
            short_term_memory_size: interaction.short_term_memory_size,
        };

        Interaction::update_by_column(&mut self.rb, &interaction, "id")
            .await
            .unwrap();

        // println!("data response: {:?}", data);

        Interaction::select_by_column(&mut self.rb, "id", interaction.id)
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
        let interaction = Interaction::select_by_column(&mut self.rb, "id", interaction_id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        let interaction = Interaction {
            id: interaction.id.clone(),
            created_at: interaction.created_at,
            updated_at: FastDateTime::now(),

            user_name: interaction.user_name,

            long_term_memory: interaction.long_term_memory,
            short_term_memory: Some(memory),
            short_term_memory_size: interaction.short_term_memory_size,
        };

        Interaction::update_by_column(&mut self.rb, &interaction, "id")
            .await
            .unwrap();

        // println!("data response: {:?}", data);

        Interaction::select_by_column(&mut self.rb, "id", interaction.id)
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
        Interaction::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .map(|m| m.to_owned())
    }

    pub async fn set_default_interaction(&mut self, id: Uuid) -> Meta {
        let meta = Meta::select_all(&mut self.rb)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        let meta = Meta {
            id: meta.id.clone(),
            created_at: meta.created_at,
            updated_at: FastDateTime::now(),

            default_interaction: Some(id),
        };

        Meta::update_by_column(&mut self.rb, &meta, "id")
            .await
            .unwrap();

        // println!("data response: {:?}", data);

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
                Interaction::select_by_column(&mut self.rb, "id", id)
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
        let interaction = match self.get_default_interaction().await {
            Some(interaction) => interaction,
            None => {
                let new_default = self
                    .new_interaction(user_name, constitution, memory_size)
                    .await;

                self.set_default_interaction(new_default.id.clone()).await;

                new_default
            }
        };

        interaction
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction> {
        Interaction::select_all(&mut self.rb)
            .await
            .unwrap()
            .iter()
            .map(|i| i.to_owned())
            .collect()
    }
}
