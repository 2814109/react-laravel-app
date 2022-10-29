struct Human {
    pub age: i32,
}

trait GrowOld {
    fn grow_old(&mut self);
}

impl GrowOld for Human {
    fn grow_old(&mut self) {
        self.age += 1;
    }
}

pub fn debug() {
    let mut human = Human { age: 20 };
    human.grow_old();
    println!("{}", human.age);
}

pub fn multiple_println() {
    for count in 0..3 {
        println!("{}. Hello World!", count);
    }
}
