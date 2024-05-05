mod simulator;
use simulator::percolation;

#[allow(unused_imports)]
use rand::prelude::*;

fn main() {
   let mut g = percolation::Grid::new(20, 20);
   g.cli_display_diagram_no_colors();
   g.percolate(10, 100, true);
   println!();
   println!();
   println!();
   println!();
   g.cli_display_diagram_no_colors();
}
