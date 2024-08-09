use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let database = JiraDatabase::new(String::from("C:\\Users\\joshu\\OneDrive\\Desktop\\Rust_Projects\\bootcamp\\4. Projects\\1. CLI\\Problem\\Stage 3\\Step 2\\data\\db.json"));
    let mut nav = Navigator::new(Rc::new(database));

    loop {
        clearscreen::clear().unwrap();
        let mut page: &Box<dyn ui::Page> ;
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
        if let Some(p) = nav.get_current_page() {
            page = p;
        }
        else {
            break
        }
        match page.draw_page() {
            Ok(page) => {},
            Err(err) => {
                println!("Error rendering page: {}\nPress any key to continue...", err);
                wait_for_key_press();    
            }
        }
        

        let mut input = get_user_input();
        
        //gets rid of \n and \r charachers
        if let Some(first_char) = input.chars().next() {
            input.clear();
            input.push(first_char);
        }

        match page.handle_input(&input) {
            Ok(option) => {
                if let Some(action) = option {
                    match nav.handle_action(action) {
                        Ok(T) => {},
                        Err(err) => {
                            println!("Error processing request: {}\nPress any key to continue...", err);
                            wait_for_key_press();    
                        }
                    }
                }
            },
            Err(err) => {println!("Error parsing input: {}\nPress any key to continue...", err);
                                wait_for_key_press();    
            }
        }
    }   
}