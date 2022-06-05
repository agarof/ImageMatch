mod common;

mod create {
    use crate::common::{connect_db, data::*};

    use db::{credentials, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn basic() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();

        credentials::create(id, &mut trans).await.unwrap();
    }
}

mod auth {
    use crate::common::{connect_db, data::USERS};

    use db::{credentials, result::Error, users};
    use sqlx::{types::Uuid, Acquire};

    #[tokio::test]
    async fn exists() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let uuid = credentials::create(id, &mut trans).await.unwrap();
        let auth = credentials::auth(uuid, &mut trans).await.unwrap();

        assert_eq!(id, auth)
    }

    #[tokio::test]
    async fn doesnt_exist() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let uuid = Uuid::from_u128(9231856239808572938);
        let error = credentials::auth(uuid, &mut trans).await.unwrap_err();

        match error {
            Error::InvalidToken => (),
            _ => panic!(),
        }
    }
}

mod delete {
    use crate::common::{connect_db, data::USERS};

    use db::{credentials, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn single() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let uuid = credentials::create(id, &mut trans).await.unwrap();

        assert_eq!(credentials::auth(uuid, &mut trans).await.unwrap(), id);
        credentials::delete(uuid, &mut trans).await.unwrap();
        credentials::auth(uuid, &mut trans).await.unwrap_err();
    }
}

mod logout_user {
    use crate::common::{connect_db, data::USERS};

    use db::{credentials, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn multiple() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let uuid_1 = credentials::create(id, &mut trans).await.unwrap();
        let uuid_2 = credentials::create(id, &mut trans).await.unwrap();

        assert_eq!(credentials::auth(uuid_1, &mut trans).await.unwrap(), id);
        assert_eq!(credentials::auth(uuid_2, &mut trans).await.unwrap(), id);

        credentials::logout_user(id, &mut trans).await.unwrap();
        credentials::auth(uuid_1, &mut trans).await.unwrap_err();
        credentials::auth(uuid_2, &mut trans).await.unwrap_err();
    }
}
