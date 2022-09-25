use std::io;


fn most_significant_bits(x: usize) -> usize {
    (64 - x.leading_zeros() - 1).try_into().unwrap()
}

fn is_bit_set(val: usize, bit_pos: usize) -> bool {
    let mask = 1 << bit_pos; 
    val & mask == mask
}

#[derive(Debug)]
struct NimMove{
    pile_numb_: usize,
    number_of_stones_: usize,
}

struct Nim{
    pile_num_: usize,
    piles_: Vec<usize>,
}

impl Nim{ //Note to myself: For Grundy numbers later Trait for Normal games
    
    //piles.len() == p
    fn new(p: usize, piles: Vec<usize>) -> Nim {
        if p != piles.len() {panic!("Number of piles and piles sizes are not equal!");}

        Nim{
            pile_num_: p,
            piles_: piles,
        }
    }

    fn nim_sum(&self) -> usize {
        self.piles_.iter().fold(0, |mut acc, x| {acc = acc^x; acc})
    }

    //Return yes if the next player can win
    fn winning_position(&self) -> bool {
        if self.nim_sum() != 0 {return true;}
        false
    }

    //if it is a winning position it returns a winning move
    //otherwise returns a random move
    fn next_move(&self) -> NimMove {
        let nim_s = self.nim_sum();

        if nim_s == 0 { 
            for (ind, v) in self.piles_.iter().enumerate() {
                if *v != 0 {
                    return NimMove{pile_numb_: ind, number_of_stones_: *v};
                }
            }
           /* 
            self.piles_.iter()
                .enumerate()
                .for_each(|(ind, v)| {
                    if is_bit_set(*v, most_significant_bits(nim_s)) {
                        return NimMove{pile_numb_: ind, number_of_stones_: *v-((*v)^nim_s)};
                    }
                    }
                )
            */
        }
        
        for (ind, v) in self.piles_.iter().enumerate() {
            if is_bit_set(*v, most_significant_bits(nim_s)) {
                return NimMove{pile_numb_: ind, number_of_stones_: *v-((*v)^nim_s)};
            }
        }

        return NimMove{pile_numb_: 0, number_of_stones_: 0};
    }

}


///A simple line by line input reader for code shortening
fn read_number() -> usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)
      .expect("Failed to read line");
    input_line.trim().parse()
      .expect("Type in a number or a valid number")
}


fn main() {
    println!("Give me the number of piles!");
    let n = read_number();
    
    let mut piles: Vec<usize> = Vec::new();
    println!("Give me line by line the stones in the piles!");
    for _i in 0..n {
        piles.push(read_number());
    }

    let Test: Nim = Nim::new(n, piles);
    
    println!("Nim sum of the current game: {}", Test.nim_sum());
    println!("The next player can win?: {}", Test.winning_position());
    println!("Current optimal play: {:#?}", Test.next_move());

    //println!("Most significant bit value: {}", most_significant_bits(5));
}

