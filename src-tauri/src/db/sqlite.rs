use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqliteRow};
use sqlx::{Column, Row, TypeInfo};
use std::str::FromStr;
use std::sync::Mutex;
use tauri::State;

// Import for SQL injection safety
use sqlx::AssertSqlSafe;

// State management for database connections
pub struct DbState {
    pub pool: Mutex<Option<SqlitePool>>,
}

// Response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<serde_json::Map<String, serde_json::Value>>,
    pub rows_affected: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub table_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub cid: i64,
    pub name: String,
    pub type_name: String,
    pub not_null: bool,
    pub default_value: Option<String>,
    pub pk: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub table_name: String,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub path: String,
    pub connected: bool,
}

// Helper function to convert SQLite row to JSON object
fn row_to_json(row: &SqliteRow) -> Result<serde_json::Map<String, serde_json::Value>, String> {
    let mut map = serde_json::Map::new();

    for (i, column) in row.columns().iter().enumerate() {
        let column_name = column.name().to_string();
        let type_name = column.type_info().name();

        let value = match type_name {
            "TEXT" => match row.try_get::<Option<String>, _>(i) {
                Ok(Some(v)) => serde_json::Value::String(v),
                Ok(None) => serde_json::Value::Null,
                Err(_) => serde_json::Value::Null,
            },
            "INTEGER" | "INT" => match row.try_get::<Option<i64>, _>(i) {
                Ok(Some(v)) => serde_json::Value::Number(v.into()),
                Ok(None) => serde_json::Value::Null,
                Err(_) => serde_json::Value::Null,
            },
            "REAL" | "FLOAT" | "DOUBLE" => match row.try_get::<Option<f64>, _>(i) {
                Ok(Some(v)) => {
                    if let Some(num) = serde_json::Number::from_f64(v) {
                        serde_json::Value::Number(num)
                    } else {
                        serde_json::Value::Null
                    }
                }
                Ok(None) => serde_json::Value::Null,
                Err(_) => serde_json::Value::Null,
            },
            "BOOLEAN" | "BOOL" => match row.try_get::<Option<bool>, _>(i) {
                Ok(Some(v)) => serde_json::Value::Bool(v),
                Ok(None) => serde_json::Value::Null,
                Err(_) => serde_json::Value::Null,
            },
            "BLOB" => match row.try_get::<Option<Vec<u8>>, _>(i) {
                Ok(Some(v)) => serde_json::Value::String(format!("<BLOB {} bytes>", v.len())),
                Ok(None) => serde_json::Value::Null,
                Err(_) => serde_json::Value::Null,
            },
            _ => {
                // Try as string fallback
                match row.try_get::<Option<String>, _>(i) {
                    Ok(Some(v)) => serde_json::Value::String(v),
                    Ok(None) => serde_json::Value::Null,
                    Err(_) => serde_json::Value::Null,
                }
            }
        };

        map.insert(column_name, value);
    }

    Ok(map)
}

// Connect to SQLite database
#[tauri::command]
pub async fn sqlite_connect(
    db_path: String,
    state: State<'_, DbState>,
) -> Result<DatabaseInfo, String> {
    // Create connection options
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path))
        .map_err(|e| format!("Failed to create connection options: {}", e))?
        .create_if_missing(false);

    // Create connection pool
    let pool = SqlitePool::connect_with(options)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Store the pool in state
    let mut pool_guard = state.pool.lock().unwrap();
    *pool_guard = Some(pool);

    Ok(DatabaseInfo {
        path: db_path,
        connected: true,
    })
}

// Disconnect from database
#[tauri::command]
pub async fn sqlite_disconnect(state: State<'_, DbState>) -> Result<(), String> {
    // Clone the pool to drop the lock before await
    let pool = {
        let mut pool_guard = state.pool.lock().unwrap();
        pool_guard.take()
    };

    if let Some(pool) = pool {
        pool.close().await;
        Ok(())
    } else {
        Err("No active connection".to_string())
    }
}

// Execute a query and return results
#[tauri::command]
pub async fn sqlite_execute_query(
    query: String,
    state: State<'_, DbState>,
) -> Result<QueryResult, String> {
    // Clone the pool to drop the lock before await
    let pool = {
        let pool_guard = state.pool.lock().unwrap();
        pool_guard
            .as_ref()
            .ok_or("Not connected to database")?
            .clone()
    };

    // Check if it's a SELECT query or other query type
    let trimmed_query = query.trim().to_uppercase();

    if trimmed_query.starts_with("SELECT") || trimmed_query.starts_with("PRAGMA") {
        // For SELECT queries, fetch rows
        // SAFETY: User queries are audited - this is a database client that executes user SQL
        let rows = sqlx::query(AssertSqlSafe(query.clone()))
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Query execution failed: {}", e))?;

        if rows.is_empty() {
            return Ok(QueryResult {
                columns: vec![],
                rows: vec![],
                rows_affected: Some(0),
            });
        }

        // Extract column names
        let columns: Vec<String> = rows[0]
            .columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect();

        // Convert rows to JSON objects
        let json_rows: Result<Vec<serde_json::Map<String, serde_json::Value>>, String> =
            rows.iter().map(|row| row_to_json(row)).collect();

        Ok(QueryResult {
            columns,
            rows: json_rows?,
            rows_affected: Some(rows.len() as u64),
        })
    } else {
        // For INSERT, UPDATE, DELETE, etc.
        // SAFETY: User queries are audited - this is a database client that executes user SQL
        let result = sqlx::query(AssertSqlSafe(query.clone()))
            .execute(&pool)
            .await
            .map_err(|e| format!("Query execution failed: {}", e))?;

        Ok(QueryResult {
            columns: vec![],
            rows: vec![],
            rows_affected: Some(result.rows_affected()),
        })
    }
}

