fn main() {
    let contents = include_str!("../input.hand");
    println!("{}", process_hands(contents.into()));
}
/**
 *
👉 : moves the memory pointer to the next cell

👈 : moves the memory pointer to the previous cell

👆 : increment the memory cell at the current position

👇 : decreases the memory cell at the current position.

🤜 : if the memory cell at the current position is 0, jump just after the corresponding 🤛

🤛 : if the memory cell at the current position is not 0, jump just after the corresponding 🤜

👊 : Display the current character represented by the ASCII code defined by the current position.
*/

fn process_hands(code: String) -> String {
    let mut result = String::new();
    let mut memory = vec![0u8];
    let instructions: Vec<char> = code.trim().chars().collect();
    let mut memory_index: usize = 0;
    let mut reading_index: usize = 0;
    let mut jump_back_stack: Vec<usize> = Vec::new();
    let mut jump_forward_stack: Vec<usize> = Vec::new();
    let mut looking_for_end_loop = false;
    let mut nested_loops = 0;
    let mut restore_position = 0;

    while reading_index < instructions.len() {
        match instructions.get(reading_index).unwrap() {
            '👉' if !looking_for_end_loop => {
                memory_index += 1;
                if memory.len() == memory_index {
                    memory.push(0u8);
                }
            }
            '👈' if !looking_for_end_loop => {
                memory_index -= 1;
            }
            '👆' if !looking_for_end_loop => {
                let value = memory.get_mut(memory_index).unwrap();
                *value = if *value == 255u8 { 0 } else { *value + 1 };
            }
            '👇' if !looking_for_end_loop => {
                let value = memory.get_mut(memory_index).unwrap();
                *value = if *value == 0u8 { 255 } else { *value - 1 };
            }
            '🤜' if looking_for_end_loop => {
                nested_loops += 1;
            }

            '🤜' if *(memory.get(memory_index).unwrap()) == 0 => match jump_forward_stack.pop() {
                Some(index) => reading_index = index,
                None => {
                    looking_for_end_loop = true;
                    nested_loops = 0;
                    restore_position = reading_index;
                }
            },

            '🤜' => {
                jump_back_stack.push(reading_index);
            }

            '🤛' if looking_for_end_loop => {
                nested_loops -= 1;
                if nested_loops == -1 {
                    jump_forward_stack.push(reading_index);
                    reading_index = restore_position - 1;
                    looking_for_end_loop = false;
                }
            }

            '🤛' if *(memory.get(memory_index).unwrap()) != 0 => {
                reading_index = *(jump_back_stack.get(jump_back_stack.len() - 1).unwrap());
            }

            '🤛' => {
                jump_back_stack.pop();
            }

            '👊' if !looking_for_end_loop => {
                let c = char::from(*(memory.get(memory_index).unwrap()));
                result.push(c);
            }

            _ => {}
        }

        reading_index += 1;
    }

    println!("{}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "👇🤜👇👇👇👇👇👇👇👉👆👈🤛👉👇👊👇🤜👇👉👆👆👆👆👆👈🤛👉👆👆👊👆👆👆👆👆👆👆👊👊👆👆👆👊";
        assert_eq!(process_hands(input.into()), "Hello");
    }

    #[test]
    fn test2() {
        let input = "👉👆👆👆👆👆👆👆👆🤜👇👈👆👆👆👆👆👆👆👆👆👉🤛👈👊👉👉👆👉👇🤜👆🤛👆👆👉👆👆👉👆👆👆🤜👉🤜👇👉👆👆👆👈👈👆👆👆👉🤛👈👈🤛👉👇👇👇👇👇👊👉👇👉👆👆👆👊👊👆👆👆👊👉👇👊👈👈👆🤜👉🤜👆👉👆🤛👉👉🤛👈👇👇👇👇👇👇👇👇👇👇👇👇👇👇👊👉👉👊👆👆👆👊👇👇👇👇👇👇👊👇👇👇👇👇👇👇👇👊👉👆👊👉👆👊";
        assert_eq!(process_hands(input.into()), "Hello World!\n")
    }
}
