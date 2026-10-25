fn main() {
    
    //&str
    let sentence:&str="hi coders!";
    println!("{sentence}");
    //this type of string are immutable ya cant  change it
    
    //string
    let mut que=String::from("hi coders");
    println!("{que}");
    que.push_str(",whats up");
    println!("{que}");
    // but in this ya can do changes
    
}