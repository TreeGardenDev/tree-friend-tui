use std::io::Read;

#[derive(Debug)]
pub struct TreePresentation{
    tree: Vec<Vec<char>>,
    color: Color,
    color_distribution: colorDistribution,
    seed: u64,
    size: u8,
}
impl TreePresentation{
    pub fn new()->Self{
        let seed=get_seed_from_config();
        Self{
            tree: begin_tree(1),
            color: Color::new(seed),
            color_distribution: colorDistribution::new(seed),
            seed,
            size: 1,
        }
    }
    fn grow_tree(&mut self){
        self.size+=1;
    }

}





enum color_optons{
    red,
    green,
    brown,
}


#[derive(Debug)]
struct colorDistribution{
    percent_red:u8,
    percent_green:u8,
    percent_brown:u8,

}
impl colorDistribution{
    fn new(seed:u64)->Self{
        let red=seed.to_be_bytes()[1];
        let green=seed.to_be_bytes()[2];
        let brown=seed.to_be_bytes()[3];
        let distribution=[red,green,brown];

        let total:u8=red+green+brown;
        println!("{}", total);
        
        println!("red: {}",red);
        let percent_red:u8=red/total*100;


        println!("green: {}",green);
        let percent_green:u8=green/total*100;
        println!("{}",percent_green);
        let percent_brown:u8=brown/total*100;
        Self{
            percent_red,
            percent_green,
            percent_brown,
        }
    }

}

#[derive(Debug)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}
impl Default for Color{
    fn default()->Self{
        Self{
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Color{
    fn new(seed:u64)-> Self{
        let mut colors=Self::default();
        colors.base_color_from_seed(seed);
        colors
    }
    fn base_color_from_seed(&mut self, seed:u64){
       let distrib=seed.to_be_bytes();

        let red:u8=distrib[0];
        let green:u8=distrib[3];
        let blue:u8=distrib[7];
        self.red=red;
        self.green=green;
        self.blue=blue;
    }
}

fn get_seed_from_config()->u64{
    let mut result:u64=0;
    let file_exists=std::fs::metadata("config.txt");
    if file_exists.is_ok(){
        let mut file=std::fs::File::open("config.txt").unwrap();
        let mut contents=String::new();
        file.read_to_string(&mut contents).unwrap();
        contents=contents.replace("Seed: ","");
        contents=contents.replace("\n","");
        println!("{}", contents);
        let seed=contents.parse();
        println!("{:?}", seed);
        result=seed.unwrap()

    }
    result
}

fn begin_tree(size:u8)-> Vec<Vec<char>>{
    let mut tree=Vec::new();
    for _ in 0..size*3{
        let mut row=Vec::new();
        row.push('|');
        row.push(' ');
        row.push(' ');
        row.push(' ');
        row.push(' ');
        row.push(' ');
        row.push('|');
        row.push('\n');
        tree.push(row);
    }
    tree
}
