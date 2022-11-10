struct Item {
    weight:u32
}

impl Item {
    fn new(weight:u32) -> Self {
        Item {
            weight:weight
        }
    }
}

struct Bin {
    capacity:u32,
    filled:u32,
    itens:Vec<Item>,
}

impl Bin {
    fn new(capacity:u32) -> Self {
        Bin {
            capacity:capacity,
            filled:0,
            itens:Vec::with_capacity(capacity as usize)
        }
    }
    fn fit(&self, item:&Item) -> bool {
        item.weight + self.filled <= self.capacity
    }
    fn add(&mut self, item:&Item) {
        self.itens.push(Item::new(item.weight));
        self.filled += item.weight;
    }
}

fn main() {
    let mut itens:Vec<Item> = Vec::new();
    itens.push(Item::new(5));
    itens.push(Item::new(2));
    itens.push(Item::new(8));
    itens.push(Item::new(1));
    itens.push(Item::new(4));

    let result = first_fit(&itens, 10);

    for bin in &result {
        print!("\nBIN -> ");
        for item in &bin.itens {
            print!("{} ", item.weight);
        }
    }
    println!();

    println!("Bins used {}", result.len());
}

fn first_fit(itens:&Vec<Item>, capacity_of_bin:u32) -> Vec<Bin> {
    let mut bins:Vec<Bin> = Vec::new();
    bins.push(Bin::new(capacity_of_bin));
    
    for item in itens  {
        let mut inserted = false;
        for bin in &mut bins{
            if bin.fit(item) {
                bin.add(item);
                inserted = true;
                break;
            }
        }
        if inserted == false {
            // Create a new bin
            let mut new_bin = Bin::new(capacity_of_bin);
            new_bin.add(item);
            bins.push(new_bin);
        } 
    }

    return bins;
}

fn best_fit(itens:&Vec<Item>, capacity_of_bin:u32) -> Vec<Bin> {
    let mut bins:Vec<Bin> = Vec::new();
    bins.push(Bin::new(capacity_of_bin));
    
    for item in itens  {
        let mut fit_any = false;
        let mut index_bin_to_insert:usize = 0;
        let mut best:u32 = u32::MAX;
        let mut conter = 0;
        for bin in &mut bins {
            if bin.fit(item) {
                fit_any = true;
                if bin.filled + item.weight < best{
                    best = bin.filled + item.weight;
                    index_bin_to_insert = conter;
                }
                break;
            }
            conter += 1;
        }
        if fit_any == false {
            // Create a new bin
            let mut new_bin = Bin::new(capacity_of_bin);
            new_bin.add(item);
            bins.push(new_bin);
        } 
        else {
            bins.get_mut(index_bin_to_insert).unwrap().add(item);
        }
    }

    return bins;
}