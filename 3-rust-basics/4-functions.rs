
fn word (messege: &str) {
    println!("{}\n",messege);
}
//When a function returns a value, we add the syntax -> <type> 
//after the list of function arguments and before the opening curly bracket for the function body. 
//The arrow syntax -> indicates that the function returns a value to the caller.
// The <type> portion lets the compiler know the data type of the value returned.

fn check_time(num: u32)-> u32 {

    if num>=12 {
        //return early
        return num-12;
    }
    else {
        num
    }
}
// We can use the return keyword at any point in the function to halt execution and send a value back to the caller. 
//Usually, the use of the return keyword is used in combination with a conditional test.

//When you explicitly use the return keyword, you end the statement with a semicolon. 
//If you send back a return value without using the return keyword, you don't end the statement with a semicolon. 
// You might have noticed that we didn't use the ending semicolon for the num

fn main() {

    let time=11;

    println!("The {} in 12 hour format is {}",time,check_time(time));

    if time>=22 || time<=3 {
        word("goodnight");
    }
    else {
        word("Have a nice day");
    }
}