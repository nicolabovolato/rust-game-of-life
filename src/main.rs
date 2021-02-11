/*
Rules:
- less than two cells nearby: cell dies
- two or three cells nearby: cell survives
- more than three cells nearby: cell dies
- three cells nearby: a new cell grows
*/

mod game_of_life;

use game_of_life::World;

use std::{ thread, time, env, process };

use num_bigint::BigUint;
use num_traits::Zero;

fn main() {

    let interval = time::Duration::from_millis(500);

    let (seed, world_size) = parse_params_from_args();

    let mut world = World::new(seed, world_size).unwrap();

    while !world.is_stable() {

        print!("\x1B[2J\x1B[1;1H");   //clear screen and potision cursor at 1;1
        print!("{}", world);
        
        world.advance();

        thread::sleep(interval);
    }

    println!("World is stable");
}

fn parse_params_from_args() -> (BigUint, u8) {

    let args: Vec<String> = env::args().collect();
    let mut iter = args.iter().skip(1);

    let mut wrong_usage = false;
    let mut seed: BigUint = Zero::zero();
    let mut world_size: u8 = 3;

    while let Some(arg) = iter.next() {

        match &arg[..] {
            
            "--seed" | "-s" => 
                if let Ok(s) = iter.next().unwrap_or(&" ".to_string()).parse() {
                    seed = s
                }
                else { wrong_usage = true; },

            "--world-size" | "-w" => 
                if let Ok(w) = iter.next().unwrap_or(&" ".to_string()).parse() {
                    world_size = w
                }
                else { wrong_usage = true; },

            "--help" | "-h" => print_help_and_exit(0),

            _ => wrong_usage= true
        }
    }

    if wrong_usage || seed == Zero::zero() { print_help_and_exit(1); }

    (seed, world_size)
}

fn print_help_and_exit(return_code: i32) {
    
    let filename = std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
        .unwrap();
    
    println!("
Conway's Game of life simulation

Usage:
    {0} [OPTIONS] ...

Command Line Arguments:
    -s, --seed <num>        : REQUIRED, The initial value of the world, written as a decimal number
    -w, --world-size <num>  : Side length of the square world (Default 3) 

Example:
    {0} -s 23 -w 4     : Creates a 4 x 4 world with the starting value of 23
    ", filename);

    process::exit(return_code);
}
