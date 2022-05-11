use std::io;
use std::mem::drop;
use std::io::Write;
use std::fs::ReadDir;
mod file_handling;

fn main() -> io::Result<()> {
    let mut folders: Vec<ReadDir> = Vec::new();
    let mut songs: Vec<file_handling::Needed> = Vec::new();

    loop {
        let mut input: String = String::new();
        print!("Would you like to add/add another path?[Y/N]: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        match &input.trim().to_uppercase()[..] {
            "N" => break,
            "Y" => {
                folders.push(file_handling::get_folders()?);
                drop(input);
                continue;
            }
            _   => {
                println!("Invalid input");
                drop(input);
                continue;
            }
        }
    }

    file_handling::parse_folder(folders, &mut songs)?;

    for song in songs{
        println!("{}", song);
    }

    return Ok(());
}
