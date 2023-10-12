use std::fs::File;
use std::io::Write;
mod random;
mod filling;

fn main() {
    // large random text file
    let ab_list = random::random::gen_random_ab();
    println!("{:#?}", ab_list); 

    // write this to file
    filling::filling::write_to_file("ab_list.txt", &ab_list).expect("Unable to write to file");

    // Read the file and compress it
    let mut compressed = String::new();

    filling::filling::read_file("ab_list.txt", &mut compressed).expect("Unable to read file");

    println!("file compressed: {}", compressed);

    // write compressed string to new file

    let mut file = File::create("compressed.txt").expect("Unable to create file");
    file.write_all(compressed.as_bytes()).expect("Unable to write to file");

    // check file size before and after 

    let before = filling::filling::get_file_size("ab_list.txt").expect("Unable to get file size");
    let after = filling::filling::get_file_size("compressed.txt").expect("Unable to get file size");
    println!("File size before compression: {} bytes", before);
    println!("File size after compression: {} bytes", after);
    println!("Compression ratio: {}%", (1.0 - (after as f64 / before as f64)) * 100.0);
    
}
