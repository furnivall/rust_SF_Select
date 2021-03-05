use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use ansi_term::Colour::{Green, Red, Cyan, Yellow, Blue, Purple};
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor};
use std::io::stdin;
use std::collections::HashMap;
fn main()-> io::Result<()> {
        
    let mut counter = 0;
    let mut fighters = Vec::<Fighter>::new();

    let file = File::open("FighterData.csv")?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
       //skip header row
        if counter == 0{
            counter+=1;
            continue}
        match line{
        Ok(v)=>{
            let curr = split_into_struct(v);
            fighters.push(curr);
        }
        Err(_)=>()}
    }
    //victory! if the below code runs, it means I have defeated lifetimes.
    let mut fighter_index = 3;
    
    select_screen(&fighters, fighter_index);
    let stdin = stdin();
    for c in stdin.keys(){
         match c.unwrap(){
             Key::Left=>{
                 fighter_index = left(fighter_index);
                 select_screen(&fighters, fighter_index);},
             Key::Right=>{
                 fighter_index = right(fighter_index);
                 select_screen(&fighters, fighter_index);
             },
             Key::Up=>{
                 fighter_index = up(fighter_index);
                 select_screen(&fighters, fighter_index);
             },
             Key::Down=>{
                 fighter_index = down(fighter_index);
                 select_screen(&fighters, fighter_index);
             },
             Key::Ctrl('q')=>break,
                _ => {}
            }
        }
    
   Ok(())
}

pub fn up(mut index:u8)->u8{
    let start_vec:Vec<u8> = (0..14).collect();
    let end_vec:Vec<u8> = vec![11, 12, 10, 0, 1, 13, 2, 3, 4, 5, 6, 7, 8, 9]; 
    let path: HashMap<_,_> = start_vec.iter().zip(end_vec.iter()).collect();
    print!("{:?}", path);
    let output:u8 = match path.get(&index){
        Some(&&num)=> num,
        _=>0,
    };
    output
}

pub fn left(mut index:u8) -> u8{
    if vec![0, 2, 6, 10].contains(&index){
    return index
    }
    index -=1;
    println!("{}", index);
    index
}
pub fn right(mut index:u8) -> u8{
    if vec![1, 5, 9, 13].contains(&index){
        return index
    }
    index += 1;
    println!("{}", index);
    index
}
pub fn down(mut index:u8) -> u8{
    let start_vec:Vec<u8> = (0..14).collect();
    let end_vec:Vec<u8> = vec![3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 2, 0, 1, 5]; 
    let path: HashMap<_,_> = start_vec.iter().zip(end_vec.iter()).collect();
    print!("{:?}", path);
    let output:u8 = match path.get(&index){
        Some(&&num)=> num,
        _=>0,
    };
    output
}
pub fn select_screen(fighters:&Vec<Fighter>, index:u8)-> u8{
    //clear screen and return to top to give illusion of animation
    print!("{}", clear::All);
    print!("{}", cursor::Goto(1,1));
    print!("\n\n");
    for (i, fighter) in fighters.iter().enumerate(){
        //to deal with the two blank spots in top row
        if  vec![0, 2].contains(&i){
            print!("         ");}
        //line breaks
        if vec![2, 6, 10].contains(&i){
            print!("\n");}
        let spaces = " ".repeat(8 - fighter.len());
        print!("|{}", spaces);
        if usize::from(index) == i{
            fighter.print_data(true);
        }
        else{
            fighter.print_data(false);
        }
        //end of lines need pipes too
        if vec![1,5,9,13].contains(&i){
            print!("|");}
    }
    fighters[usize::from(index)].get_info();
    let output:u8 = index+1;
    output
}


pub struct Fighter{
    name: String,
    location: String,
    fightstyle: String,
}
impl Fighter{
    fn print_data(&self, selected:bool){
        if selected{
            print!("{}", Yellow.paint(&self.name))
        }
        else{
            print!("{}", Purple.paint(&self.name))
        }
    }
    fn get_info(&self){
        println!("\n\n\n{} is from  {} with fighting style {}\n\n", 
                 Green.on(Red).underline().paint(&self.name),
                 Cyan.on(Blue).paint(&self.location), 
                 Yellow.underline().paint(&self.fightstyle));
                 }
    fn len(&self)->usize{
        self.name.len()
    }
}

pub fn split_into_struct(line:String)->Fighter
{
    let split_text:Vec<&str> = line.split(',').collect();
    let name = split_text[0].to_string();
    let location = split_text[1].to_string();
    let fightstyle = split_text[2].to_string();
    let fighter_struct = Fighter{name, location, fightstyle};
    fighter_struct
}