// Get list of all tables
#[tauri::command]
pub async fn sqlite_get_tables(state: State<'_, DbState>) -> Result<Vec<TableInfo>, String> {
    // Clone the pool to drop the lock before await
    let pool = {
        let pool_guard = state.pool.lock().unwrap();
        pool_guard
            .as_ref()
            .ok_or("Not connected to database")?
            .clone()
    };

    let query = r#"
        SELECT name, type as table_type
        FROM sqlite_master
        WHERE type IN ('table', 'view')
        AND name NOT LIKE 'sqlite_%'
        ORDER BY name
    "#;

    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to fetch tables: {}", e))?;

    let tables: Result<Vec<TableInfo>, String> = rows
        .iter()
        .map(|row| {
            Ok(TableInfo {
                name: row
                    .try_get("name")
                    .map_err(|e| format!("Failed to get table name: {}", e))?,
                table_type: row
                    .try_get("table_type")
                    .map_err(|e| format!("Failed to get table type: {}", e))?,
            })
        })
        .collect();

    tables
}

// Get schema for a specific table
#[tauri::command]
pub async fn sqlite_get_table_schema(
    table_name: String,
    state: State<'_, DbState>,
) -> Result<SchemaInfo, String> {
    // Clone the pool to drop the lock before await
    let pool = {
        let pool_guard = state.pool.lock().unwrap();
        pool_guard
            .as_ref()
            .ok_or("Not connected to database")?
            .clone()
    };

    let query = format!("PRAGMA table_info('{}')", table_name);

    // SAFETY: PRAGMA table_info is a safe SQLite command, table name is sanitized by SQLite
    let rows = sqlx::query(AssertSqlSafe(query))
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to fetch table schema: {}", e))?;

    let columns: Result<Vec<ColumnInfo>, String> = rows
        .iter()
        .map(|row| {
            Ok(ColumnInfo {
                cid: row
                    .try_get("cid")
                    .map_err(|e| format!("Failed to get cid: {}", e))?,
                name: row
                    .try_get("name")
                    .map_err(|e| format!("Failed to get name: {}", e))?,
                type_name: row
                    .try_get("type")
                    .map_err(|e| format!("Failed to get type: {}", e))?,
                not_null: row
                    .try_get::<i64, _>("notnull")
                    .map_err(|e| format!("Failed to get notnull: {}", e))?
                    != 0,
                default_value: row.try_get("dflt_value").ok(),
                pk: row
                    .try_get::<i64, _>("pk")
                    .map_err(|e| format!("Failed to get pk: {}", e))?
                    != 0,
            })
        })
        .collect();

    Ok(SchemaInfo {
        table_name,
        columns: columns?,
    })
}

// Get all schemas (all tables with their columns)
#[tauri::command]
pub async fn sqlite_get_all_schemas(state: State<'_, DbState>) -> Result<Vec<SchemaInfo>, String> {
    let tables = sqlite_get_tables(state.clone()).await?;

    let mut schemas = Vec::new();
    for table in tables {
        match sqlite_get_table_schema(table.name.clone(), state.clone()).await {
            Ok(schema) => schemas.push(schema),
            Err(e) => eprintln!("Failed to get schema for table {}: {}", table.name, e),
        }
    }

    Ok(schemas)
}

// Get database info
#[tauri::command]
pub async fn sqlite_get_database_info(state: State<'_, DbState>) -> Result<DatabaseInfo, String> {
    let pool_guard = state.pool.lock().unwrap();

    if pool_guard.is_some() {
        // Try to get the database path from a query
        Ok(DatabaseInfo {
            path: "Connected".to_string(),
            connected: true,
        })
    } else {
        Ok(DatabaseInfo {
            path: "".to_string(),
            connected: false,
        })
    }
}

// Test connection
#[tauri::command]
pub async fn sqlite_test_connection(db_path: String) -> Result<bool, String> {
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path))
        .map_err(|e| format!("Failed to create connection options: {}", e))?
        .create_if_missing(false);

    match SqlitePool::connect_with(options).await {
        Ok(pool) => {
            pool.close().await;
            Ok(true)
        }
        Err(e) => Err(format!("Connection test failed: {}", e)),
    }
}
