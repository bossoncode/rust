pub fn print(){
    //Print to console
    println!("Hey, babe.");

    //Basic formatting
    println!("Do you have {} GB?", 69);

    println!("{} is a {} yo {}", "Akinkunmi", 21, "developer");

    //Positional arguments
    println!("{0} loves to code and {0} doesn't like {1}", "Akinkunmi", "football");

    //Named arguments
    println!("We all love {programmingLanguage}", programmingLanguage = "JavaScript");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 2, 34, 16);
}