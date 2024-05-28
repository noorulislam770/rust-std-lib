
//fn get_sound_(name: &str) -> Result<SoundData, String> {

    //if name == "alert" {
     //   Ok(SoundData::new("alert")),
   // }else {
    //    Err("unable to find sound data".to_owned())
  //  }
//}
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str ) -> Result <MenuChoice, String> {

    match input {

        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Wrong Menu Item Selected".to_owned()),
    }


}


fn print_choice (choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}
fn pick_choice(input: &str ) -> Result<(),String>{
    let choice : MenuChoice= get_choice(input)?;
    print_choice(&choice);
    Ok(())
    
}
fn main () {

  //  let sound = get_sound("alert");
    //match sound {
      //  Ok(_) => println!("sound data located"),
   // Err(e) => println!("error: {:?}",e),
    //}   
    
    let choice : Result<MenuChoice, _> = get_choice("quit");
    match choice {
        Ok(choice) => print_choice(&choice),
        Err(e) => println!("some error occured : {} ", e),
    }
    
    pick_choice("mainmenu");



}
