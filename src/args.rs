    use colored::Colorize;

    use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Error};
    use std::{
        fs::OpenOptions,
        io::{Read, Seek, SeekFrom, Write}, str::FromStr,
    };

    /// Encode a payload to a file
    pub fn encode(file_name: &str, chunk_type: &str, payload: &str) -> Result<(), Error> {
        let temp_file_path = format!("{}.temp", file_name);

        // Move the contents of the file to a temporary location
        std::fs::File::create(&temp_file_path)?;
        std::fs::copy(file_name, &temp_file_path)?;

        // The PNG file to encode to
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&temp_file_path)
        {
            // Return file if ok
            Ok(file) => file,

            // Tell user there is an error
            Err(error) => {
                println!(
                    "{} '{}'",
                    "Failed to open file".red().bold(),
                    file_name.bold()
                );
                return Err(error.into());
            }
        };

        // Read the contents of the file to the `data` vec

        let mut data: Vec<u8> = Vec::new();

        if let Err(error) = file.read_to_end(&mut data) {
            println!(
                "{}",
                "An internal error has occured, please try again."
                    .red()
                    .bold()
            );
            return Err(error.into());
        }

        // Try to convert the bytes into a `Png` struct
        let mut png = match Png::try_from(&data[..]) {
            Ok(png) => png,
            Err(error) => {
                println!(
                    "{}",
                    "A bad PNG file has been given, the given PNG file may be corrupted."
                        .red()
                        .bold()
                );
                return Err(error);
            }
        };

        // Try to create a ChunkType struct from the chunk type
        let chunk_type = match ChunkType::from_str(chunk_type) {
            Ok(chunk_type) => chunk_type,
            Err(error) => {
                println!(
                    "{}",
                    "A bad chunk type has been given, make sure it is all valid ASCII and that the third char is uppercase."
                        .red()
                        .bold()
                );
                return Err(error);
            }
        };

        // Create a new chunk from the chunk type and the payload (converted to a Vec<u8>)
        let chunk = Chunk::new(chunk_type, payload.as_bytes().to_vec());

        // Add the chunk to the PNG file
        png.append_chunk(chunk);

        file.seek(SeekFrom::Start(0))?;

        file.write_all(&png.as_bytes()[..])?;

        // Write the contents of the temp file to the PNG file
        std::fs::remove_file(file_name)?;
        std::fs::rename(&temp_file_path, file_name)?;

        Ok(())
    }

    pub fn decode(file_name: &str, chunk_type: &str) -> Result<(), Error> {
        // Try to read the file
        let mut file = match OpenOptions::new().read(true).open(file_name) {
            Ok(file) => file,
            Err(error) => {
                println!(
                    "{} '{}'",
                    "Failed to read file".red().bold(),
                    file_name.white().bold()
                );
                return Err(error.into());
            }
        };

        // Create a data vec and try to write to it.
        let mut data: Vec<u8> = Vec::new();

        if let Err(error) = file.read_to_end(&mut data) {
            println!(
                "{}",
                "An internal error has occurred, please try again."
                    .red()
                    .bold()
            );
            return Err(error.into());
        }

        // Try to create a PNG struct from the file
        let png = match Png::try_from(&data[..]) {
            Ok(png) => png,
            Err(error) => {
                println!(
                    "{}",
                    "A bad PNG file has been given, the given PNG file may be corrupted."
                        .red()
                        .bold()
                );
                return Err(error);
            }
        };

        match png.chunk_by_type(chunk_type) {
            Some(chunk) => {
                println!(
                    "{} '{}'",
                    "Found chunk with type".green().bold(),
                    chunk_type.white().bold()
                );

                print!("{} ", "Message:".white().bold());
                println!("{}", chunk.data_as_string()?);
            }
            None => {
                println!(
                    "{} '{}'",
                    "Failed to find chunk with type".red().bold(),
                    chunk_type.white().bold()
                );
            }
        };

        Ok(())
    }

    pub fn remove(file_name: &str, chunk_type: &str) -> Result<(), Error> {
        let temp_file_path = format!("{}.temp", file_name);

        // Move the contents of the file to a temporary location
        std::fs::File::create(&temp_file_path)?;
        std::fs::copy(file_name, &temp_file_path)?;

        // The PNG file to encode to
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&temp_file_path)
        {
            // Return file if ok
            Ok(file) => file,

            // Tell user there is an error
            Err(error) => {
                println!(
                    "{} '{}'",
                    "Failed to open file".red().bold(),
                    file_name.bold()
                );
                return Err(error.into());
            }
        };

        let mut data = Vec::new();

        file.read_to_end(&mut data)?;

        let mut png = Png::try_from(&data[..])?;



        if let Err(error) = png.remove_chunk(chunk_type) {
            println!(
                "'{}' {}",
                chunk_type.white().bold(),
                "was not found".white().bold()
            );
            return Err(error);
        }

        println!("{}", "Removed chunk from file successfully".green().bold());

        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;


        file.write_all(&png.as_bytes()[..])?;


        // Write the contents of the temp file to the PNG file
        std::fs::remove_file(file_name)?;
        std::fs::rename(&temp_file_path, file_name)?;

        Ok(())
    }
