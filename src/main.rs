use std::{io::{self, Write}, time::Duration};


const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const CUBE_WIDTH: f64 = 10.0;
const BACKGROUND_ASCII_CODE: char = ' ';

const TEN_MILLIS: Duration = std::time::Duration::from_millis(5);


const INCREMENT_SPEED: f64 = 0.6;
const DISTANCE_FROM_CAM: i32 = 60;
const K1: f64 = 40.0;

struct Values{x: f64, y: f64, z: f64}
impl Values {fn new() -> Self { Values { x: 0.0, y: 0.0, z: 0.0 }}}


struct MoreVals{xp: i32, yp: i32, ooz: f64, idx: i32}
impl MoreVals {fn new() -> Self { MoreVals{xp: 0 , yp: 0, ooz: 0.0, idx: 0}}}

fn calculate_x(i: f64, j: f64, k: f64, a: &f64, b: &f64, c: &f64) -> f64 {
    (j * sin(a) * sin(b) * cos(c))
    -(k * cos(a) * sin(b) * cos(c))
    +(j * cos(a) * sin(c))
    +(k * sin(a) * sin(c))
    +(i * cos(b) * cos(c))
}


fn calculate_y(i: f64, j: f64, k: f64, a: &f64, b: &f64, c: &f64) -> f64 {
    (j * cos(a) * cos(c))
    +(k * sin(a) * cos(c))
    -(j * sin(a) * sin(b) * sin(c))
    +(k * cos(a) * sin(b) * sin(c))
    -(i * cos(b) * sin(c))
}

fn calculate_z(i: f64, j: f64, k: f64, a: &f64, b: &f64, _c: &f64) -> f64 {
    (k * cos(a) * cos(b))
    -(j * sin(a) * cos(b))
    +(i * sin(b))
}

fn sin(x: &f64) -> f64 {return f64::sin(*x)}
fn cos(x: &f64) -> f64 {return f64::cos(*x)}

fn calculate_for_surface(cube_x: f64, cube_y: f64, cube_z: f64, ch: char, values: &mut Values, more_vals: &mut MoreVals, z_buffer: &mut [f64; 44 * 160], buffer: &mut [char; 44 * 160], a: &f64, b: &f64, c: &f64) {
    values.x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    values.y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    values.z = calculate_z(cube_x, cube_y, cube_z, a, b, c) + DISTANCE_FROM_CAM as f64;

    more_vals.ooz = 1.0/(values.z);
    more_vals.xp = (WIDTH as f64/2.0 + K1 * more_vals.ooz * values.x * 2.0) as i32;
    more_vals.yp = (HEIGHT as f64/2.0 + K1 * more_vals.ooz * values.y) as i32;
    more_vals.idx = more_vals.xp + more_vals.yp * WIDTH as i32;

    if more_vals.idx >= 0 && more_vals.idx < (WIDTH * HEIGHT) as i32 {
        if more_vals.ooz > z_buffer[more_vals.idx as usize] {
            z_buffer[more_vals.idx as usize] = more_vals.ooz;
            buffer[more_vals.idx as usize] = ch;
        } 
    }
}

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let c: f64 = 0.0;


    let mut z_buffer: [f64; 44 * 160] = [0.0; 44 * 160];
    let mut buffer: [char; 44 * 160] = [' '; 44 * 160];

    let mut values = Values::new();
    let mut more_vals = MoreVals::new();


    print!("\x1b[2J");
    io::stdout().flush().unwrap();

    loop {
        buffer = [BACKGROUND_ASCII_CODE; WIDTH * HEIGHT];
        z_buffer = [0.0; WIDTH * HEIGHT];

        let mut cube_x = -CUBE_WIDTH;

        while cube_x < CUBE_WIDTH {
            let mut cube_y = -CUBE_WIDTH;

            while cube_y < CUBE_WIDTH {

                calculate_for_surface(cube_x, cube_y, -CUBE_WIDTH, '.', &mut values, &mut more_vals, &mut z_buffer, &mut buffer, &a, &b, &c);
                calculate_for_surface(CUBE_WIDTH, cube_y, cube_x, '$', &mut values, &mut more_vals, &mut z_buffer, &mut buffer, &a, &b, &c);
                calculate_for_surface(-CUBE_WIDTH, cube_y, -cube_x, '~', &mut values, &mut more_vals, &mut z_buffer, &mut buffer, &a, &b, &c);
                calculate_for_surface(-cube_x, cube_y, CUBE_WIDTH, '#', &mut values, &mut more_vals, &mut z_buffer, &mut buffer, &a, &b, &c);
                calculate_for_surface(cube_x, -CUBE_WIDTH, -cube_y, ';', &mut values, &mut more_vals, &mut z_buffer, &mut buffer, &a, &b, &c);
                calculate_for_surface(cube_x, CUBE_WIDTH, cube_y, '+', &mut values, &mut more_vals, &mut z_buffer, &mut buffer, &a, &b, &c);

                cube_y += INCREMENT_SPEED
            }
            cube_x += INCREMENT_SPEED
        }

        print!("\x1b[H");
        let mut k = 0;
    
        while k < (WIDTH * HEIGHT) {
            match k % WIDTH {
                0 => {
                    // io::stdout().flush().unwrap();
                    println!("")
                },
                _ => {
                    print!("{}",buffer[k]);
                }
            }
            k+=1;
        }
    
        a+= 0.005;
        b+= 0.005;
        
        std::thread::sleep(TEN_MILLIS);
    }
}