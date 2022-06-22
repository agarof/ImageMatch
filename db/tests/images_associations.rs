mod common;

mod create {
    use crate::common::{connect_db, data::*};

    use db::{images, images_associations, result::Error, sessions};
    use sqlx::Acquire;

    #[tokio::test]
    async fn one() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        images_associations::create(image, session, &mut trans)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn multiple_images() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image_1 = images::create(&mut trans).await.unwrap();
        let image_2 = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        images_associations::create(image_1, session, &mut trans)
            .await
            .unwrap();
        images_associations::create(image_2, session, &mut trans)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn multiple_sessions() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session_1 = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
        let session_2 = sessions::create("Session", DATES[1](), DATES[2](), DATES[3](), &mut trans)
            .await
            .unwrap();

        images_associations::create(image, session_1, &mut trans)
            .await
            .unwrap();
        images_associations::create(image, session_2, &mut trans)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn same_combination() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        images_associations::create(image, session, &mut trans)
            .await
            .unwrap();
        images_associations::create(image, session, &mut trans)
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn invalid_image() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        assert!(matches!(
            images_associations::create(images::Id(100000), session, &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidImage
        ));
    }

    #[tokio::test]
    async fn invalid_session() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        assert!(matches!(
            images_associations::create(image, sessions::Id(1000000), &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidSession
        ));
    }
}

mod delete {
    use crate::common::{connect_db, data::*};

    use db::{images, images_associations, result::Error, sessions};
    use sqlx::Acquire;

    #[tokio::test]
    async fn once() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
        let id = images_associations::create(image, session, &mut trans)
            .await
            .unwrap();

        images_associations::delete(id, &mut trans).await.unwrap();
    }

    #[tokio::test]
    async fn twice() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
        let id = images_associations::create(image, session, &mut trans)
            .await
            .unwrap();

        images_associations::delete(id, &mut trans).await.unwrap();
        assert!(matches!(
            images_associations::delete(id, &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidImageAssociation
        ));
    }
}

mod delete_by_session {
    use crate::common::{connect_db, data::*};

    use db::{images, images_associations, result::Error, sessions};
    use sqlx::Acquire;

    #[tokio::test]
    async fn once() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
        images_associations::create(image, session, &mut trans)
            .await
            .unwrap();

        images_associations::delete_by_session(image, session, &mut trans)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn twice() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();
        images_associations::create(image, session, &mut trans)
            .await
            .unwrap();

        images_associations::delete_by_session(image, session, &mut trans)
            .await
            .unwrap();
        assert!(matches!(
            images_associations::delete_by_session(image, session, &mut trans)
                .await
                .unwrap_err(),
            Error::InvalidImageAssociation
        ));
    }
}

mod by_session {
    use crate::common::{connect_db, data::*};

    use db::{images, images_associations, sessions};
    use sqlx::Acquire;

    #[tokio::test]
    async fn basic() {
        let mut db = connect_db().await;
        let mut trans = db.begin().await.unwrap();

        let image_1 = images::create(&mut trans).await.unwrap();
        let image_2 = images::create(&mut trans).await.unwrap();
        let session = sessions::create("Session", DATES[0](), DATES[1](), DATES[2](), &mut trans)
            .await
            .unwrap();

        images_associations::create(image_1, session, &mut trans)
            .await
            .unwrap();
        images_associations::create(image_2, session, &mut trans)
            .await
            .unwrap();

        let images = images_associations::by_session(session, &mut trans)
            .await
            .unwrap();

        assert_eq!(images.len(), 2);
        assert!(images.iter().any(|&i| i == image_1));
        assert!(images.iter().any(|&i| i == image_2));
    }
}
