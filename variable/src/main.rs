fn main() {
    // validate
    println!("Hello, world!");
    // declare a variable
    let a_num;
    let a_word = "hello";
    // bind a value to the a_var
    a_num = 1;
    println!("the var is {},the word is {}", a_num, a_word);
    str_string_demo();
    tuples_demo();
    structs_demo()
}
fn str_string_demo() {
    // str and String
    println!("the str and String demo");
    let str: &str = "hello str";
    let string: String = "hello string".to_string();
    let string_form: String = String::from("hello string");
    let isequal_string: bool = string == string_form;
    println!("the str is {},the string is {},isequalString {}", str, string, isequal_string);
}
fn tuples_demo() {
    println!("the tuples demo");
    let tuples: (char, u32) = ('a', 10);
    println!("the tuples is {:?}", tuples)
}

fn structs_demo() {
    println!("the struct demo");
    #[derive(Debug)]
    struct Student {
        name: String,
        age: u32,
    }
    let std = Student { name: String::from("li li"), age: 18 };
    println!("the struct is {:?}", std);

    println!(" <struct>.<field> 访问结构中的字段,name is {} ,{}", std.name, std.age);
}
