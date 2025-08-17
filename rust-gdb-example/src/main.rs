// 定义动物类型枚举
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

// 定义动物结构体
#[derive(Debug)]
struct Animal {
    kind: AnimalType,
    name: String,
    age: u32,
}

impl Animal {
    // 添加方法来使用所有字段
    fn describe(&self) -> String {
        let animal_type = match self.kind {
            AnimalType::Cat => "猫",
            AnimalType::Dog => "狗",
        };
        format!("{}是一只{}，今年{}岁", self.name, animal_type, self.age)
    }
    
    fn is_old(&self) -> bool {
        self.age > 5
    }
}

fn main() {
    let animals: Vec<Animal> = vec![
        Animal {
            kind: AnimalType::Cat,
            name: "Chip".to_string(),
            age: 4,
        },
        Animal {
            kind: AnimalType::Cat, 
            name: "Nacho".to_string(), 
            age: 6
        }, 
        Animal {
            kind: AnimalType::Dog, 
            name: "Taco".to_string(), 
            age: 2
        },
    ];
    
    get_chip(&animals);
    
    // 展示所有动物的信息
    println!("\n所有动物信息:");
    for animal in &animals {
        println!("{}", animal.describe());
        if animal.is_old() {
            println!("  -> {}已经老了", animal.name);
        }
    }
}

fn get_chip(animals: &Vec<Animal>) {
    let chip = animals.get(0);
    println!("chip: {:?}", chip);
}
