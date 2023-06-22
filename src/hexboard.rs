use rand::rngs::StdRng;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct HexBoard {
    pub size: u16,
    rng: StdRng,
    disorder: f64,
    pub grid: Vec<Vec<Hex>>,
    pub fillrate: f32,
}

impl HexBoard {
    pub fn new(size: u16, rng: StdRng, disorder: f64, fillrate: f32) -> Self {
        let mut grid = vec![];
        for x in 0..size {

            let mut gridy: Vec<Hex> = vec![];
            for y in 0..size {
                gridy.push(Hex::new(x as i16, y as i16));
            }
            grid.push(gridy);
        }
        return HexBoard{size, rng, disorder, grid, fillrate}
    }

    //get value at x, y
    pub fn get_cell(&self, x: i16, y:i16) -> Hex {
        return self.grid[x as usize][y as usize];
    }

    //set value at x, y
    pub fn set_cell(&mut self, x: i16, y: i16, value: bool) {
        self.grid[x as usize][y as usize].value = value;
    }


    pub fn initialize(&mut self) {
        for _ in 0..((self.size * self.size) as f32 * self.fillrate) as u16 {
            let mut c: bool = true;
            while c {
                let x = self.rng.gen_range(0..self.size);
                let y = self.rng.gen_range(0..self.size - 1);
                let hex = self.get_cell(x as i16, y as i16);
                if hex.value == false{
                    self.set_cell(x as i16, y as i16, true);
                    c = false;
                }
            }
        }
    }

