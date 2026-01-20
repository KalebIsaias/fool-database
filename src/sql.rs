use crate::db::{Database, DataType, Row, Column, ColumnType};

pub struct WhereClause {
  pub column: String,
  pub value: String,
}

pub enum Command {
  CreateTable {
    table_name: String,
    columns: Vec<Column>,
  },
  Insert {
    table_name: String,
    row: Row,
  },
  Select {
    table_name: String,
    fields: Vec<String>,
    where_clause: Option<WhereClause>,
  }
}

#[derive(Debug)]
pub enum QueryResult {
  Message(String),
  Rows(Vec<Row>),
  Error(String),
}

pub fn execute(db: &mut Database, command: Command) -> QueryResult {
  match command {
    Command::CreateTable {  table_name, columns } => {
      db.create_table(table_name.clone(), columns);
      QueryResult::Message(format!("Table '{}' created successfully.", table_name))
    }
    Command::Insert {  table_name, row } => {
      match db.get_table_mut(&table_name) {
        Some(table) => {
          for (col_name, val) in &row {
            let column_def = table.columns.iter().find(|c| &c.name == col_name);

            match column_def {
              Some(col) => {
                let is_valid = match (&col.col_type, val) {
                  (ColumnType::Integer, DataType::Integer(_)) => true,
                  (ColumnType::Text, DataType::Text(_)) => true,
                  _ => false,
                };
                if !is_valid {
                  return QueryResult::Error(
                    format!("Type mismatch for column '{}'. Expected {:?}, got {:?}", 
                    col_name, col.col_type, val)
                  );
                }
              },
              None => {
                return QueryResult::Error(format!(
                  "Column '{}' does not exist in table '{}'.", col_name, table_name
                ));
              }
            }
          }
          table.insert(row);
          QueryResult::Message(format!("Row inserted into table '{}'.", table_name))
        }
        None => QueryResult::Error(format!("Table '{}' does not exist.", table_name)),
      }
    }

    Command::Select { table_name, fields: _, where_clause } => {
      match db.get_table(&table_name) {
        Some(table) => {
          let mut result_rows = Vec::new();
          for row in &table.rows {
            let mut keep = true;
            if let Some(w) = &where_clause {
                if let Some(cell_value) = row.get(&w.column) {
                    let matches = match cell_value {
                        DataType::Text(t) => t == &w.value,
                        DataType::Integer(i) => i.to_string() == w.value,
                    };

                    if !matches { keep = false; }
                } else {
                  keep = false;
                }
            }
            if keep { result_rows.push(row.clone()); }
          }
          QueryResult::Rows(result_rows)
        }
        None => QueryResult::Error(format!("Table '{}' not found.", table_name)),
      }
    }
  }
}