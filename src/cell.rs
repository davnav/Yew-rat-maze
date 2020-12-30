#[derive(Debug,Clone)]
pub enum State{
    yellow,
    green,
}

#[derive(Debug,Clone)]
pub struct Cell{
    pub state:State,
    pub value:u8,
    pub row:usize,
    pub col:usize,

}

impl Cell{
    pub fn new_yellow(value:u8,row:usize,col:usize) -> Self{
        Self{
            state:State::yellow,
            value,
            row,
            col

        }

    }

    pub fn new_green(value:u8,row:usize,col:usize) -> Self{
        Self{
            state:State::green,
            value,
            row,
            col,
        }
    } 

    pub fn set_yellow(&mut self) {
        self.state = State::yellow
    }
    pub fn set_green(&mut self) {
        self.state = State::green
    }

}