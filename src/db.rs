use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
  Integer,
  Text,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
  Integer(i32),
  Text(String),
}

#[derive(Debug, Clone)]
pub struct Column {
  pub name: String,
  pub col_type: ColumnType,
}

pub type Row = HashMap<String, DataType>;

#[derive(Debug, Clone)]
pub struct Table {
  pub name: String,
  pub columns: Vec<Column>,
  pub rows: Vec<Row>,
}

impl Table {
  pub fn new(name: String, columns: Vec<Column>) -> Self {
    Table {
      name,
      columns,
      rows: Vec::new(),
    }
  }

  pub fn insert(&mut self, row: Row) {
    self.rows.push(row);
  }
}

pub struct Database {
  pub tables: HashMap<String, Table>,
}

impl Database {
  pub fn new() -> Self {
    Database { tables: HashMap::new() }
  }

  pub fn create_table(&mut self, table_name: String, columns: Vec<Column>) {
    if !self.tables.contains_key(&table_name) {
      let table = Table::new(table_name.clone(), columns);
      self.tables.insert(table_name, table);
    }
  }

  pub fn get_table_mut(&mut self, table_name: &str) -> Option<&mut Table> {
    self.tables.get_mut(table_name)
  }

  pub fn get_table(&self, table_name: &str) -> Option<&Table> {
    self.tables.get(table_name)
  }   
}