    pub fn get_neighbours(&self, x: i16, y: i16) -> Vec<Hex> {
        let directions1 = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1], vec![-1, 1], vec![-1, -1]];
        let directions2 = vec![vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1], vec![1, 1], vec![1, -1]];
        let mut n = vec![];
        if y % 2 == 0 {
            for dir in directions1 {
                let newx = self.wrapx(x + dir[0]);
                let newy = self.wrapy(y + dir[1]);
                n.push(self.grid[newx as usize][newy as usize])
            }
        }
        else {
            for dir in directions2 {
                let newx = self.wrapx(x + dir[0]);
                let newy = self.wrapy(y + dir[1]);
                n.push(self.grid[newx as usize][newy as usize])
            }
        }

        return  n;
    }

    pub fn wrapx(&self, index: i16) -> u16 {

        if index < 0 {
            return (self.size as i16 - 1) as u16;
        }

        else if index as u16 >= self.size {
            return 0;
        }

        return index as u16;
    }

    pub fn wrapy(&self, index: i16) -> u16 {

        if index < 0 {
            return (self.size as i16 - 1) as u16;
        }

        else if index as u16 >= self.size - 1 {
            return 0;
        }

        return index as u16;
    }

    pub fn get_energy(&mut self, hex: Hex) -> u8 {
        let mut count: u8 = 0;
        let neighbours = self.get_neighbours(hex.x as i16, hex.y as i16);
        for neighbour in  neighbours{
            if neighbour.value == true {
                count += 1;
            }
        }
        return count;
    }

    pub fn advance_timestep_repulsive(&mut self) {
        for x in 0..self.size{
            for y in 0..self.size {

                let hex = self.get_cell(x as i16, y as i16);
                let start_energy = self.get_energy(hex);
                let ne = self.get_neighbours(x as i16, y as i16);
                let i = self.rng.gen_range(0..ne.len());
                let dest = ne[i];
                if dest.value == false && hex.value == true{
                    
                    let end_energy = (self.get_energy(dest) as i16 - 1) as f64;
                    //move particle according to energy change
                    if start_energy as f64 > end_energy {
                        self.set_cell(x as i16, y as i16, false);
                        self.set_cell(dest.x, dest.y, true);
                    }
                
                    else if start_energy as f64 <= end_energy {
                        let delta: f64 = (start_energy as f64 - end_energy) as f64;
                        let check: bool = self.accept_change(delta);
                        if check == true {
                            self.set_cell(x as i16, y as i16, false);
                            self.set_cell(dest.x, dest.y, true);
                        }
                    }
                }
            }
        }
    }

    //returns the probability of a particle to jump to a neighbouring free cell
    pub fn accept_change(&mut self, delta_energy: f64) -> bool {
        let number: f64 = delta_energy * self.disorder;
        return self.rng.gen_bool(number.exp());
    }

    //returns the total amount of energy in the system (counts multiple times neighbours)
    pub fn total_energy(&self) -> u16 {
        let mut count = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {

                if self.grid[i][j].value == true {
                    let n = self.get_neighbours(i as i16, j as i16);
                    // Check adjacent cells
                    for hex in n {
                        if hex.value == true {
                            count += 1;
                        }
                    }
                }
            }
        }
        return count;
    }

    pub fn get_order(&self) -> Vec<f32> {

        let mut count_a: f32 = 0.0;
        let mut count_b: f32 = 0.0;
        let mut count_c: f32 = 0.0;
        for i in 0..3 {
            for x in 0..self.size {
                for y in 0..self.size {
                    let hex = self.get_cell(x as i16, y as i16);
                    if y % 2 == 0{
                        if i == 0 {
                            if x % 3 == 0 && hex.value == true{
                                count_a += 1.0;
                            }
                        }
                        else if i == 1 {
                            if (x as i16 - 1) % 3 == 0 && hex.value == true{
                                count_b += 1.0;
                            }
                        }
                        else if i == 2 {
                            if (x as i16 - 2) % 3 == 0 && hex.value == true{
                                count_c +=1.0;
                            }
                        }
                    }
                    else if (y - 1) % 2 == 0 {
                        if i == 0 {
                            if (x as i16 - 1) % 3 == 0 && hex.value == true{
                                count_a += 1.0;
                            }
                        }
                        else if i == 1 {
                            if (x as i16 - 2) % 3 == 0 && hex.value == true{
                                count_b += 1.0;
                            }
                        }
                        else if i == 2 {
                            if x % 3 == 0 && hex.value == true{
                                count_c += 1.0;
                            }
                        }
                    }
                }
            }
        }
        let amount = ((self.size * self.size) as f32 * self.fillrate) as u16;
        let order = vec![(count_a as f32 / amount as f32), (count_b / amount as f32), (count_c / amount as f32),];

        return order;
    }

    pub fn get_order_single(&self) -> f32 {

        let mut count_a: f32 = 0.0;
        let mut count_b: f32 = 0.0;
        let mut count_c: f32 = 0.0;
        for i in 0..3 {
            for x in 0..self.size {
                for y in 0..self.size {
                    let hex = self.get_cell(x as i16, y as i16);
                    if y % 2 == 0{
                        if i == 0 {
                            if x % 3 == 0 && hex.value == true{
                                count_a += 1.0;
                            }
                        }
                        else if i == 1 {
                            if (x as i16 - 1) % 3 == 0 && hex.value == true{
                                count_b += 1.0;
                            }
                        }
                        else if i == 2 {
                            if (x as i16 - 2) % 3 == 0 && hex.value == true{
                                count_c +=1.0;
                            }
                        }
                    }
                    else if (y - 1) % 2 == 0 {
                        if i == 0 {
                            if (x as i16 - 1) % 3 == 0 && hex.value == true{
                                count_a += 1.0;
                            }
                        }
                        else if i == 1 {
                            if (x as i16 - 2) % 3 == 0 && hex.value == true{
                                count_b += 1.0;
                            }
                        }
                        else if i == 2 {
                            if x % 3 == 0 && hex.value == true{
                                count_c += 1.0;
                            }
                        }
                    }
                }
            }
        }
        let amount = ((self.size * self.size) as f32 * self.fillrate) as u16;
        let order = (count_a as f32 / amount as f32) + ((count_b as f32 / amount as f32) * 2.0) + ((count_c as f32 / amount as f32) * 3.0); 

        return order;
    }

    pub fn printfile(&mut self, filename: &str) {
        let mut xdata: Vec<u16> = vec![];
        let mut ydata: Vec<u16> = vec![];
        for x in 0..self.size {
            for y in 0..self.size {
                if self.get_cell(x as i16, y as i16).value == true {
                    xdata.push(x);
                    ydata.push(y);
                }
            }
        }
        // Open the file for writing
        let mut file = File::create(filename).expect("Failed to create file");
        // Write the CSV headers
        file.write_all(b"x,y\n").expect("Failed to write headers");
        let s1 = format!("0,{}\n", self.size);
        let s2 = format!("{},0\n", self.size);
        file.write_all(s1.as_bytes()).expect("Failed to write headers");
        file.write_all(s2.as_bytes()).expect("Failed to write headers");
        file.write_all(b"0,0\n").expect("Failed to write headers");
        // Write the data rows
        for (_, (&x, &y)) in xdata.iter().zip(ydata.iter()).enumerate() {
            let shifted_x = if y % 2 == 1 { x as f32 + 0.5 } else { x as f32 };
            let row = format!("{},{}\n", shifted_x, y);
            file.write_all(row.as_bytes()).expect("Failed to write row");
        }
    
        file.flush().expect("Failed to flush file");
    }

}

#[derive(Copy, Clone)]
pub struct Hex {
    pub x: i16,
    pub y: i16,
    pub value: bool,
}

impl Hex {
    //costruttore
    pub fn new(x: i16, y: i16) -> Self {
        let value: bool = false;
        return Hex{x, y, value}
    }
}