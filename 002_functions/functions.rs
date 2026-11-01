fn main(){
    //In fn. main you can call all fn.
    
    greeting();// this is how you can call a functoin
    
    //here's arguments
    
    let num1:i8=10;
    let num2:i8=20;
    let result:i8=sum(num1,num2);
    println!("{result}");
    
}

fn greeting(){
    
    println!("Good morning to you man!\nI am xyz \nnice to meet you.");
    
}

//parameters & arguments

fn sum(int1:i8,int2:i8)->i8{// Here's parameters
    return int1+int2;
    
}