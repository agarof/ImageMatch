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
            users::create(email1, pass2, &mut trans).await.unwrap_err(),
            Error::DuplicateEmail
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
            users::confirm(Id(598023940), &mut trans).await.unwrap_err(),
            Error::InvalidUserId
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
            USERS
                .into_iter()
                .find(|&(email, _)| email == user.email)
                .expect("Did not find user in list");
        }
    }
}

mod find_by_credentials {
    use crate::common::{connect_db, data::*};

    use db::{result::Error, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn valid() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        let expected = users::create(email, pass, &mut trans).await.unwrap();
        users::confirm(expected, &mut trans).await.unwrap();
        let (id, admin) = users::find_by_credentials(email, pass, &mut trans)
            .await
            .unwrap();

        assert_eq!(id, expected);
        assert!(!admin);
    }

    #[tokio::test]
    async fn admin() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        let expected = users::create(email, pass, &mut trans).await.unwrap();
        users::confirm(expected, &mut trans).await.unwrap();
        users::set_admin(expected, true, &mut trans).await.unwrap();
        let (id, admin) = users::find_by_credentials(email, pass, &mut trans)
            .await
            .unwrap();

        assert_eq!(id, expected);
        assert!(admin);
    }

    #[tokio::test]
    async fn invalid_password() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        let id = users::create(email, pass, &mut trans).await.unwrap();
        users::confirm(id, &mut trans).await.unwrap();
        let e = users::find_by_credentials(email, USERS[1].1, &mut trans)
            .await
            .unwrap_err();

        assert!(matches!(e, Error::InvalidCredentials))
    }

    #[tokio::test]
    async fn invalid_email() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        let id = users::create(email, pass, &mut trans).await.unwrap();
        users::confirm(id, &mut trans).await.unwrap();
        let e = users::find_by_credentials(USERS[1].0, pass, &mut trans)
            .await
            .unwrap_err();

        assert!(matches!(e, Error::InvalidCredentials));
    }
}

mod set_admin {
    use crate::common::{connect_db, data::*};

    use db::{result::Error, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn valid_id() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];

        let id = users::create(email, pass, &mut trans).await.unwrap();
        users::set_admin(id, true, &mut trans).await.unwrap();
    }

    #[tokio::test]
    async fn invalid_id() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let error = users::set_admin(users::Id(94886529), true, &mut trans)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidUserId,));
    }
}
