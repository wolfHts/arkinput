use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::{AppStats, DailyStats, InputRecord, SearchFilter};

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS inputs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME NOT NULL,
                app_name TEXT NOT NULL,
                window_title TEXT,
                content TEXT NOT NULL,
                key_count INTEGER DEFAULT 1,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_timestamp ON inputs(timestamp);
            CREATE INDEX IF NOT EXISTS idx_app_name ON inputs(app_name);

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn insert_record(&self, record: &InputRecord) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO inputs (timestamp, app_name, window_title, content, key_count) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                record.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                record.app_name,
                record.window_title,
                record.content,
                record.key_count,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_records(&self, filter: &SearchFilter) -> Result<Vec<InputRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::from(
            "SELECT id, timestamp, app_name, window_title, content, key_count, created_at FROM inputs WHERE 1=1",
        );
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref query) = filter.query {
            sql.push_str(" AND content LIKE ?");
            params_vec.push(Box::new(format!("%{}%", query)));
        }

        if let Some(ref app_name) = filter.app_name {
            sql.push_str(" AND app_name = ?");
            params_vec.push(Box::new(app_name.clone()));
        }

        if let Some(ref start_date) = filter.start_date {
            sql.push_str(" AND timestamp >= ?");
            params_vec.push(Box::new(start_date.clone()));
        }

        if let Some(ref end_date) = filter.end_date {
            sql.push_str(" AND timestamp <= ?");
            params_vec.push(Box::new(end_date.clone()));
        }

        sql.push_str(" ORDER BY timestamp DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;

        let records = stmt.query_map(params_refs.as_slice(), |row| {
            let timestamp_str: String = row.get(1)?;
            let timestamp = NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|_| Utc::now());

            let created_at_str: Option<String> = row.get(6)?;
            let created_at = created_at_str.and_then(|s| {
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                    .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    .ok()
            });

            Ok(InputRecord {
                id: Some(row.get(0)?),
                timestamp,
                app_name: row.get(2)?,
                window_title: row.get(3)?,
                content: row.get(4)?,
                key_count: row.get(5)?,
                created_at,
            })
        })?;

        records.collect()
    }

    pub fn get_today_stats(&self) -> Result<DailyStats> {
        let conn = self.conn.lock().unwrap();
        let today = Utc::now().format("%Y-%m-%d").to_string();

        let (total_keys, total_records): (i64, i64) = conn.query_row(
            "SELECT COALESCE(SUM(key_count), 0), COUNT(*) FROM inputs WHERE date(timestamp) = ?",
            params![today],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;

        let mut stmt = conn.prepare(
            "SELECT app_name, SUM(key_count), COUNT(*) FROM inputs WHERE date(timestamp) = ? GROUP BY app_name ORDER BY SUM(key_count) DESC",
        )?;

        let app_stats = stmt
            .query_map(params![today], |row| {
                Ok(AppStats {
                    app_name: row.get(0)?,
                    key_count: row.get(1)?,
                    record_count: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(DailyStats {
            date: today,
            total_keys,
            total_records,
            app_stats,
        })
    }

    pub fn get_app_list(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT DISTINCT app_name FROM inputs ORDER BY app_name")?;
        let apps = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        Ok(apps)
    }

    pub fn delete_records_before(&self, date: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count = conn.execute("DELETE FROM inputs WHERE timestamp < ?", params![date])?;
        Ok(count)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT value FROM settings WHERE key = ?",
            params![key],
            |row| row.get(0),
        );
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }
}
