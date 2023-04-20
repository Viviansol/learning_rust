fn main() {
    println!("Hello, world!");

    let some_string :&str = "fixed length string";

    let mut growable_string:String = String::from("this string will grow");
    println!("the string is: \"{}\"", growable_string);

    //growable_string.push(ch:'s');
    //println!("the string is: \"{}\"", growable_string);

    growable_string.pop();
    println!("the string is: \"{}\"", growable_string);

    let empty_string :String =String::new();

    let concat: String = format!{"{}{}", s_1, s_2};

    let my_information : (&str, i32) = ("salary", 40_000);

    println!("{}, is equeal to {}", my_information.0, my_information.1);

    let (salary : &str, salary_value:i32)= my_information;

    let nested_tuple: (i32, f64, (i32, i32), &str)=(4, 5.0,(3,3), "hello");

    let element :o32 = nested_tuple.2.0;

    let empty_tuple : ()=();
    let mut number_array : [i32;5]=[4,5,6,8,9];
    

}
