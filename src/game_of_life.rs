use std::fmt;

pub struct World {
    world_size: u8,
    world: u128,
    states: Vec<u128>,
    stable: bool
}

impl World {
    const MAX_WORLD_SIZE: u8 = 11;  //World is a 128 bit number, thus 11^2 < 128 < 12^2
    const UNDERPOPULATION_TRESHOLD: u8 = 2;
    const OVERPOPULATION_TRESHOLD: u8 = 3;
    const REPRODUCTION_TRIGGER: u8 = 3;

    pub fn new<'a>(seed: u128, world_size: u8) -> Result<World, &'a str> {


        if world_size > World::MAX_WORLD_SIZE {
            return Err("World size exceeds maximum allowed.");
        }

        if world_size < 1 {
            return Err("World size must be greater than 0");
        }

        Ok(
            World {
                world: seed,
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

        let mut new_world: u128 = self.world.clone();
    
        for i in 0..self.world_size.pow(2) {
    
            let cell_count = self.count_nearby_cells(i);

            let current = self.get_cell(i);          
    
            if current == 1 
                && (cell_count < World::UNDERPOPULATION_TRESHOLD || cell_count > World::OVERPOPULATION_TRESHOLD) {
                new_world = new_world ^ (1 << i);
            }
            else if current == 0 && cell_count == World::REPRODUCTION_TRIGGER {
                new_world = new_world | (1 << i);
            }
        }
    
        if (&self.states).contains(&new_world) {    
            self.stable = true;
            return; 
        }
    
        self.world = new_world;

        self.states.push(self.world);
    }

    fn count_nearby_cells(&self, index: u8) -> u8 {
    
        let mut cell_count = 0;
        
        let first_col = index % self.world_size == 0;
        let last_col = (index + 1) % self.world_size == 0;
        let first_row = index < self.world_size;
        let last_row = index > (self.world_size.pow(2) - self.world_size);
    
        if !first_col { cell_count += self.get_cell(index - 1); }
        if !last_col  { cell_count += self.get_cell(index + 1); }
    
        if !first_row { 
            cell_count += self.get_cell(index - self.world_size); 
            if !first_col { cell_count += self.get_cell(index - self.world_size - 1); }
            if !last_col  { cell_count += self.get_cell(index - self.world_size + 1); }
        }
    
        if !last_row  { 
            cell_count += self.get_cell(index + self.world_size); 
            if !first_col { cell_count += self.get_cell(index + self.world_size - 1); }
            if !last_col  { cell_count += self.get_cell(index + self.world_size + 1); }
        }

        cell_count
    }    

    fn get_cell(&self, index: u8) -> u8 {
        if ((self.world & 2u128.pow(index.into())) >> index) == 1 {1} else {0}
    }   
}

impl fmt::Display for World {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let mut world_str = String::from("");
    
        let spacer = "─".repeat(self.world_size as usize);
    
        let opening_str = format!("┌{}┐\n",spacer);
        let closing_str = format!("└{}┘",spacer);
    
        for i in 0..self.world_size.pow(2) {
            
            if i % self.world_size == 0 {
                world_str.push('|');
            }
    
            let current = self.get_cell(i);        
    
            world_str.push(if current == 1 {'■'} else {' '});

            if (i + 1) % self.world_size == 0 {
                world_str.push('|');
                world_str.push('\n');
            }
        }
    
        writeln!(f,"{}{}{}",opening_str,world_str,closing_str)
    }
}
