use std::fmt;

use num_bigint::BigUint;
use num_traits::One;

pub struct World {
    world_size: u8,
    world: BigUint,
    states: Vec<BigUint>,
    stable: bool
}

impl World {
    const MAX_WORLD_SIZE: u8 = 50;
    const UNDERPOPULATION_TRESHOLD: u8 = 2;
    const OVERPOPULATION_TRESHOLD: u8 = 3;
    const REPRODUCTION_TRIGGER: u8 = 3;

    pub fn new<'a>(seed: BigUint, world_size: u8) -> Result<World, &'a str> {


        if world_size > World::MAX_WORLD_SIZE {
            return Err("World size exceeds maximum allowed.");
        }

        if world_size < 1 {
            return Err("World size must be greater than 0");
        }

        Ok(
            World {
                world: seed.clone(),
                world_size: world_size,
                states: vec![seed],
                stable: false
            }
        )
        
    }

    pub fn is_stable(&self) -> bool {
        self.stable
    }

    pub fn advance(&mut self) {
    
        if self.stable { return; }

        let mut new_world: BigUint = self.world.clone();

        let world_size = self.world_size as u16;
        let one: BigUint = One::one();

        for i in 0..world_size.pow(2) {
    
            let cell_count = self.count_nearby_cells(i);
            let current = self.get_cell(i);          
    
            if current == 1 
                && (cell_count < World::UNDERPOPULATION_TRESHOLD || cell_count > World::OVERPOPULATION_TRESHOLD) {
                new_world = BigUint::from(new_world ^ (&one << i));
            }
            else if current == 0 && cell_count == World::REPRODUCTION_TRIGGER {
                new_world = BigUint::from(new_world | (&one << i));
            }
        }
    
        if (self.states).contains(&new_world) {    
            self.stable = true;
            return; 
        }
    
        self.world = new_world;

        self.states.push(self.world.clone());
    }

    fn count_nearby_cells(&self, index: u16) -> u8 {
    
        let mut cell_count = 0;
        let world_size = self.world_size.into();

        let first_col = index % world_size == 0;
        let last_col = (index + 1) % world_size == 0;
        let first_row = index < world_size;
        let last_row = index > (world_size.pow(2) - world_size);
    
        if !first_col { cell_count += self.get_cell(index - 1); }
        if !last_col  { cell_count += self.get_cell(index + 1); }
    
        if !first_row { 
            cell_count += self.get_cell(index - world_size); 
            if !first_col { cell_count += self.get_cell(index - world_size - 1); }
            if !last_col  { cell_count += self.get_cell(index - world_size + 1); }
        }
    
        if !last_row  { 
            cell_count += self.get_cell(index + world_size); 
            if !first_col { cell_count += self.get_cell(index + world_size - 1); }
            if !last_col  { cell_count += self.get_cell(index + world_size + 1); }
        }

        cell_count
    }    

    fn get_cell(&self, index: u16) -> u8 {
        if ((&self.world & BigUint::from(2u8).pow(index.into())) >> index) == One::one() {1} else {0}
    }   
}

impl fmt::Display for World {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let mut world_str = String::from("");
        let world_size = self.world_size as u16;

        let spacer = "─".repeat(world_size as usize * 2);
    
        let opening_str = format!("┌{}┐\n",spacer);
        let closing_str = format!("└{}┘",spacer);
    
        for i in 0..world_size.pow(2) {
            
            if i % world_size == 0 {
                world_str.push('|');
            }
    
            let current = self.get_cell(i);        
    
            world_str.push_str(if current == 1 {"██"} else {"  "});

            if (i + 1) % world_size == 0 {
                world_str.push('|');
                world_str.push('\n');
            }
        }
    
        writeln!(f,"{}{}{}",opening_str,world_str,closing_str)  
    }
}

#[cfg(test)]
mod tests {

    use num_bigint::BigUint;
    use num_traits::Zero;

    #[test]
    #[should_panic]
    fn new_world_size_greater_than_max() {

        let world_size = super::World::MAX_WORLD_SIZE + 1;

        let _world = super::World::new(Zero::zero(),world_size).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_world_size_lesser_than_one() {

        let world_size = 0;

        let _world = super::World::new(Zero::zero(),world_size).unwrap();
    }

    #[test]
    fn is_stable_returns_correct_value() {

        let mut world = super::World{ world_size: 1, world: Zero::zero(), states: vec![Zero::zero()], stable: false };

        assert_eq!(world.is_stable(), false);

        world.stable = true;

        assert_eq!(world.is_stable(),  true);
    }

    #[test]
    fn get_cell_returns_correct_value() {
        let seed = BigUint::parse_bytes(b"010010001",2).unwrap();
        let world = super::World{ world_size: 3, world: seed.clone(), states: vec![seed], stable: false };

        assert_eq!(world.get_cell(0),1);
        assert_eq!(world.get_cell(1),0);
        assert_eq!(world.get_cell(2),0);
        assert_eq!(world.get_cell(3),0);
        assert_eq!(world.get_cell(4),1);
        assert_eq!(world.get_cell(5),0);
        assert_eq!(world.get_cell(6),0);
        assert_eq!(world.get_cell(7),1);
        assert_eq!(world.get_cell(8),0);
    }

    #[test]
    fn count_nearby_cells_returns_correct_value() {
        
        /*
            1110
            0101
            1100
            1000
        */
        
        let seed = BigUint::parse_bytes(b"0001001110100111",2).unwrap();
        let world = super::World{ world_size: 4, world: seed.clone(), states: vec![seed], stable: false };

        assert_eq!(world.count_nearby_cells(0),  2);
        assert_eq!(world.count_nearby_cells(1),  3);
        assert_eq!(world.count_nearby_cells(2),  3);
        assert_eq!(world.count_nearby_cells(3),  2);
        assert_eq!(world.count_nearby_cells(4),  5);
        assert_eq!(world.count_nearby_cells(5),  5);
        assert_eq!(world.count_nearby_cells(6),  5);
        assert_eq!(world.count_nearby_cells(7),  1);
        assert_eq!(world.count_nearby_cells(8),  3);
        assert_eq!(world.count_nearby_cells(9),  3);
        assert_eq!(world.count_nearby_cells(10), 3);
        assert_eq!(world.count_nearby_cells(11), 1);
        assert_eq!(world.count_nearby_cells(12), 2);
        assert_eq!(world.count_nearby_cells(13), 3);
        assert_eq!(world.count_nearby_cells(14), 1);
        assert_eq!(world.count_nearby_cells(15), 0);
    }

    #[test]
    fn advance_works_correctly() {
        
        /*
            110  ->  110  -> 010 -> 000 -> 000
            010      001     001    011    000
            110      110     010    000    000
        */

        let seed = BigUint::parse_bytes(b"011010011",2).unwrap();
        let mut world = super::World{ world_size: 3, world: seed.clone(), states: vec![seed.clone()], stable: false };

        assert_eq!(world.world, seed);

        world.advance();
        assert_eq!(world.world, BigUint::parse_bytes(b"011100011",2).unwrap());

        world.advance();
        assert_eq!(world.world, BigUint::parse_bytes(b"010100010",2).unwrap());

        world.advance();
        assert_eq!(world.world, BigUint::parse_bytes(b"000110000",2).unwrap());

        world.advance();
        assert_eq!(world.world, Zero::zero());

        world.advance();
        assert_eq!(world.world, Zero::zero());
        assert_eq!(world.stable, true);
    }
}
