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
    id: Uuid,
    created_at: FastDateTime,
    updated_at: FastDateTime,

    username: String,

    template_memory: Option<String>,
    dynamic_memory: Option<String>,
}

crud!(Interaction {});

pub struct DatabaseCore {
    rb: Rbatis,
}

impl DatabaseCore {
    pub async fn new() -> Self {
        let rb = Rbatis::new();
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

                template_memory: None,
                dynamic_memory: None,
            }),
            "interaction",
        )
        .await
        .unwrap();

        Self { rb }
    }

    pub async fn new_interaction(&mut self, username: String) -> Interaction {
        let interaction = Interaction {
            id: Uuid::new(),
            created_at: FastDateTime::now(),
            updated_at: FastDateTime::now(),

            username,

            template_memory: None,
            dynamic_memory: None,
        };

        let id = Interaction::insert(&mut self.rb, &interaction)
            .await
            .unwrap()
            .last_insert_id
            .as_i64()
            .unwrap();

        println!("id: {}", id);

        Interaction::select_by_column(&mut self.rb, "id", id)
            .await
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }
}
