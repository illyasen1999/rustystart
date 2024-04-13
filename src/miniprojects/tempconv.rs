use std::io;
// use std::process;

struct Temperature {
    value_input: f32,
}

impl Temperature {
    // f -> c = (f - 32.0) * (5.0 / 9.0)
    // "A"
    fn fah_to_cel(&self) -> f32 {
        let res = (self.value_input - 32.0) * (5.0 / 9.0);
        return res;
    }
    // c -> f = c * (9.0 / 5.0) + 32.0
    // "B"
    fn cel_to_fah(&self) -> f32 {
        let res = self.value_input * (9.0 / 5.0) + 32.0;
        return res;
    }
    // c -> k = c + 273.15
    // "C"
    fn cel_to_kel(&self) -> f32 {
        let res = self.value_input + 273.15;
        return res;
    }
    // k -> c = k - 273.15
    // "D"
    fn kel_to_cel(&self) -> f32 {
        let res = self.value_input - 273.15;
        return res;
    }
    // f -> k = (f - 32.0) * (5.0 / 9.0) + 273.15
    // "E"
    fn fah_to_kel(&self) -> f32 {
        let res = (self.value_input - 32.0) * (5.0 / 9.0) + 273.15;
        return res;
    }
    // k -> f = (k - 273.15) * (9.0 / 5.0) + 32.0
    // "F"
    fn kel_to_fah(&self) -> f32 {
        let res = (self.value_input - 273.15) * (9.0 / 5.0) + 32.0;
        return res;
    }
}

pub fn tempconv() {
    println!("Temp Conv App");
    'main_loop: loop {
        let mut user_temp = String::new();

        println!("Enter Temperature: ");

        io::stdin()
            .read_line(&mut user_temp)
            .expect("Not in choices");

        let user_temp: f32 = match user_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let temp = Temperature{ value_input: user_temp };
        
        let mut user_choice = String::new();

        println!("[A]: Cel to Fah, [B]: Fah to Cel, [C]: Cel to Kel, [D]: Kel to Cel, [E]: Fah to Kel, [F]: Kel to Fah, [Q]: Exit the program");
        println!("Enter Choice of conversion: ");

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Not in choices");

        println!("{}", user_choice);

        match user_choice.trim().to_uppercase().as_str() {
            "A" => println!("Fah: {}", &temp.cel_to_fah()),
            "B" => println!("Cel: {}", &temp.fah_to_cel()),

            "C" => println!("Kel: {}", &temp.cel_to_kel()),
            "D" => println!("Cel: {}", &temp.kel_to_cel()),

            "E" => println!("Kel: {}", &temp.fah_to_kel()),
            "F" => println!("Fah: {}", &temp.kel_to_fah()),

            "Q" => {
                println!("Program exited early");
                // process::exit(1);
                break;
            }
            _ => println!("Not in choices"),
        }

        loop {
            println!("Calculate temperature again?(Y/N): ");

            let mut exit_or = String::new();
    
            io::stdin()
                .read_line(&mut exit_or)
                .expect("Not in choices");
    
            let exit_choices = ["Y", "N"];
    
            if !exit_choices.contains(&exit_or.trim().to_uppercase().as_str()) {
                println!("Not in choices");
            }
            else if exit_or.trim().to_uppercase().as_str() == exit_choices[1]{
                println!("Program Exited");
                break 'main_loop;
            }
            else {
                continue 'main_loop;
            }
        }
    }
}

// 1 function for user input maybe???
fn _user_input(){
    // 
}

// Formulas
// f -> c = (f - 32.0) * (5.0 / 9.0)
// c -> f = c * (9.0 / 5.0) + 32.0

// c -> k = c + 273.15
// k -> c = k - 273.15

// f -> k = (f - 32.0) * (5.0 / 9.0) + 273.15
// k -> f = (k - 273.15) * (9.0 / 5.0) + 32.0