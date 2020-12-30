#![recursion_limit="512"]

use wasm_bindgen::prelude::*;

use cell::Cell;
use boolinator::Boolinator;
use yew::prelude::*;
//use yew::services::interval::{IntervalService, IntervalTask};
use yew::{Classes,html,Component,ComponentLink,Html,ShouldRender};
//use yew::virtual_dom::*;
use yew::Properties;

mod cell;
use cell::State;

pub enum Msg{
  //  ToggleCellule(usize),
    Solve,
}
#[derive(Debug,Clone)]
pub struct Cellules{
    link:ComponentLink<Self>,
    cellule:Vec<Cell>,
    path:Vec<(usize,usize)>,
    // props:Props,

}

impl Cellules{
    fn view_cellule(&self, x:usize,cell:&Cell) -> Html{

 
        let mut cell_color =  match  cell.state {
                State::yellow => { 'y' },
                State::green => { 'g' },
        };

             // log::info!("{:?}",cell_color);
        html!{
            <div key=x class=format!("game-cellule{}",cell_color)>{ cell.value } </div> 
        }

    }

    fn draw_path(&mut self){
        log::info!("{:?}",self.path);
        for (r,c) in self.path.iter(){
            log::info!("{:?}{:?}",r,c);
            self.cellule[(r*4)+c].set_green();
        //    log::info!("{:?}",self.cellule);
        }
    }
//     /// rat-maze problem solving algorithm wrapper
//     /// this method call easy when clicking
    fn solvewrap(&mut self){
        let row =0;
        let col = 0;
        self.solve(row,col); 
    }

    ///boundary check of the Maze or Grid
    /// this is would be useful in the actual algorithm
    fn outofboundary(&self,row:usize,col:usize) -> bool{
        if row == 3 && col ==3{
            return true
        }

        return false

    }

    ///Actual method where the backtracking algorithm is implemented
    /// this method will be recursively called based the choices
    /// we will be adding the "path" in this algorithm, before calling the method recursively
    /// if we couldn't solve the Maze, pop the added "path" and next choice we have.
    fn solve(&mut self,row:usize,col:usize)-> bool{

        if self.cellule[0].value == 0  
            && row == 0 
            && col == 0{

              log::info!("Rat path would be :");
              log::info!("{:?}",self.path);

              return false
        }else if self.cellule[0].value == 1 
                    && row == 0
                    && col == 0 {
              self.path.push((self.cellule[0].row,self.cellule[0].col));  
        }


        if self.outofboundary(row,col){
              /// print the path found in the browser console  
              log::info!("Rat path would be :");
              log::info!("{:?}",self.path);
              ///clear the path to avoid adding coordinates again 
            //  self.path.clear();
              return true
        }else{
            let mut index = 0;
            if col <  3 {
                    index = row *4 + (col+1);
               if self.cellule[index].value == 1{
            
                  self.path.push((self.cellule[index].row,self.cellule[index].col));
                  if self.solve(row,col+1){
                
                     return true
                    }
                  self.path.pop();
                }
            
            }
            if row < 3 {
                    index = (row + 1) * 4 + col;
                    if self.cellule[index].value == 1{

                        self.path.push((self.cellule[index].row,self.cellule[index].col));
                
                        if self.solve(row+1,col){
                
                            return true
                        }

                        self.path.pop();
                    } 
            }
            else{
                    return false
            }
                
        } 
        

        return false
    }
 }
// #[derive(Clone,Properties)]
// pub struct Props{
//     #[prop_or_default]
//     class:Classes
// }
// impl Default for Color{
//     fn default() -> Self{
//         Color::Yellow
//     }


impl Component for Cellules{

    type Message = Msg;
    type Properties = ();

    fn create(props:Self::Properties,link:ComponentLink<Self>) -> Self{

        Self{
            link,
            cellule:vec![Cell::new_yellow(1,0,0),Cell::new_yellow(1,0,1),Cell::new_yellow(0,0,2),Cell::new_yellow(0,0,3),
            Cell::new_yellow(1,1,0),Cell::new_yellow(0,1,1),Cell::new_yellow(0,1,2), Cell::new_yellow(0,1,3),
            Cell::new_yellow(1,2,0),Cell::new_yellow(1,2,1),Cell::new_yellow(0,2,2),Cell::new_yellow(0,2,3),
            Cell::new_yellow(1,3,0),Cell::new_yellow(1,3,1),Cell::new_yellow(1,3,2),Cell::new_yellow(1,3,3)],
          //  props:props,
            path:Vec::new(),

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::Solve => {
                            self.solvewrap();
                            log::info!("solved");
                            self.draw_path();
                            log::info!("drew path");
                            self.path.clear();
                        }
        }     
                true 
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
       false 
    }

    

    fn view(&self) -> Html{
            let cell_rows = self.cellule.chunks(4)
                                    .enumerate()
                                    .map(|(y,cell_row)| {

                                        let cells = cell_row.iter()
                                                    .enumerate()
                                                    .map(|(x,cell)| {
                                                        self.view_cellule(x,cell)
                                                    }) ;
                                        html!{
                                                <div >{ for cells } </div>
                                          
                                        }
                                        
                                    });
                                    
            html!{
                <div>
                    <div> { for cell_rows }</div>
                    <button onclick=self.link.callback( |_| Msg::Solve)>{"solve"}</button> 
                </div>
            }                        
                                    
                                    
                                    
                                   

         
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {

    ///starting the Yew app
    yew::start_app::<Cellules>();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Click on the solve button to solve rat-maze");
}
