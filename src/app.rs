use std::{error, time, io::Write};
use crate::tree;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub new: bool,
    tree: Tree,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            tree: Tree::new(),
            new:false
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let app=Self::default();
        println!("{:?}",app.tree.seed);
        println!("{:?}", app.tree.tree);
        //app.seed.write_to_config();
        app 
            
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
    pub fn new_tree(&mut self){
        self.new=true;
        self.tree.seed.write_to_config();
        self.new=false;
    }
}


#[derive(Debug)]
struct Seeding{
    seed: u64,
    system_time: time::SystemTime,
    
}

impl Default for Seeding{
    fn default() -> Self {
        Self{
            seed:0,
            system_time: time::SystemTime::now(),
        }
    }
}
impl Seeding{
    fn new()->Self{
       let mut seed=Self::default();
       seed.break_sys_time();
       seed
    }
    fn break_sys_time(&mut self){
        let time=self.system_time;
        let time=time.duration_since(time::UNIX_EPOCH).unwrap();
        println!("{:?}",time);
        let duration=time.as_millis() as u64;
        println!("{}",duration);
        
        self.seed=duration;
    }
    fn write_to_config(&self){
        let seed=self.seed.to_string();
        let file_exists=std::fs::metadata("config.txt");
        if file_exists.is_err(){
            let mut file=std::fs::File::create("config.txt").unwrap();
            file.write_all("Seed: ".as_bytes()).unwrap();
            file.write_all(seed.as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();
        }
        else{
            std::fs::remove_file("config.txt").unwrap();
            let mut file=std::fs::File::create("config.txt").unwrap();
            file.write_all("Seed: ".as_bytes()).unwrap();
            file.write_all(seed.as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();


            //file.write_all("Seed: ".as_bytes()).unwrap();
            //file.write_all(seed.as_bytes()).unwrap();
        }
    }
}

#[derive(Debug)]
struct Tree {
    tree: tree::TreePresentation,
    growth: u8,
    seed: Seeding,
    type_of_tree: String,
}
impl Default for Tree{
    fn default()-> Self{
        Self{
            tree: tree::TreePresentation::new(),
            growth: 0,
            seed: Seeding::new(),
            type_of_tree: "Oak".to_string(),
        }
    }
}
impl Tree{
    fn new()-> Self{
        Self::default()
    }
}
