use csv::Reader;
use reqwest::blocking::Client;
use rusqlite::{params, Connection, NO_PARAMS};
use std::error::Error;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut response = Client::new().get(url).send()?;
    let mut file = std::fs::File::create(file_path)?;
    std::io::copy(&mut response, &mut file)?;
    Ok(())
}

//write a function that creates a table in the database
pub fn create_table(conn: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS titanic (
            passenger_id INTEGER PRIMARY KEY AUTOINCREMENT,
            survived INTEGER,
            p_class INTEGER,
            name TEXT,
            sex TEXT,
            age TEXT,
            sib_sp INTEGER,
            parch INTEGER,
            ticket TEXT,
            fare REAL,
            cabin TEXT,
            embarked TEXT
        )",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn load_transform(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a SQLite database connection
    let conn = Connection::open("Titanic.db")?;

    // Open the CSV file and read its contents
    let file = std::fs::File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    // Prepare a SQL statement for insertion
    let mut stmt = conn.prepare(
        "INSERT INTO titanic (
            passenger_id,
            survived,
            p_class,
            name,
            sex,
            age,
            sib_sp,
            parch,
            ticket,
            fare,
            cabin,
            embarked
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )?;

    // Iterate over the CSV records and insert them into the database
    for result in rdr.records() {
        let record = result?;
        let id: i32 = record[0].parse().unwrap();
        let survived: i32 = record[1].parse().unwrap();
        let pclass: i32 = record[2].parse().unwrap();
        let name = &record[3];
        let sex = &record[4];
        let age = &record[5];
        let sibsp: i32 = record[6].parse().unwrap();
        let parch: i32 = record[7].parse().unwrap();
        let ticket = &record[8];
        let fare: f64 = record[9].parse()?;
        let cabin = &record[10];
        let embarked = &record[11];

        // Execute the SQL statement
        stmt.execute(params![
            id, survived, pclass, name, sex, age, sibsp, parch, ticket, fare, cabin, embarked
        ])?;
    }

    Ok(())
}

//write a function that inserts a record into the database
pub fn insert(
    c: &rusqlite::Connection,
    survived: i32,
    pclass: i32,
    name: &str,
    sex: &str,
    age: &str,
    sibsp: i32,
    parch: i32,
    ticket: &str,
    fare: f64,
    cabin: &str,
    embarked: &str,
) -> Result<(), rusqlite::Error> {
    c.execute(
        "INSERT INTO titanic (
            survived,
            p_class,
            name,
            sex,
            age,
            sib_sp,
            parch,
            ticket,
            fare,
            cabin,
            embarked
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![survived, pclass, name, sex, age, sibsp, parch, ticket, fare, cabin, embarked],
    )?;
    Ok(())
}

//write a function that reads the data from the database
pub fn read(
    c: &rusqlite::Connection,
) -> Result<(i32, i32, i32, String, String, String, i32, i32, String, f64, String, String), rusqlite::Error> {
    let mut stmt =
        c.prepare("SELECT
        passenger_id,
        survived,
        p_class,
        name,
        sex,
        age,
        sib_sp,
        parch,
        ticket,
        fare,
        cabin,
        embarked FROM titanic WHERE passenger_id = ?;")?;
    let rslt = stmt.query_row([1], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
            row.get(10)?,
            row.get(11)?,
        ))
    })?;

    // let mut indexs = Vec::new();
    // for index in rslt {
    //     indexs.push(index?);
    // }
    Ok(rslt)
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
pub fn delete(c: &rusqlite::Connection, name: &str) -> Result<(), rusqlite::Error> {
    c.execute("DELETE FROM titanic WHERE name = ?", &[name])?;
    Ok(())
}
