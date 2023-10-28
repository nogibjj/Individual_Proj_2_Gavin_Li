use reqwest::blocking::Client;
use csv::Reader;
use std::error::Error;
use rusqlite::{Connection, params, NO_PARAMS};

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut response = Client::new().get(url).send()?;
    let mut file = std::fs::File::create(file_path)?;
    std::io::copy(&mut response, &mut file)?;
    Ok(())
}

//write a function that creates a table in the database
pub fn create_table(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS indexs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name_cap_2 TEXT,
            num_rom_ca TEXT,
            Shape_Leng REAL,
            Shape_Area REAL
        )",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn load_transform(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a SQLite database connection
    let conn = Connection::open("ktopomapseriesindexDB.db")?;

    // Open the CSV file and read its contents
    let file = std::fs::File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    // Prepare a SQL statement for insertion
    let mut stmt = conn.prepare("INSERT INTO indexs (name_cap_2, num_rom_ca, Shape_Leng, Shape_Area) VALUES (?, ?, ?, ?)")?;

    // Iterate over the CSV records and insert them into the database
    for result in rdr.records() {
        let record = result?;
        let name_cap_2 = &record[0];
        let num_rom_ca = &record[1];
        let shape_leng: f64 = record[2].parse()?;
        let shape_area: f64 = record[3].parse()?;
        
        // Execute the SQL statement
        stmt.execute(params![name_cap_2, num_rom_ca, shape_leng, shape_area])?;
    }

    Ok(())
}

//write a function that inserts a record into the database
pub fn insert(
    c: &rusqlite::Connection,
    name_cap_2: &str,
    num_rom_ca: &str,
    shape_leng: f64,
    shape_area: f64,
) -> Result<(), rusqlite::Error> {
    c.execute(
        "INSERT INTO indexs (name_cap_2, num_rom_ca, Shape_Leng, Shape_Area) VALUES (?, ?, ?, ?)",
        params![name_cap_2, num_rom_ca, &shape_leng, &shape_area],
    )?;
    Ok(())
}

//write a function that reads the data from the database
pub fn read(c: &rusqlite::Connection) -> Result<Vec<(i64, String, String, f64, f64)>, rusqlite::Error> {
    let mut stmt = c.prepare("SELECT id, name_cap_2, num_rom_ca, Shape_Leng, Shape_Area FROM indexs")?;
    let indexs_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })?;

    let mut indexs = Vec::new();
    for index in indexs_iter {
        indexs.push(index?);
    }
    Ok(indexs)
}

//write a function that updates the shape length
pub fn update_shape_leng(
    c: &rusqlite::Connection,
    shape_leng: f64,
    num_rom_ca: &str,
) -> Result<(), rusqlite::Error> {
    c.execute(
        "UPDATE indexs SET Shape_Leng = ? WHERE num_rom_ca = ?",
        params![shape_leng, num_rom_ca],
    )?;
    Ok(())
}

//write a function that deletes a record from the database
pub fn delete(c: &rusqlite::Connection, num_rom_ca: &str) -> Result<(), rusqlite::Error> {
    c.execute("DELETE FROM indexs WHERE num_rom_ca = ?", &[num_rom_ca])?;
    Ok(())
}