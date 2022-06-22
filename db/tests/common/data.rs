use sqlx::types::time::OffsetDateTime;

#[allow(dead_code)]
pub const USERS: [(&str, &str); 3] = [
    ("toto.tata@tutu.com", "Entreeeeeee"),
    ("titi.tyty@tutu.com", "SaaS"),
    ("track.mania@nadeo.com", "Cactus"),
];

// Dates in ascending order, a month apart in the year 5432
#[allow(dead_code)]
pub const DATES: [fn() -> OffsetDateTime; 6] = [
    || OffsetDateTime::from_unix_timestamp(109250809446).unwrap(),
    || OffsetDateTime::from_unix_timestamp(109253487846).unwrap(),
    || OffsetDateTime::from_unix_timestamp(109255993446).unwrap(),
    || OffsetDateTime::from_unix_timestamp(109258668246).unwrap(),
    || OffsetDateTime::from_unix_timestamp(109261260246).unwrap(),
    || OffsetDateTime::from_unix_timestamp(109263938646).unwrap(),
];
