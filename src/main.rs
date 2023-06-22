mod plot;
mod batchrun;
mod hexboard;

use hexboard::HexBoard;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::env;
use plot::Plot;
use plotters::prelude::*;

fn main() {
    
    env::set_var("RUST_BACKTRACE", "1");

    //set sim parameters
    let rng = StdRng::seed_from_u64(123067890);

    let mut board = HexBoard::new(30, rng, 1000 as f64, 1.0/3.0);
    board.initialize();

    //run simulation
    let mut energy: Vec<(i32, f64)> = vec![];
    let mut order: Vec<Vec<(i32, f64)>> = vec![vec![], vec![], vec![]];
    let mut order_single: Vec<(i32, f64)> = vec![];
    let mut c = true;
    let mut x = 0;
    while c == true {
        board.advance_timestep_repulsive();
        energy.push((x, board.total_energy() as f64));
        order_single.push((x, board.get_order_single() as f64));
        let orders = board.get_order();
        order[0].push((x, orders[0] as f64));
        order[1].push((x, orders[1] as f64));
        order[2].push((x, orders[2] as f64));
        x += 1;
        if orders[0] > 0.95 || orders[1] > 0.95 || orders[2] > 0.95 {
            c = false;
        }
    }

    //output data
    let energy_plot = Plot::new(500, 500, 0..energy.len() as i32, 0.0..energy[0].1);
    let order_plot = Plot::new(500, 500, 0..order[0].len() as i32, 0.0..1.0);
    let order_single_plot = Plot::new(500, 500, 0..order_single.len() as i32, 0.0..3.0);
    board.printfile("data.csv");

    order_single_plot.plot_timeseries("Order Merged.svg", "Unified order parameter", order_single, &RED, "Parametro d'ordine");
    energy_plot.plot_timeseries("Energy.svg", "Total system energy", energy, &RED, "energy");
    let labels: Vec<String> = vec!["1".to_owned(), "2".to_owned(), "3".to_owned(),];
    order_plot.plot_multiple_timeseries("Order.svg", "Total system order", order, &BLUE, &RED, labels);

    //run batch runs to evaluate order varying different parameters of the simulation
    let fill_batchrun = batchrun::Batchrun::new(5, "Fill variation");
    let betaj_batchrun = batchrun::Batchrun::new(5, "Betaj variation");
    let even_size_batchrun = batchrun::Batchrun::new(5, "Even size variation");
    let odd_size_batchrun = batchrun::Batchrun::new(5, "Odd size variation");
    let small_size_high_betaj_batchrun = batchrun::Batchrun::new(5, "Small size high betaj variation");
    
    fill_batchrun.fill_test(30, 10.0, 3000, 0.3, 0.8);
    betaj_batchrun.betaj_test(30, 3000, 0.5, 5.0);
    even_size_batchrun.size_test(6, 46, 3000, 10.0);
    odd_size_batchrun.size_test(5, 45, 3000, 10.0);
    small_size_high_betaj_batchrun.size_test(4, 14, 300, 4.0);

}
