# percolation-sim-rs
A percolation simulator written in rust c;

## Usage
percolation-sim-rs is a module for simulating the mathematical/physical phenomena of percolation.

### Building 
To build and run the simulator you can :
```
$ cargo build --release
$ ./target/release/percolation-sim
```
### Writing your own code
Just edit main.rs but leave 
```rs
mod simulator;
use simulator::percolation;
```
at the top of the file.
You can create a new grid struct of height H and width W as follows:
```rs
let mut new_grid = percolation::Grid::new(H, W);
```
Given a probability of a/b for any two nodes to be connected, you can percolate the grid like this:
```rs
new_grid.percolate(a, b, true);
```
or
```rs
new_grid.percolate(a, b, false);
```
There are many methods within the library used for displaying the grid after percolation. But displaying it using the methods which also display color doesn't produce the expected results. As such, it is recommended as of now to use:
```rs
new_grid.display_diagram_no_colors();
```
for display your grids.
