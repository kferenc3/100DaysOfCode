use day_seven::my_structs::*;
use day_seven::rectangle::*;

fn main() {
    let mut juzerke = User{
        email: String::from("someone@email.com"),
        username: String::from("john doe"),
        active: true,
        sign_in_count: 1,
    };

    //in case we would like to create a new user, with roughly the same data, then this syntax works as a shortcut:
    let mut juzerke2 = User {
        email: String::from("myemail@email.com"),
        ..juzerke
    };
    //however due to the type of username the data will be moved. Therefore it will be out of scope in juzerke after this. active and sign-in
    //are still valid though as those are stack types, therefore being copied instead of moved

    //By defaults structs own the data they contain. They can be passed references as well, but that will be later covered.

    println!("{}", juzerke.email); //struct values can be accessed with . notation and can be modified like that as well
    juzerke.email = String::from("different@email.com");

    let width1 = 30;
    let height1 = 50;
    println!("The rectangle's are is {}", area(width1,height1)); //area calculation without structs

    let rect1 = (30,50); //Some way this is more consise but the elements can only be accessed by index and easy to mix the 2 up.
    println!("Now the same area, but with a tuple function: {}", area_tup(rect1));

    let rect2 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("Again the same, but with a struct. Readable and compact. Nice! {}", area_struct(&rect2));
    println!("Rectangle is {:#?}", rect2); //{:?} is the Debug output format. See the struct definition how to opt in for this.

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale), //the dbg macro takes ownership, evaluates, prints to stderr and then returns ownership
        height: 50,
    };
    
    dbg!(&rect3); //the dbg macro also prints the file and line number as well along with the values of Rectangle
    println!("Now we are calculating the area with a method: {}", rect3.area_method());

    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq)

}
