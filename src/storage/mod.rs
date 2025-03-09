use rusqlite::{Connection, Result};

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let storage = Storage { conn };
        storage.init_tables()?;
        Ok(storage)
    }

    pub fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "
        CREATE TABLE IF NOT EXISTS rate_limits (
            user_id TEXT PRIMARY KEY,
            request_limit INTEGER,
            last_updated INTEGER
        );

        CREATE TABLE IF NOT EXISTS job_queue (
            job_id TEXT PRIMARY KEY,
            payload TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        ",
        )?;
        Ok(())
    }

    /// Inserts or updates a rate limit for a user.
    pub fn set_rate_limit(&self, user_id: &str, limit: u64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO rate_limits (user_id, request_limit, last_updated)
         VALUES (?1, ?2, strftime('%s', 'now'))
         ON CONFLICT(user_id) DO UPDATE 
         SET request_limit = excluded.request_limit, last_updated = excluded.last_updated;",
            (user_id, limit),
        )?;
        Ok(())
    }

    /// Retrieves the rate limit for a user.
    pub fn get_rate_limit(&self, user_id: &str) -> Result<Option<u64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT request_limit FROM rate_limits WHERE user_id = ?1")?;
        let mut rows = stmt.query([user_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    /// Pushes a job into the queue.
    pub fn push_job(&self, job_id: &str, payload: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO job_queue (job_id, payload, created_at)
                 VALUES (?1, ?2, strftime('%s', 'now'))",
            (job_id, payload),
        )?;
        Ok(())
    }

    /// Fetches and removes the next job from the queue.
    pub fn pop_job(&self) -> Result<Option<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT job_id, payload FROM job_queue ORDER BY created_at LIMIT 1")?;
        let mut rows = stmt.query([])?;

        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?)))
        } else {
            Ok(None)
        }
    }
}
