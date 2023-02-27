use myself::database::DatabaseCore;

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
