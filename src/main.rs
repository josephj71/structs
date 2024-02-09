#[derive(Debug)]
struct Car{
    mpg: u32,
    color: String,
    top_speed: u32
}

impl Car {
    fn set_mpg(&mut self, new_mpg: u32){
        self.mpg = new_mpg;
    }

    fn set_color(&mut self, new_color: String){
        self.color = new_color;
    }

    fn set_top_speed(&mut self, new_top_speed: u32){
        self.top_speed = new_top_speed;
    }
}

fn main() {
    let mut car = Car {
                    mpg: 10,
                    color: String::from("blue"),
                    top_speed: 100,
                    };

    car.set_mpg(25);
    car.set_color(String::from("red"));
    car.set_top_speed(250);

    println!("{:?}", car);
}

