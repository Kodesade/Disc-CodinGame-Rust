use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let s = input_line.trim_matches('\n').to_string();
    let mut index = 0;
    let mut final_str:Vec<u8> = Vec::new();

    if s.len() < 8{
        println!("NO DATA");
        return;
    }

    loop{
        let mut binary_char = String::new();
        loop{
            
            let indexed_char = s.chars().nth(index).unwrap();
            let indexed_char_next = s.chars().nth(index+1).unwrap();
            
            if indexed_char == indexed_char_next{
                binary_char.push('0');
            }else{
                binary_char.push('1');
            }

            index += 1;
            if index % 8 == 0{
                break;
            }
        }

        let decimal_char:u8 = isize::from_str_radix(&binary_char, 2).unwrap() as u8;
        final_str.push(decimal_char);

        if index+1 == s.len(){
            break;
        }
    }
    println!("{}",String::from_utf8(final_str).unwrap());
}
