pub struct SqlDb {}

impl SqlDb {
    pub fn new() {
        let conn = duckdb::Connection::open_in_memory();
        
    }
}