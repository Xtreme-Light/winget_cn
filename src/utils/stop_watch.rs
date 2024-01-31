use std::time::Instant;



#[derive(Debug)]
pub struct StopWatch{
    name:Option<String>,
    start:Option<Instant>,
}

impl StopWatch{
    pub fn new(name:Option<String>) -> Self{
        Self{
            name,
            start:None,
        }
    }

    pub fn start(&mut self){
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self){
        self.start = None;
    }

    pub fn elapsed(&self) -> Option<u64>{
        match self.start {
            Some(start) => Some(start.elapsed().as_millis() as u64),
            None => None,
        }
    }

    pub fn running<F> (&mut self,f:F) ->&Self
    where F:Fn(){
        self.start();
        f();
        self.stop();
        self
    }

    pub fn pretty(&self) {
        match self.name {
            Some(ref name) => {
                match self.elapsed() {
                    Some(elapsed) => println!("{}",format!("运行 {} 消耗 {}秒",name,elapsed)),
                    None => println!("{}", format!("{} 运行中",name)),
                } 
                  
            },
            None => {
                match self.elapsed() {
                    Some(elapsed) => println!("{}",format!("运行消耗 {} 秒",elapsed)),
                    None => println!(""),
                } 
            },
        }
    }

    pub fn name(&self) -> &Option<String>{
        &self.name
    }
}