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
                compressed.push_str(count_a.to_string().as_str()); // push the count of a
                compressed.push_str("b");
                flag_prev_was_a = false;
                count_b += 1;
                println!("There were {} a's", count_a);
                count_a = 0;
            }
            else if c == 'a' && !flag_prev_was_a { // change in letter (b => a)
                compressed.push_str(count_b.to_string().as_str()); // push the count of b
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
        compressed.push_str(count_b.to_string().as_str()); // push the count of last b
        println!("There were {} b's", count_b);
        Ok(())
    }

    pub fn get_file_size(filename: &str) -> std::io::Result<u64> { // to check the file size
        let metadata = fs::metadata(filename)?;  
        Ok(metadata.len()) // len of data
    }

    pub fn decompress(filename: &str, decompressed_str: &mut String) -> std::io::Result<()> {
		let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        println!("compressed file: {}", contents);

        // go through each charater of the contents
        let char_contents = contents.chars();

		// let mut character: char = 'a';
		let mut prev: char = 'a';

		let mut num = "0".to_string();
		//num = "0".to_string();

		//decompressed_str = "";

		for c in char_contents {
			
			if c == 'a' && prev == num.chars().last().unwrap() { // if i reach a and prev was a number
				for _ in 0..num.parse::<u32>().unwrap() { // write that many b's in the decompressed string
					decompressed_str.push_str('b'.to_string().as_str());
				}
				// character = 'a';
				num.clear();
			}
			else if c == 'b' && prev == num.chars().last().unwrap() { // if i reach b and prev was a number
				for _ in 0..num.parse::<u32>().unwrap() { // write that many a's in the decompressed string
					decompressed_str.push_str('a'.to_string().as_str());
				}
				// character = 'b';
				num.clear();
			}
			else if c.is_digit(10) {
				// store the number
				// this could be the first digit of a 2 digit number.
				//   or the 2nd digit of a 2 digit number
				//   or a single digit number
				num.push_str(c.to_string().as_str()); 
			}
			prev = c;
		}
		// at the end there will defo be a num stored for the last b's
		for _ in 0..num.parse::<u32>().unwrap() { // write that many b's in the decompressed string
			decompressed_str.push_str('b'.to_string().as_str());
		}

		println!("\ndecompressed file: {}", decompressed_str);

		let mut file = File::create("decompressed.txt").expect("Unable to create file");
		file.write_all(decompressed_str.as_bytes()).expect("Unable to write to file");

		Ok(())
        
	}
 }

 #[cfg(test)]
pub mod tests {
use crate::random::random::gen_random_ab;
use crate::filling::filling::read_file;
use crate::filling::filling::write_to_file;
use crate::filling::filling::decompress;
use std::io::Write;
use std::io::Read;
use std::fs;
use std::fs::File;
     
     #[test]
     fn test_write_to_file() {
         let list = gen_random_ab();
         let filename = "random_ab.txt";
         write_to_file(filename, &list).unwrap();
         let contents = fs::read_to_string(filename).unwrap();
         // loop thru the contents to see if they are a or b
         for c in contents.chars() {
             assert!(c == 'a' || c == 'b');
         }
     }
     #[test]
     fn test_read_file() {
         let filename = "random_ab.txt";
         let mut compressed = String::new();
         read_file(filename, &mut compressed).unwrap();
         // loop thru the contents to see if they are either a or b or number
         for c in compressed.chars() {
             assert!(c == 'a' || c == 'b' || c.is_digit(10));
         }
     }
     #[test]
     fn test_compress() {
         let filename = "random_ab.txt";
         let mut compressed = String::new();
         read_file(filename, &mut compressed).unwrap();
         let mut compressed_file = File::create("compressed.txt").unwrap();
         compressed_file.write_all(compressed.as_bytes()).unwrap();
     }
     #[test]
     fn test_decompress() {
         let filename = "compressed.txt";
         let mut decomp_str = String::new();
         decompress(filename, &mut decomp_str).expect("Error in decompression");
         for c in decomp_str.chars() {
             assert!(c == 'a' || c == 'b');
         }
         // read file to see if it is the same as decompressed string
         let mut file = File::open("decompressed.txt").expect("Couldn't open");
         let mut contents = String::new();
         file.read_to_string(&mut contents).expect("Couldn't read");
         assert_eq!(contents, decomp_str);
     }
 }