mod common;

mod create {
    use crate::common::{connect_db, data::*};

    use db::{registrations, sessions, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn valid() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let user = users::create(USERS[0].0, USERS[0].1, &mut trans)
            .await
            .unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        registrations::create(user, session, &mut trans)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn invalid_user() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        registrations::create(users::Id(48917312), session, &mut trans)
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn invalid_session() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let user = users::create(USERS[0].0, USERS[0].1, &mut trans)
            .await
            .unwrap();

        registrations::create(user, sessions::Id(1293867), &mut trans)
            .await
            .unwrap_err();
    }
}

mod by_user {
    use crate::common::{connect_db, data::*};

    use db::{registrations, sessions, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn valid() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let user = users::create(USERS[0].0, USERS[0].1, &mut trans)
            .await
            .unwrap();
        let session_1 = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
        sessions::create("Session", DATES[1](), DATES[2](), DATES[3](), &mut trans)
            .await
            .unwrap();
        let session_3 = sessions::create("Session", DATES[2](), DATES[3](), DATES[4](), &mut trans)
            .await
            .unwrap();

        registrations::create(user, session_1, &mut trans)
            .await
            .unwrap();
        registrations::create(user, session_3, &mut trans)
            .await
            .unwrap();

        let sessions = registrations::by_user(user, &mut trans).await.unwrap();

        assert!(sessions.iter().any(|session| session.id == session_1
            && session.phase1 == DATES[0]()
            && session.phase2 == DATES[1]()
            && session.phase3 == DATES[2]()));
        assert!(sessions.iter().any(|session| session.id == session_3
            && session.phase1 == DATES[2]()
            && session.phase2 == DATES[3]()
            && session.phase3 == DATES[4]()));
    }
}

mod by_session {
    use crate::common::{connect_db, data::*};

    use db::{registrations, sessions, users};
    use sqlx::Acquire;

    #[tokio::test]
    async fn valid() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let user_1 = users::create(USERS[0].0, USERS[0].1, &mut trans)
            .await
            .unwrap();
        users::create(USERS[1].0, USERS[1].1, &mut trans)
            .await
            .unwrap();
        let user_3 = users::create(USERS[2].0, USERS[2].1, &mut trans)
            .await
            .unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        registrations::create(user_1, session, &mut trans)
            .await
            .unwrap();
        registrations::create(user_3, session, &mut trans)
            .await
            .unwrap();

        let users = registrations::by_session(session, &mut trans)
            .await
            .unwrap();

        assert!(users
            .iter()
            .any(|user| user.id == user_1 && user.email == USERS[0].0));
        assert!(users
            .iter()
            .any(|user| user.id == user_3 && user.email == USERS[2].0));
    }
}
