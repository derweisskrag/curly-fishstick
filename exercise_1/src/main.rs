#[derive(Debug, PartialEq)]
struct Student<'a> {
    name: &'a str,
    age: i8,
    grade: Option<Grade>,
}

#[derive(Debug, PartialEq)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl<'a> Student<'a> {
    fn new(name: &'a str, age: i8, points: f32) -> Self {
        Self {
            name,
            age,
            grade: Some(match points {
                91.0..=100.0 => Grade::A,
                81.0..=90.0 => Grade::B,
                71.0..=80.0 => Grade::C,
                61.0..=70.0 => Grade::D,
                51.0..=60.0 => Grade::E,
                _ => Grade::F,
            }),
        }
    }
}

fn main() {
    let alice = Student::new("Alice", 21, 95.2);
    println!("The Alice student got {:?}", alice.grade);
    assert_eq!(alice.grade, Some(Grade::A));
}
