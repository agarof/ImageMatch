mod common;

mod create {
    use crate::common::{connect_db, data::*};

    use db::{result::Error, sessions};
    use sqlx::Acquire;

    #[tokio::test]
    async fn valid() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn one_after_two() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        assert!(matches!(
            sessions::create("Session", DATES[1](), DATES[0](), DATES[2](), &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidDates
        ));
    }

    #[tokio::test]
    async fn one_after_three() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        assert!(matches!(
            sessions::create("Session", DATES[1](), DATES[2](), DATES[0](), &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidDates
        ));
    }

    #[tokio::test]
    async fn two_after_three() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        assert!(matches!(
            sessions::create("Session", DATES[0](), DATES[2](), DATES[1](), &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidDates
        ));
    }
}
