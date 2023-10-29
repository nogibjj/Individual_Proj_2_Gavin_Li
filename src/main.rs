extern crate csv;

use individual_proj_2_gavin_li::create_table;
use individual_proj_2_gavin_li::delete;
use individual_proj_2_gavin_li::extract;
use individual_proj_2_gavin_li::insert;
use individual_proj_2_gavin_li::load_transform;
use individual_proj_2_gavin_li::read;
use individual_proj_2_gavin_li::update_shape_leng;
use rusqlite::Connection;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://raw.githubusercontent.com/nogibjj/Individual_Proj_2_Gavin_Li/main/resources/train.csv";
    let file_path = "train.csv";
    let db_path = "Titanic.db";

    let db_exists = std::path::Path::new(db_path).exists();

    extract(url, file_path)?;

    let conn = Connection::open(db_path)?;
    create_table(&conn)?;
    if !db_exists {
        load_transform(file_path)?;
    }

    // let name_cap_2 = "Test Name";
    // let num_rom_ca = "Test Num";
    // let shape_leng = 1.1;
    // let shape_area = 2.2;

    insert(
        &conn,
        1,
        1,
        "Some Random Name",
        "male",
        "26",
        0,
        1,
        "PC 18000",
        0.0,
        "C86",
        "c",
    )?;
    println!("Insertion Succeeded");

    match read(&conn) {
        Ok(rslt) => {
            println!(
                "passenger_id: {}, survived: {}, p_class: {}, name: {}, sex: {}, age: {}, sib_sp: {}, parch: {}, ticket: {}, fare:{}, cabin: {}, embarked: {}",
                rslt.0, rslt.1, rslt.2, rslt.3, rslt.4, rslt.5, rslt.6, rslt.7, rslt.8, rslt.9,rslt.10,rslt.11
            );
        }
        Err(err) => {
            eprintln!("Error reading data: {}", err);
        }
    }

    let new_name = "new name";
    let old_name = "Some Random Name";

    match update_shape_leng(&conn, new_name, old_name) {
        Ok(_) => {
            println!("Updated passenger's name from {} to {}", old_name, new_name);
        }
        Err(err) => {
            eprintln!("Error updating data: {}", err);
        }
    }

    // Prepare a SQL statement to select a single row based on num_rom_ca
    let mut stmt = conn.prepare("SELECT * FROM titanic WHERE name = ?")?;

    // Bind the parameter and execute the query
    let rslt = stmt.query_row(&[new_name], |row| {
        Ok((
            row.get::<usize, i32>(0)?,
            row.get::<usize, i32>(1)?,
            row.get::<usize, i32>(2)?,
            row.get::<usize, String>(3)?,
            row.get::<usize, String>(4)?,
            row.get::<usize, String>(5)?,
            row.get::<usize, i32>(6)?,
            row.get::<usize, i32>(7)?,
            row.get::<usize, String>(8)?,
            row.get::<usize, f64>(9)?,
            row.get::<usize, String>(10)?,
            row.get::<usize, String>(11)?,
        ))
    });

    match rslt {
        Ok(rslt) => {
            println!(
                "passenger_id: {}, survived: {}, p_class: {}, name: {}, sex: {}, age: {}, sib_sp: {}, parch: {}, ticket: {}, fare:{}, cabin: {}, embarked: {}",
                rslt.0, rslt.1, rslt.2, rslt.3, rslt.4, rslt.5, rslt.6, rslt.7, rslt.8, rslt.9,rslt.10,rslt.11
            );
        }
        Err(err) => {
            eprintln!("Error reading data: {}", err);
        }
    }

    // let test_name = "Some Random Name";

    match delete(&conn, new_name) {
        Ok(_) => {
            println!("Deleted record with name: {}", new_name);
        }
        Err(err) => {
            eprintln!("Error deleting data: {}", err);
        }
    }

    let mut stmt = conn.prepare("SELECT * FROM titanic WHERE name = ?")?;

    // Bind the parameter and execute the query
    let rslt = stmt.query_row(&[new_name], |row| {
        Ok((
            row.get::<usize, i32>(0)?,
            row.get::<usize, i32>(1)?,
            row.get::<usize, i32>(2)?,
            row.get::<usize, String>(3)?,
            row.get::<usize, String>(4)?,
            row.get::<usize, String>(5)?,
            row.get::<usize, i32>(6)?,
            row.get::<usize, i32>(7)?,
            row.get::<usize, String>(8)?,
            row.get::<usize, f64>(9)?,
            row.get::<usize, String>(10)?,
            row.get::<usize, String>(11)?,
        ))
    });

    match rslt {
        Ok(rslt) => {
            println!(
                "passenger_id: {}, survived: {}, p_class: {}, name: {}, sex: {}, age: {}, sib_sp: {}, parch: {}, ticket: {}, fare:{}, cabin: {}, embarked: {}",
                rslt.0, rslt.1, rslt.2, rslt.3, rslt.4, rslt.5, rslt.6, rslt.7, rslt.8, rslt.9,rslt.10,rslt.11
            );
        }
        Err(err) => {
            eprintln!("Error reading data: {}", err);
        }
    }

    Ok(())
}
