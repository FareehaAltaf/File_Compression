use std::fs::File;
use std::io;
use std::io::Write;

mod filling;
mod random;

fn main() {
    // large random text file
    let ab_list = random::random::gen_random_ab();
    println!("{:#?}", ab_list);  //testing purposes

    // write this to file
    filling::filling::write_to_file("ab_list.txt", &ab_list).expect("Unable to write to file");

    // Read the file and compress it
    let mut compressed = String::new();

    filling::filling::read_file("ab_list.txt", &mut compressed).expect("Unable to read file");

    println!("file compressed: {}", compressed);

    // write compressed string to new file

    let mut file = File::create("compressed.txt").expect("Unable to create file");
    file.write_all(compressed.as_bytes())
        .expect("Unable to write to file");

    // check file size before and after

    let before = filling::filling::get_file_size("ab_list.txt").expect("Unable to get file size");
    let after = filling::filling::get_file_size("compressed.txt").expect("Unable to get file size");
    println!("\nFile size before compression: {} bytes", before);
    println!("File size after compression: {} bytes", after);
    println!(
        "=> Compression ratio: {}%",
        (1.0 - (after as f64 / before as f64)) * 100.0
    );
    println!("");

    // decompress the file
    // Check for exit condition
    println!("Do you want to decompress? (yes/no)");
    let mut exit_command = String::new();

    // Read user input and store it in exit_command
    io::stdin()
        .read_line(&mut exit_command)
        .expect("Failed to read input");

    if exit_command.trim().to_lowercase() == "yes" {
        let mut decomp_str = String::new();
        filling::filling::decompress("compressed.txt", &mut decomp_str).expect("Unable to decompress file");
    } else {
        println!("Goodbye Compresser!")
    }; // Exit the loop
}
