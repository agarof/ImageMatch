mod common;

mod create {
    use crate::common::connect_db;

    use db::images;
    use sqlx::Acquire;

    #[tokio::test]
    async fn one() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        images::create(&mut trans).await.unwrap();
    }
}
