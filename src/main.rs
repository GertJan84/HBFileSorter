use anyhow::Result;
use std::{
    env,
    collections::HashMap,
    fs::{create_dir, rename, read_dir},
    path::Path,
    process::exit
};

fn main() -> Result<()> {

    //let args: Vec<String> = env::args().collect();
    let bin = env::current_exe()?;
    let current_directory = bin.parent().unwrap();

   // let current_directory = match args.len() {
   //     1 => {
   //         let bin = env::current_exe()?;
   //         bin.parent().unwrap()
   //     },
   //     2 => { // TODO: Implement for system executable
   //         let user_path = Path::new(&args[1]);
   //         if !user_path.exists() {
   //             eprintln!("Can't find: {}", args[1]);
   //             exit(0)
   //         }
   //         user_path
   //     },
   //     _ => {
   //         eprintln!("Usages: ./HBFileSorter empty");
   //         exit(0)
   //     }


   // };

    println!("{}", current_directory.to_str().unwrap());

    // {"Microservices": ["pdf", epub]}
    let mut files: HashMap<String, Vec<&str>> = HashMap::new();

    for res in read_dir(current_directory)? {
        let entry = match res {
            Ok(e) => e,
            Err(_) => continue
        };

        let entry_path = entry.path();

        if entry_path == current_directory {
            continue
        }

        if !entry_path.is_file() {
            continue
        }

        if let Some(extention) = entry_path.extension() {
            let filename = entry_path.file_stem().unwrap().to_str().unwrap();

            if !files.get(filename).is_some() {
                files.insert(filename.to_string(), Vec::new());
            }

            match extention.to_str().unwrap() {
                "pdf" => {
                    files.entry(filename.to_string()).or_default().push("pdf")
                },
                "epub" => {
                    files.entry(filename.to_string()).or_default().push("epub")
                },
                _ => continue
            }
        }


    }

    for (filename, extentions) in files.into_iter() {
        let dest_path = current_directory.join(&filename);

        println!("Create folder: {}", &filename);
        if let Err(err) = create_dir(&dest_path) {
            eprintln!("{}", err);
            continue
        }

        for extention in extentions {
            let full_filename = format!("{}.{}", filename, extention);
            println!("Move: {}", full_filename,);
            rename(&full_filename, &dest_path.join(&full_filename))?
        }
    }

    Ok(())
}
