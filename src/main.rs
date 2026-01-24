use std::io::{self, Write};
use std::collections::HashMap;
use fool_database::db::{Column, ColumnType, DataType, Database, Row};
use fool_database::sql::{Command, execute, QueryResult, WhereClause};
fn main() {
  let mut db = Database::new();

  loop {
    println!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input == "exit" { break };        

    let parts: Vec<&str> = input.split_whitespace().collect();

    let command = match parts.as_slice() {
      ["create", "table", table_name, cols_desc @ ..] => {
        let mut columns = Vec::new();
        for desc in cols_desc {
          let pair: Vec<&str> = desc.split(':').collect();
          if pair.len() == 2 {
            let name = pair[0].to_string();
            let type_str = pair[1];

            let col_type = match type_str {
              "int" => ColumnType::Integer,
              "text" => ColumnType::Text,
              _ => {
                println!("Unknown column type: {}", type_str);
                continue;
              }
            };
            columns.push(Column { name, col_type });
          }
        }
        Some(Command::CreateTable {
          table_name: table_name.to_string(),
          columns,
        })
      },
      ["insert", table_name, kvs @ ..] => {
        let mut row: Row = HashMap::new();
        for kv in kvs {
          let pair: Vec<&str> = kv.split('=').collect();
          if pair.len() == 2 {
            let col_name = pair[0].to_string();
            let value_str = pair[1];

            let value = if let Ok(int_val) = value_str.parse::<i32>() {
              DataType::Integer(int_val)
            } else {
              DataType::Text(value_str.to_string())
            };
            row.insert(col_name, value);
          }
        }
        Some(Command::Insert {
          table_name: table_name.to_string(),
          row,
        })
      },
      ["select", table_name, "where", condition] => {
        let cond_parts: Vec<&str> = condition.split('=').collect();
        if cond_parts.len() == 2 {
          let col_name = cond_parts[0];
          let value = cond_parts[1];

          Some(Command::Select {
            table_name: table_name.to_string(),
            fields: vec![],
            where_clause: Some(WhereClause {
              column: col_name.to_string(),
              value: value.to_string(),
            }),
          })
        } else {
          println!("Invalid syntax. Use: select <table> where <col>=<val>");
          None
        }
      },
      ["select", table_name] => {
        Some(Command::Select {
          table_name: table_name.to_string(),
          fields: vec![],
          where_clause: None,
        })
      },
      _ => {
        println!("Unknown command");
        None
      }
    };
    if let Some(cmd) = command {
      match execute(&mut db, cmd) {
        QueryResult::Message(msg) => println!("{}", msg),
        QueryResult::Error(err) => println!("Error: {}", err),
        QueryResult::Rows(rows) => {
          for row in rows {
            println!("{:?}", row);
          }
        }
      }
    }
  }
}