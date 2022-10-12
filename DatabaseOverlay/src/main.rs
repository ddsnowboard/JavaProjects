fn main() {
    println!("Hello, world!");
}

trait ColumnValue: sqlite::ValueInto + Sized {
    fn convert(v: &sqlite::Value) -> Option<Self>
    where
        Self: Sized;
}

impl<T: sqlite::ValueInto> ColumnValue for T {
    fn convert(v: &sqlite::Value) -> Option<T> {
        Self::into(v)
    }
}

trait Row {
    fn read<T: ColumnValue>(&self, n: usize) -> T;
}

struct SqlError {
    pub message: String,
}

impl From<sqlite::Error> for SqlError {
    fn from(err: sqlite::Error) -> Self {
        Self {
            message: err.message.unwrap_or_else(|| "unknown error".to_string()),
        }
    }
}

trait DataSource<T: Row> {
    fn query<'a>(
        &'a mut self,
        q: &str,
    ) -> Result<Box<dyn Iterator<Item = Result<T, SqlError>> + 'a>, SqlError>;
}

impl Row for sqlite::Row {
    fn read<T: sqlite::ValueInto>(&self, n: usize) -> T {
        self.get(n)
    }
}

struct SqliteDataSource {
    connection: sqlite::Connection,
}

impl SqliteDataSource {
    fn new(database: &str) -> Result<Self, sqlite::Error> {
        Ok(Self {
            connection: sqlite::open(database)?,
        })
    }
}

impl DataSource<sqlite::Row> for SqliteDataSource {
    fn query(
        &mut self,
        q: &str,
    ) -> Result<Box<dyn Iterator<Item = Result<sqlite::Row, SqlError>> + '_>, SqlError> {
        let statement = self.connection.prepare(q).map_err(SqlError::from)?;
        Ok(Box::new(
            statement
                .into_cursor()
                .map(|res| res.map_err(SqlError::from)),
        ))
    }
}

struct OverlayDataSource<RowType:Row> {
    sources:Vec<Box<dyn DataSource<RowType>>>
}

// Implement actual merging here? Sounds complicated
