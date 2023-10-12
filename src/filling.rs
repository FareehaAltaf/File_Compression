pub mod filling{
    
    use std::fs::File;
    use std::io::Write;
    use std::io::Read;
    use std::fs;
    
    pub fn write_to_file(filename: &str, vec: &Vec<(u32, u32)>) -> std::io::Result<()> { // this will write as and bs to file according to number in vector
        let mut file = File::create(filename)?;
        for pair in vec {
            for _ in 0..pair.0 {
                write!(file, "a")?;
            }
            for _ in 0..pair.1 { 
                write!(file, "b")?;
            }
        }
        Ok(()) // success
    }

    // function to read the file
    pub fn read_file(filename: &str, compressed: &mut String) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        println!("{}", contents);

        // go through each charater of the contents
        let mut flag_prev_was_a = false; // flag to check if previous char was a

        let mut count_a = 0;   // count of a
        let mut count_b = 0;   // count of b

        let mut char_contents = contents.chars();

        // access the first character
        if let Some(first_char) = char_contents.next() {
            if first_char == 'a' {
                compressed.push_str("a");
                flag_prev_was_a = true;
                count_a += 1;
            } else {
                flag_prev_was_a = false;
            }
        }

        for c in char_contents {
            if c == 'a' && flag_prev_was_a { // same letter (a => a)
                count_a += 1;
            }
            else if c == 'b' && flag_prev_was_a { // change in letter (a => b) 
                compressed.push_str("b");
                flag_prev_was_a = false;
                count_b += 1;
                println!("There were {} a's", count_a);
                count_a = 0;
            }
            else if c == 'a' && !flag_prev_was_a { // change in letter (b => a)
                compressed.push_str("a");
                flag_prev_was_a = true;
                count_a += 1;
                println!("There were {} b's", count_b);
                count_b = 0;
            }
            else if c == 'b' && !flag_prev_was_a { //   same letter (b => b)
                count_b += 1;
            }
        }
        Ok(())
    }

    pub fn get_file_size(filename: &str) -> std::io::Result<u64> { // to check the file size
        let metadata = fs::metadata(filename)?;  
        Ok(metadata.len()) // len of data
    }
}