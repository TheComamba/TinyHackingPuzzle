use rusqlite::Connection;

fn establish_connection() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("database.db")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS credentials (
        user TEXT NOT NULL PRIMARY KEY,
        password_hash TEXT NOT NULL
    )",
        (),
    )?;

    Ok(connection)
}

pub(crate) fn add_user(user: &str, password: &str) -> Result<(), rusqlite::Error> {
    let connection = establish_connection()?;
    connection.execute(
        "INSERT INTO credentials (user, password_hash) VALUES (?1, ?2)",
        (user, password),
    )?;
    Ok(())
}

pub(crate) fn check_user(user: &str, password: &str) -> Result<bool, rusqlite::Error> {
    let connection = establish_connection()?;
    let hash: String = connection.query_row(
        "SELECT password_hash FROM credentials WHERE user = ?1",
        [user],
        |row| row.get(0),
    )?;

    Ok(hash == password)
}
