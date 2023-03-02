use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::{crud, Rbatis};
// use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::rbdc::datetime::FastDateTime;
use rbdc::uuid::Uuid;
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;
// use rbdc_sqlite::driver::SqliteDriver;
// use rbs::to_value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: Uuid,
    pub created_at: FastDateTime,
    pub updated_at: FastDateTime,

    pub username: String,

    pub template_memory: String,
    pub dynamic_memory: Option<String>,

    pub dynamic_memory_size: i32,
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

impl MemoryEngine {
    pub async fn new() -> Self {
        let mut rb = Rbatis::new();
        rb.init(SqliteDriver {}, "sqlite://target/sqlite.db")
            .unwrap();

        let s = SqliteTableSync::default();
        s.sync(
            rb.acquire().await.unwrap(),
            to_value!(Interaction {
                id: Uuid::new(),
                created_at: FastDateTime::now(),
                updated_at: FastDateTime::now(),

                username: "".into(),

                template_memory: "".into(),
                dynamic_memory: None,
                dynamic_memory_size: 0,
            }),
            "interaction",
        )
        .await
        .unwrap();

        s.sync(
            rb.acquire().await.unwrap(),
            to_value!(Meta {
                id: Uuid::new(),
                created_at: FastDateTime::now(),
                updated_at: FastDateTime::now(),

                default_interaction: None,
            }),
            "meta",
        )
        .await
        .unwrap();

        let meta = Meta::select_all(&mut rb)
            .await
            .unwrap()
            .first()
            .map(|m| m.to_owned());

        if meta.is_none() {
            let meta = Meta {
                id: Uuid::new(),
                created_at: FastDateTime::now(),
                updated_at: FastDateTime::now(),

                default_interaction: None,
            };

            Meta::insert(&mut rb, &meta).await.unwrap();
        }

        Self { rb }
    }

    pub async fn new_interaction(&mut self, username: String, constitution: String) -> Interaction {
        let interaction = Interaction {
            id: Uuid::new(),
            created_at: FastDateTime::now(),
            updated_at: FastDateTime::now(),

            username,

            template_memory: constitution,
            dynamic_memory: None,
            dynamic_memory_size: 10,
        };

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

            username: interaction.username,

            template_memory: constitution,
            dynamic_memory: interaction.dynamic_memory,
            dynamic_memory_size: interaction.dynamic_memory_size,
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

        let memory = match interaction.dynamic_memory {
            Some(last_memory) => format!("{}\n{}", last_memory, new_interaction),
            None => new_interaction,
        };

        let lines = memory.split("\n").collect::<Vec<&str>>();
        let max_lines = interaction.dynamic_memory_size as usize;

        let memory = if lines.len() > max_lines {
            lines[lines.len() - max_lines..].join("\n")
        } else {
            memory
        };

        let interaction = Interaction {
            id: interaction.id.clone(),
            created_at: interaction.created_at,
            updated_at: FastDateTime::now(),

            username: interaction.username,

            template_memory: interaction.template_memory,
            dynamic_memory: Some(memory),
            dynamic_memory_size: interaction.dynamic_memory_size,
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

    pub async fn get_interaction(&mut self, id: Uuid) -> Interaction {
        Interaction::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
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
                    .unwrap()
                    .to_owned(),
            ),
            None => None,
        }
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
