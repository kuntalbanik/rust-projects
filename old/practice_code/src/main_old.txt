fn main() {
    let coffee_price = 5.99;
    let seasons = ["Spring", "Summer", "Autumn", "Winter"];

    /*




    */
    // Test Display Trait
    println!("{}", 10);
    println!("{}", coffee_price);
    println!("{}", "New line text");
    /*
    


    
    */
    // Test Debug Trait
    println!("{:?}", seasons);
    // using pretty print using  '#' sign
    println!("{:#?}", seasons);
    
    println!("The coffee price is : {:?}", coffee_price);
    /*
    
    
    
    
    */
    {
        let coffee_price = 1.99;
        println!("The coffee price is {coffee_price}");
    }

    /*




    */
    // dbg!(...) Macro
    dbg!(seasons);

    /*




    */
    // Tuple example
    let employee = ("John", 32, "Accounts");
    let (name, age, department) = employee;
    dbg!(employee);
    println!("{name} - {age} - {department}");

    /*




    */
    // Range example
    let month_days = 1..31;

    for day in month_days {
        print!("{day}");
    }
    
    println!("");
    /*




    */

    // Simple "Generic"  example
    let week_days: std::ops::Range<i32> = 1..8;
    for w_days in week_days {
        print!("{w_days}");
    }

}
