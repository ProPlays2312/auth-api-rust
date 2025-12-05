use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct SqlxRepo {
    pub pool: PgPool,
}

impl SqlxRepo {
    pub async fn new(db_url: &str) -> Result<Self, anyhow::Error> {
        // 1. Create a variable for the URL (as requested)
        let url = db_url;

        println!("[+] Connecting to database...");

        // 2. Connect using the URL variable
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create pool: {e}"))?;

        // 3. Test the connection explicitly
        // We run a simple query to make sure the database is actually alive
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .map_err(|e| anyhow::anyhow!("[!] Database connection test failed: {e}"))?;

        println!("[+] Connection test passed! Initializing schema...");

        // 4. Initialize Tables (One-time setup)
        init_schema(&pool).await?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

// --- One-Time Initialization Logic ---
async fn init_schema(pool: &PgPool) -> Result<(), anyhow::Error> {

    // A. Timestamp Function
    sqlx::query(r"
        CREATE OR REPLACE FUNCTION update_timestamp()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ language 'plpgsql';
    ").execute(pool).await?;

    // B. Create Tables

    // Users Table
    sqlx::query(r"
        CREATE TABLE IF NOT EXISTS users (
            uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) NOT NULL UNIQUE,
            hash VARCHAR(255) NOT NULL,
            token_version INT DEFAULT 0,
            is_verified BOOLEAN DEFAULT FALSE,
            is_active BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    ").execute(pool).await?;

    // Profiles Table
    sqlx::query(r"
        CREATE TABLE IF NOT EXISTS profiles (
            user_uuid UUID PRIMARY KEY REFERENCES users(uuid) ON DELETE CASCADE,
            first_name VARCHAR(100),
            last_name VARCHAR(100),
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
    ").execute(pool).await?;

    // C. Triggers
    ensure_trigger(pool, "users", "update_users_modtime").await?;
    ensure_trigger(pool, "profiles", "update_profiles_modtime").await?;

    println!("[+] Schema initialized (Tables & Triggers ready).");
    Ok(())
}

// --- Trigger Helper ---
async fn ensure_trigger(pool: &PgPool, table: &str, trigger_name: &str) -> Result<(), anyhow::Error> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_trigger WHERE tgname = $1)"
    )
        .bind(trigger_name)
        .fetch_one(pool)
        .await?;

    if !exists {
        let q = format!(r"
            CREATE TRIGGER {trigger_name}
            BEFORE UPDATE ON {table}
            FOR EACH ROW EXECUTE FUNCTION update_timestamp();
        ");
        sqlx::query(&q).execute(pool).await?;
        println!("   -> Created missing trigger: {trigger_name}");
    }
    Ok(())
}

// --- Helper for running raw queries from other files ---
pub async fn run_raw_query(pool: &PgPool, query: &str) -> Result<u64, anyhow::Error> {
    let result = sqlx::query(query).execute(pool).await?;
    Ok(result.rows_affected())
}