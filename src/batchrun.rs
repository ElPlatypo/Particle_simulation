use crate::Plot;
use crate::HexBoard;

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use plotters::prelude::*;

pub struct Batchrun {
    runs_number: i32,
    title: String
}

impl Batchrun {

    //constructor
    pub fn new(number: i32, title: &str) -> Self {
        return Batchrun{runs_number: number, title: title.to_string()};
    }

    //batch runner
    pub fn fill_test(&self, matrix_size: u16, betaj: f64, sim_lenght: i32, min_fill: f64, max_fill: f64) {

        println!("> Beginning fill test");

        let mut rng: StdRng = StdRng::seed_from_u64(3463462432);
        let order_plot = Plot::new(500, 500, 0..sim_lenght, 0.0..3.0);
        let mut orders: Vec<Vec<(i32, f64)>> = vec![];
        let fillpercentage = self.gen_range(self.runs_number as f64, min_fill, max_fill);

        for run in 0..self.runs_number {
            
            let mut board = HexBoard::new(
                matrix_size,
                StdRng::seed_from_u64(rng.gen()),
                betaj,
                fillpercentage[run as usize] as f32,
            );
            let mut order: Vec<(i32, f64)> = vec![];

            board.initialize();

            for x in 0..sim_lenght {
                board.advance_timestep_repulsive();
                order.push((x, board.get_order_single() as f64))
            }
            orders.push(order);
            println!("fill percentage: {:.2}", fillpercentage[run as usize]);
        }
        let mut labels: Vec<String> = Vec::new();
        for value in fillpercentage {
            labels.push(value.to_string());
        }
        order_plot.plot_multiple_timeseries(&(self.title.to_owned() + ".svg"), &self.title, orders, &BLUE, &RED, labels);

    }

    pub fn betaj_test(&self, matrix_size: u16, sim_lenght: i32, min_betaj: f64, max_betaj: f64) {

        println!("> Beginning betaj test");

        let mut rng: StdRng = StdRng::seed_from_u64(3463462432);
        let order_plot = Plot::new(500, 500, 0..sim_lenght, 0.0..3.0);
        let mut orders: Vec<Vec<(i32, f64)>> = vec![];
        let betajvalues = self.gen_range(self.runs_number as f64, min_betaj, max_betaj);

        for run in 0..self.runs_number {
            
            let mut board = HexBoard::new(
                matrix_size,
                StdRng::seed_from_u64(rng.gen()),
                betajvalues[run as usize],
                1.0/4.0,
            );
            let mut order: Vec<(i32, f64)> = vec![];
            println!("betaj: {:.2}", betajvalues[run as usize]);
            board.initialize();

            for x in 0..sim_lenght {
                board.advance_timestep_repulsive();
                order.push((x, board.get_order_single() as f64))
            }
            orders.push(order);
        }
        let mut labels: Vec<String> = Vec::new();
        for value in betajvalues {
            labels.push(value.to_string());
        }
        order_plot.plot_multiple_timeseries(&(self.title.to_owned() + ".svg"), &self.title, orders, &BLUE, &RED, labels);

    }

    pub fn size_test(&self, min_matrix_size: u8, max_matrix_size: u8, sim_lenght: i32, betaj: f64) {

        println!("> Beginning size test");

        let mut rng: StdRng = StdRng::seed_from_u64(3463462432);
        let order_plot = Plot::new(500, 500, 0..sim_lenght, 0.0..3.0);
        let mut orders: Vec<Vec<(i32, f64)>> = vec![];
        let sizes = self.gen_range(self.runs_number as f64, min_matrix_size as f64, max_matrix_size as f64);

        for run in 0..self.runs_number {
            
            let mut board = HexBoard::new(
                sizes[run as usize] as u16,
                StdRng::seed_from_u64(rng.gen()),
                betaj,
                1.0/4.0,
            );
            let mut order: Vec<(i32, f64)> = vec![];
            println!("size: {}", sizes[run as usize]);
            board.initialize();

            for x in 0..sim_lenght {
                board.advance_timestep_repulsive();
                order.push((x, board.get_order_single() as f64))
            }
            orders.push(order);
        }
        let mut labels: Vec<String> = Vec::new();
        for value in sizes {
            labels.push(value.to_string());
        }
        order_plot.plot_multiple_timeseries(&(self.title.to_owned() + ".svg"), &self.title, orders, &BLUE, &RED, labels);

    }

    //function to generate a range of values with equal distance
    pub fn gen_range(&self, step_number: f64, min: f64, max: f64) -> Vec<f64> {
        let step = (max - min) / step_number;
        let mut steps = Vec::with_capacity(step_number as usize);
        for i in 0..step_number as i32 {
            steps.push(min + i as f64 * step);
        }
        return steps;
    }
}