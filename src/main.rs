use std::{iter::Enumerate, ops::Deref};

fn main() {
    println!("-------------------------------------------");
    println!("Multiples-Of-3 or Multiples-Of-5 Calculator");
    println!("\t\t** Menu **");
    println!("-------------------------------------------\n");

    loop {
        println!("Type '1' if you want to calculate the multiples of 3 out of a number\n");
        println!("\t\t\t\t  OR \n");
        println!("Type '2' (or any other character) if you want to calculate the multiples of 5");
        let menu_num = get_user_input_num();
        match menu_num {
            1 => {
                println!("You've picked the Multiples-Of-3 Calculator");
                println!("Please input a number:");
                let num = get_user_input_num();
                multiples_of_3(&num)
            }
            _ => {
                println!("You've picked the Multiples-Of-3 Calculator");
                println!("Please input a number:");
                let num = get_user_input_num();
                multiples_of_5(&num)
            }
        }

        println!("Do you want to continue using the calculator? 1 = Y / 0 = N");
        let continue_num = get_user_input_num();
        match continue_num {
            1 => {
                continue;
            }
            _ => {
                break;
            }
        }
    }
}




fn multiples_of_3(num: &i32) {
    let mut new_vec: Vec<i32> = vec![3; (*num / 3) as usize];
    let test = new_vec.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    let counter = 0;
    let mut multiples_of_3: Vec<i32> = Vec::with_capacity((*num / 3) as usize);
    test.iter().for_each(|x| {
        let num2 = ((x.0 as i32) + 1) * *(x.1);
        multiples_of_3.push(num2);
    });

    println!(
        "The multiples of 3 for the number {} are {:?}",
        *num, multiples_of_3
    )
}

fn multiples_of_5(num: &i32) {
    let mut new_vec: Vec<i32> = vec![5; (*num / 5) as usize];
    let test = new_vec.iter().enumerate().collect::<Vec<(usize, &i32)>>();
    let counter = 0;
    let mut multiples_of_5: Vec<i32> = Vec::with_capacity((*num / 3) as usize);
    test.iter().for_each(|x| {
        let num2 = ((x.0 as i32) + 1) * *(x.1);
        multiples_of_5.push(num2);
    });

    println!(
        "The multiples of 5 for the number {} are {:?}",
        *num, multiples_of_5
    )
}

fn get_user_input_num() -> i32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    n.trim().parse::<i32>().expect("failed parsing in to i32")
}
