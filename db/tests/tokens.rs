mod common;

mod create {
    use crate::common::{connect_db, data::*};

    use db::{tokens, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn basic() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();

        tokens::create(id, &mut trans).await.unwrap();
    }
}

mod auth {
    use crate::common::{connect_db, data::USERS};

    use db::{
        result::Error,
        tokens::{self, Token},
        users,
    };
    use sqlx::{types::Uuid, Acquire};

    #[tokio::test]
    async fn exists() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let (token, _) = tokens::create(id, &mut trans).await.unwrap();
        let auth = tokens::auth(token, &mut trans).await.unwrap();

        assert_eq!(id, auth)
    }

    #[tokio::test]
    async fn doesnt_exist() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let uuid = Uuid::from_u128(9231856239808572938);
        let error = tokens::auth(Token(uuid), &mut trans).await.unwrap_err();

        assert!(matches!(error, Error::InvalidToken))
    }
}

mod auth_admin {
    use crate::common::{connect_db, data::USERS};

    use db::{
        result::Error,
        tokens::{self, Token},
        users,
    };
    use sqlx::{types::Uuid, Acquire};

    #[tokio::test]
    async fn admin_user() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let (token, _) = tokens::create(id, &mut trans).await.unwrap();

        users::set_admin(id, true, &mut trans).await.unwrap();

        let auth = tokens::auth_admin(token, &mut trans).await.unwrap();

        assert_eq!(id, auth);
    }

    #[tokio::test]
    async fn normal_user() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let (token, _) = tokens::create(id, &mut trans).await.unwrap();

        let error = tokens::auth_admin(token, &mut trans).await.unwrap_err();

        assert!(matches!(error, Error::InvalidToken));
    }

    #[tokio::test]
    async fn invalid_token() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let uuid = Uuid::from_u128(91823561239);
        let error = tokens::auth_admin(Token(uuid), &mut trans)
            .await
            .unwrap_err();

        assert!(matches!(error, Error::InvalidToken));
    }
}

mod delete {
    use crate::common::{connect_db, data::USERS};

    use db::{result::Error, tokens, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn once() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let (token, _) = tokens::create(id, &mut trans).await.unwrap();

        assert_eq!(tokens::auth(token, &mut trans).await.unwrap(), id);
        tokens::delete(token, &mut trans).await.unwrap();
        tokens::auth(token, &mut trans).await.unwrap_err();
    }

    #[tokio::test]
    async fn twice() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let (token, _) = tokens::create(id, &mut trans).await.unwrap();

        assert_eq!(tokens::auth(token, &mut trans).await.unwrap(), id);
        tokens::delete(token, &mut trans).await.unwrap();
        assert!(matches!(
            tokens::delete(token, &mut trans).await.unwrap_err(),
            Error::InvalidToken
        ));
        tokens::auth(token, &mut trans).await.unwrap_err();
    }
}

mod logout_user {
    use crate::common::{connect_db, data::USERS};

    use db::{tokens, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn multiple() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();
        let (email, pass) = USERS[0];
        let id = users::create(email, pass, &mut trans).await.unwrap();
        let (token_1, _) = tokens::create(id, &mut trans).await.unwrap();
        let (token_2, _) = tokens::create(id, &mut trans).await.unwrap();

        assert_eq!(tokens::auth(token_1, &mut trans).await.unwrap(), id);
        assert_eq!(tokens::auth(token_2, &mut trans).await.unwrap(), id);

        tokens::logout_user(id, &mut trans).await.unwrap();
        tokens::auth(token_1, &mut trans).await.unwrap_err();
        tokens::auth(token_2, &mut trans).await.unwrap_err();
    }
}
