use sqlx::PgPool;

pub type PgConn = PgPool; // KHÔNG dùng Arc nữa nếu bạn không dùng clone
