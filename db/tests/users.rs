mod common;

mod create {
    use crate::common::{connect_db, data::*};

    use db::{result::Error, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn one() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        users::create(email, pass, &mut trans).await.unwrap();
    }

    #[tokio::test]
    async fn multiple() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        for (email, password) in USERS {
            users::create(email, password, &mut trans).await.unwrap();
        }
    }

    #[tokio::test]
    async fn multiple_with_same_email() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let [(email1, pass1), (_, pass2), ..] = USERS;

        users::create(email1, pass1, &mut trans).await.unwrap();
        assert!(matches!(
            users::create(email1, pass2, &mut trans).await,
            Err(Error::DuplicateEmail)
        ));
    }
}

mod confirm {
    use crate::common::{connect_db, data::*};

    use db::{
        result::Error,
        users::{self, Id},
    };
    use sqlx::Acquire;

    #[tokio::test]
    async fn exists() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        let id = users::create(email, pass, &mut trans).await.unwrap();

        users::confirm(id, &mut trans).await.unwrap();
    }

    #[tokio::test]
    async fn doesnt_exist() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        assert!(matches!(
            users::confirm(Id(598023940), &mut trans).await,
            Err(Error::InvalidUserId)
        ));
    }
}

mod list_candidates {
    use crate::common::{connect_db, data::*};

    use db::users;
    use sqlx::Acquire;

    #[tokio::test]
    async fn basic() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        for (email, password) in USERS {
            users::create(email, password, &mut trans).await.unwrap();
        }

        let list = users::list_candidates(&mut trans).await.unwrap();

        assert_eq!(list.len(), USERS.len());

        for user in list {
            let (_, password) = USERS
                .into_iter()
                .find(|&(email, _)| email == user.email)
                .expect("Did not find user in list");

            assert_eq!(user.password, password);
        }
    }
}
