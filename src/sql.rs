use rusqlite::Connection;
use sha2::{Digest, Sha256};

fn hash_str(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str::trim(str).as_bytes());
    let hash = hasher.finalize();

    // convert hash to String
    let mut hash_str = String::new();
    for byte in hash.iter() {
        hash_str.push_str(&format!("{:02x}", byte));
    }
    hash_str
}

fn set_key(connection: &Connection) -> Result<(), rusqlite::Error> {
    let mut set_key = connection.prepare("PRAGMA KEY = 'o6zVMlM7kmsKmt4uphuF4ypKz7Yzcmxq'")?;
    let _ = set_key.query(())?;
    let mut test_key = connection.prepare("SELECT count(*) FROM sqlite_master;")?;
    let _ = test_key.query(())?;
    Ok(())
}

fn establish_connection() -> Result<Connection, rusqlite::Error> {
    let connection = Connection::open("database.db")?;
    set_key(&connection)?;
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
    let hash = hash_str(password);
    let connection = establish_connection()?;
    connection.execute(
        "INSERT INTO credentials (user, password_hash) VALUES (?1, ?2)",
        (user, hash),
    )?;
    Ok(())
}

pub(crate) fn check_user(user: &str, password: &str) -> Result<bool, rusqlite::Error> {
    let connection = establish_connection()?;
    let stored_hash: String = connection.query_row(
        "SELECT password_hash FROM credentials WHERE user = ?1",
        [user],
        |row| row.get(0),
    )?;

    Ok(stored_hash == hash_str(password))
}
