#[allow(unused_imports)]
use rand::prelude::*;

pub mod percolation {
    
    #[derive(Clone)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8
    }
    
    impl Color {
        pub fn new() -> Self { 
            let r1 = rand::random::<u8>();
            let g1 = rand::random::<u8>();
            let b1 = rand::random::<u8>();
            Self {
                r: r1,
                g: g1,
                b: b1
            }
        }
    
        pub fn max(self) -> &'static str {
            if self.r > self.g && self.r > self.b {
                return "R";
            }
            if self.g > self.r && self.g > self.b {
                return "G";
            }
            return "B";
        }

    }

    pub struct Grid {
        pub nodes: Vec<Vec<(Color, Vec<bool>)>>,
        pub height: u32,
        pub width: u32
    }

    impl Grid {
    
        pub fn new(height: u32, width: u32) -> Self {
            let mut matrix = vec![ vec![
                (Color::new(), vec![false; 4]);
                width as usize];
                height as usize];

            for i in 0..(height as usize) {
                for j in 0..(width as usize) {
                    matrix[i][j].0 = Color::new();
                }
            }

            Self { 
                nodes: matrix,
                height: height,
                width: width
            }
        }

        pub fn cli_display_shortened(&self, height: u32, width: u32) -> () {
            for i in 0..(height as usize) {
                for j in 0..(width as usize) {
                    print!("{}", self.nodes[i][j].0.clone().max());
                }
                println!();
            }
        }

        pub fn cli_display_complete(&self, height: u32, width: u32) -> () {
            for i in 0..(height as usize) {
                for j in 0..(width as usize) {
                    print!("({} {} {})",
                        self.nodes[i][j].0.r,
                        self.nodes[i][j].0.g,
                        self.nodes[i][j].0.b);
                }
                println!();
            }
        }
        
        pub fn cli_display_diagram_complete(&self) {
            for i in 0..(self.height) {
                for j in 0..(self.width) {
                    print!("({} {} {}){}",
                           self.nodes[i as usize][j as usize].0.r,
                           self.nodes[i as usize][j as usize].0.g,
                           self.nodes[i as usize][j as usize].0.b,
                           if self.nodes[i as usize][j as usize].1[1] {"-"} else {" "} );
                }
                if i < self.height - 1 {
                    println!();
                    for h in 0..(self.width) {
                        print!("{}", if self.nodes[i as usize][h as usize].1[2]  {"| "} else {"  "});
                    }
                }
                println!();
            }
        }

        pub fn cli_display_diagram_shortened(&self) {
            for i in 0..(self.height) {
                for j in 0..(self.width) {
                    print!("{}{}", self.nodes[i as usize][j as usize].0.clone().max(),
                           if self.nodes[i as usize][j as usize].1[1] {"-"} else {" "} );
                }
                if i < self.height - 1 {
                    println!();
                    for h in 0..(self.width) {
                        print!("{}", if self.nodes[i as usize][h as usize].1[2] {"| "} else {"  "});
                    }
                }
                println!();
            }
        }

        pub fn cli_display_diagram_no_colors(&self) {
            for i in 0..(self.height) {
                for j in 0..(self.width) {
                    print!("O{}", if self.nodes[i as usize][j as usize].1[1] {"-"} else {" "} );
                }
                if i < self.height - 1 {
                    println!();
                    for h in 0..(self.width) {
                        print!("{}", if self.nodes[i as usize][h as usize].1[2] {"| "} else {"  "});
                    }
                }
                println!();
            }
        }

        // this function uses Lee's algorithm to color the grid properly
        // as in it makes sure that between any two nodes connected by some road
        // they share the same color
        pub fn recolor_whole(&mut self) -> () {

            //we used changed to remember which nodes in the grid we've recolored
            let mut changed: Vec<Vec<bool>> = vec![ vec![ false ; self.width as usize] ; self.height as usize];
            let dirh: Vec<i32> = vec![-1, 0, 1, 0];
            let dirw: Vec<i32> = vec![0, 1, 0, -1];

            struct Position(u32, u32);

            for i in 0..(self.height) {
            
                for j in 0..(self.width) {
                    if changed[i as usize][j as usize] == false {
                    
                        let mut inc = 0;
                        let mut sfa = 1;
                        let the_color: Color = self.nodes[i as usize][j as usize].0.clone();
                        let mut lee_stack: Vec<Position> = vec![];
                        lee_stack.push( Position(i as u32, j as u32) );
                        changed[i as usize][j as usize] = true;

                        while inc < sfa {
                            self.nodes[lee_stack[inc].0 as usize][lee_stack[inc].1 as usize].0 = the_color.clone();
                            for iter in 0..4 {
                                if lee_stack[inc].0 as i32 + dirh[iter] < self.height as i32
                                   && lee_stack[inc].0 as i32 + dirh[iter] >= 0
                                   && lee_stack[inc].1 as i32 + dirw[iter] < self.width as i32 
                                   && lee_stack[inc].1 as i32 + dirw[iter] >= 0 {
                                    if changed[(lee_stack[inc].0 as i32 + dirh[iter]) as usize]
                                              [(lee_stack[inc].1 as i32 + dirw[iter]) as usize] == false
                                       && self.nodes[lee_stack[inc].0 as usize][lee_stack[inc].1 as usize]
                                           .1[iter as usize] == true {
                                        lee_stack
                                            .push( 
                                                Position((lee_stack[inc].0 as i32 + dirh[iter]) as u32,
                                                         (lee_stack[inc].1 as i32 + dirw[iter]) as u32)
                                                );
                                        changed[(lee_stack[inc].0 as i32 + dirh[iter]) as usize]
                                               [(lee_stack[inc].1 as i32 + dirw[iter]) as usize] = true;   
                                        sfa += 1;
                                    }
                                }
                            }
                            inc += 1;
                        }
                    }

                }
            }

        }

        pub fn percolate(&mut self, probability_of: u32, out_of: u32, recolor: bool) -> () {
        
            let dirh: Vec<i32> = vec![0, -1, 0, 1, 0];
            let dirw: Vec<i32> = vec![0, 0, 1, 0, -1];
            let from_neighbor: Vec<u8> = vec![0, 2, 3, 0, 1];
            let mut roll: u32;

            for i in 0..(self.height) {
                for j in 0..(self.width) {
                    for h in 2..4 {
                        roll = rand::random::<u32>();
                        if roll % out_of <= probability_of {
                            self.nodes[i as usize][j as usize].1[h-1 as usize] = true;
                            if i as i32 + dirh[h] >= 0 && i as i32 + dirh[h] < self.height as i32
                               && j as i32 + dirw[h] >= 0 && j as i32 + dirw[h] < self.width as i32 {
                                self.nodes[(i as i32 + dirh[h]) as usize][(j as i32 +dirw[h]) as usize]
                                    .1[from_neighbor[h as usize] as usize] =  true;
                            }
                        }
                    }
                }
            }
            if recolor {
                self.recolor_whole();
            }
        }
        

    }

}


