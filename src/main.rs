#[macro_use]
extern crate rbatis;
extern crate rbdc;
use myself::database::DatabaseCore;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbatis::{rbdc::datetime::FastDateTime, Rbatis};
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
    pub vector: Vec<String>,
}
crud!(BizActivity {});

#[tokio::main]
async fn main() {
    // let mut rb = Rbatis::new();
    // // sqlite
    // rb.init(SqliteDriver {}, "sqlite://target/sqlite.db")
    //     .unwrap();

    // println!("Hello, world!");

    // let activity = BizActivity {
    //     id: Some("2".into()),
    //     name: Some("2".into()),
    //     pc_link: Some("2".into()),
    //     h5_link: Some("2".into()),
    //     pc_banner_img: None,
    //     h5_banner_img: None,
    //     sort: None,
    //     status: Some(2),
    //     remark: Some("2".into()),
    //     create_time: Some(FastDateTime::now()),
    //     version: Some(1),
    //     delete_flag: Some(1),
    //     vector: vec!["1".into(), "2".into()],
    // };

    // let s = SqliteTableSync::default();
    // s.sync(
    //     rb.acquire().await.unwrap(),
    //     to_value!(BizActivity {
    //         id: None,
    //         name: None,
    //         pc_link: None,
    //         h5_link: None,
    //         pc_banner_img: None,
    //         h5_banner_img: None,
    //         sort: None,
    //         status: None,
    //         remark: None,
    //         create_time: None,
    //         version: None,
    //         delete_flag: None,
    //         vector: vec![],
    //     }),
    //     "biz_activity",
    // )
    // .await
    // .unwrap();

    // let res = BizActivity::select_all(&mut rb).await.unwrap();

    // let _filtered_res = BizActivity::select_by_column(&mut rb, "status", 2)
    //     .await
    //     .unwrap();

    // println!("{:?}", res);
    // println!("{:?}", res);

    // BizActivity::insert(&mut rb, &activity).await.unwrap();

    let mut database_core = DatabaseCore::new().await;

    let interaction = database_core.new_interaction("bregy_1".to_string()).await;

    println!("{:?}", interaction);

    // database_core.new_interaction("bregy_2".to_string()).await;
}
