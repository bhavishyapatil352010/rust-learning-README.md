fn main(){
    
    // info will be tupple of name and age of person
    
    let info:(&str,u8)=("name",18);
    
    //siple technique
    
    let person_name=info.0;
    let person_age=info.1;
    println!("Person name ={person_name}\nPerson age ={person_age}");
    
    //Destructuring
    
    let (per_name,per_age)=info;
    println!("\nPerson name ={per_name}\nPerson age ={per_age}");
    
}
