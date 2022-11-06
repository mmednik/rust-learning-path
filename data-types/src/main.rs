// classic struct
struct Student {
    name:String,
    level:u8,
    remote:bool
}

// tuple struct
struct Grades(char, char, char, char, f32);

fn main() {
    let number:u32 = 14;
    println!("The number is {}.", number);

    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    let character_1:char = 'S';
    let character_2:char = 'f';
    let smiley_face = '😃';
    let string_1 = "miley ";
    let string_2:&str = "ace";
    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

    let tuple_e = ('E', 5i32, true);
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    let user_1 = Student {
        name: String::from("Constance Sharma"),
        level: 2,
        remote: true
    };

    let user_2 = Student {
        name: String::from("Dyson Tan"),
        level: 5,
        remote: false
    };

    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}.",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}.",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}
