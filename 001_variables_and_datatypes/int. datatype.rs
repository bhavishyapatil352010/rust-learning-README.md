fn main() {//fn denotes function.
    println!(" variables and data types");
    //unsigned int.
    let  mut num:u8 =50;// mut helps in make variable mutable.
    
    println!("num variable contains (+) data  :{}",num);
    
    num=166;//first ya have to print first num then (mut) it ; error
    
    println!("num variable (after mut.) contains (+) data  :{}",num);
    //signed int.
    let mut int:i8 = -13;//signed int. type helps to use (-) val.
    
    println!("int variable  contains (-,+) data both : {}",int);
    
    int = 40;
    println!("int variable (after mut.) contains (-,+) data both : {}",int);
}