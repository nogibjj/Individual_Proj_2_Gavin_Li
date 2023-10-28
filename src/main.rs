extern crate csv;

use Individual_Proj_2_Gavin_Li::extract;
use Individual_Proj_2_Gavin_Li::create_table;
use Individual_Proj_2_Gavin_Li::load_transform;
use Individual_Proj_2_Gavin_Li::insert;
use Individual_Proj_2_Gavin_Li::read;
use Individual_Proj_2_Gavin_Li::update_shape_leng;
use Individual_Proj_2_Gavin_Li::delete;
use rusqlite::Connection;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://catalogue.data.wa.gov.au/dataset/f39087e2-2885-473e-bc62-ca610cd94340/resource/96c892f3-b387-410c-80d0-e4dcec68e6f2/download/25ktopomapseriesindex.csv";
    let file_path = "25ktopomapseriesindex.csv";
    let db_path = "ktopomapseriesindexDB.db";

    let db_exists = std::path::Path::new(db_path).exists();

    extract(url, file_path)?;
    
    let conn = Connection::open(db_path)?;
    create_table(&conn)?;
    if !db_exists {
        load_transform(file_path)?;
    }

    let name_cap_2 = "Test Name";
    let num_rom_ca = "Test Num";
    let shape_leng = 1.1;
    let shape_area = 2.2;

    insert(&conn, name_cap_2, num_rom_ca, shape_leng, shape_area)?;

    match read(&conn) {
        Ok(indexs) => {
            for (id, name_cap_2, num_rom_ca, shape_leng, shape_area) in indexs {
                println!("ID: {}, Name: {}, Num: {}, Shape Leng: {}, Shape Area: {}",
                         id, name_cap_2, num_rom_ca, shape_leng, shape_area);
            }
        }
        Err(err) => {
            eprintln!("Error reading data: {}", err);
        }
    }

    let new_shape_leng = 3.3;
    let num_rom_ca_to_update = "Test Num";

    match update_shape_leng(&conn, new_shape_leng, num_rom_ca_to_update) {
        Ok(_) => {
            println!("Updated Shape Leng for num_rom_ca {} to {}", num_rom_ca_to_update, new_shape_leng);
        }
        Err(err) => {
            eprintln!("Error updating data: {}", err);
        }
    }

    // Prepare a SQL statement to select a single row based on num_rom_ca
    let mut stmt = conn.prepare("SELECT id, name_cap_2, num_rom_ca, Shape_Leng, Shape_Area FROM indexs WHERE num_rom_ca = ?")?;

    // Bind the parameter and execute the query
    let row = stmt.query_row(&[num_rom_ca_to_update], |row| {
        Ok((
            row.get::<usize, i32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, f64>(3)?,
            row.get::<usize, f64>(4)?,
        ))
    });

    match row {
        Ok((id, name_cap_2, num_rom_ca, shape_leng, shape_area)) => {
            println!("Updated row: ID: {}, Name: {}, Number: {}, Shape Leng: {}, Shape Area: {}",
                id, name_cap_2, num_rom_ca, shape_leng, shape_area);
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No record found with num_rom_ca: {}", num_rom_ca_to_update);
        }
        Err(err) => {
            eprintln!("Error reading data: {}", err);
        }
    }

    let num_rom_ca_to_delete = "Test Num";

    match delete(&conn, num_rom_ca_to_delete) {
        Ok(_) => {
            println!("Deleted record with num_rom_ca: {}", num_rom_ca_to_delete);
        }
        Err(err) => {
            eprintln!("Error deleting data: {}", err);
        }
    }

    // Prepare a SQL statement to select a single row based on num_rom_ca
    let mut stmt = conn.prepare("SELECT id, name_cap_2, num_rom_ca, Shape_Leng, Shape_Area FROM indexs WHERE num_rom_ca = ?")?;

    // Bind the parameter and execute the query
    let row = stmt.query_row(&[num_rom_ca_to_update], |row| {
        Ok((
            row.get::<usize, i32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, f64>(3)?,
            row.get::<usize, f64>(4)?,
        ))
    });

    match row {
        Ok((id, name_cap_2, num_rom_ca, shape_leng, shape_area)) => {
            println!("Updated row: ID: {}, Name: {}, Number: {}, Shape Leng: {}, Shape Area: {}",
                id, name_cap_2, num_rom_ca, shape_leng, shape_area);
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("Delete Successfully! No record found with num_rom_ca: {}", num_rom_ca_to_update);
        }
        Err(err) => {
            eprintln!("Error reading data: {}", err);
        }
    }

    Ok(())
}