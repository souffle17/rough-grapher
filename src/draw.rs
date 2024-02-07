use std::io::{self, Write};

use crate::{compute::point_check, parser::generate_tree};

pub fn make_graph(input: Vec<String>) {
    let left_side_string = &input[0];
    let right_side_string = &input[1];
    let x_min_in = str::parse::<f64>(&input[2]);
    let x_max_in = str::parse::<f64>(&input[3]);
    let y_min_in = str::parse::<f64>(&input[4]);
    let y_max_in = str::parse::<f64>(&input[5]);
    let x_scale_in = str::parse::<f64>(&input[6]);
    let y_scale_in = str::parse::<f64>(&input[7]);

    
    let x_scale = match x_scale_in {
        Ok(_) => x_scale_in.unwrap(),
        Err(_) => {
            eprintln!("invalid x scale, using default");
            1.0
        },
    };
    
    let y_scale = match y_scale_in {
        Ok(_) => y_scale_in.unwrap(),
        Err(_) => {
            eprintln!("invalid y scale, using default");
            1.0
        },
    };

    let x_min = match x_min_in {
        Ok(_) => (x_min_in.unwrap() / x_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid x min, using default"); 
            -10
        },
    };
    
    let x_max = match x_max_in {
        Ok(_) => (x_max_in.unwrap() / x_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid x max, using default"); 
            10
        },
    };

    let y_min = match y_min_in {
        Ok(_) => (y_min_in.unwrap() / y_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid y min, using default"); 
            -10
        },
    };
    
    let y_max = match y_max_in {
        Ok(_) => (y_max_in.unwrap() / y_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid y max, using default"); 
            10
        },
    };

    let mut graph: Vec<Vec<char>> = Vec::new();

    //coordinate axis
    for i in (y_min..y_max + 1).rev() {
        let mut row: Vec<char> = Vec::new();
        for j in x_min..x_max + 1 {
            if i != 0 {
                if j != 0 {
                    row.push(' ');
                } else {
                    row.push('|');
                }
            } else if j == 0 {
                row.push('+');
            } else {
                row.push('-');
            }
        }
        graph.push(row);
    }

    let left_side = generate_tree(left_side_string);
    let right_side = generate_tree(right_side_string);
    
    if left_side.is_none() || right_side.is_none() {
        if left_side.is_none() {
            eprintln!("first expression couldn't be resolved");
        }
        if right_side.is_none() {
            eprintln!("second expression couldn't be resolved");
        }
    }
    else {
        for i in y_min..(y_max + 1) {
            for j in x_min..(x_max + 1) {
                if point_check(left_side.as_ref(), right_side.as_ref(), j as f64 * x_scale, i as f64 * y_scale,  x_scale, y_scale) {
                    graph[(y_max - i) as usize][(x_max - j) as usize] = '•';
                }
            }

            let mut output: String = String::new();
            for char in &graph[(y_max - i) as usize] {
                output.push(*char);
            }
    
            println!("{}", output);

        }
    }
}

pub fn draw() {
    let mut parameters: Vec<String> = Vec::new();

    let mut input: String = "".to_string();

    print!("first expression: ");
    let _ = io::stdout().flush();
    
    let _ = io::stdin().read_line(&mut input).is_ok();

    parameters.push(input.clone());
    input = "".to_string();

    print!("second expression: ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    parameters.push(input.clone());
    input = "".to_string();

    print!("x-axis minimum (default -10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "-10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("x-axis maximum (default 10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("y-axis minimum (default -10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "-10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("y-axis maximum (default 10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("x scale (default 1.0): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "1.0".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("y scale (default 1.0): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "1.0".to_string();
    }

    parameters.push(input.clone());

    for s in &mut parameters {
        *s = s.trim_end().to_string();
    }

    make_graph(parameters);
}