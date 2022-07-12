fn main() {
    let x = [0, 0, 1, 2, 3, 4];
    let sum = sum_u32(&x);
    println!("{:?} sum= {:?}", x, sum);

    println!("traffic time{}", TrafficLight::YELLOW.wait());
    println!("Red:{}", aporcess(LightTime::R));
    let cricle = Shape::Circle(Circle { lenght: 10.0 });
    println!("cricle : {}", computes_area(&cricle));
}

fn aporcess<T: Porcess>(v: T) -> String {
    T::porcess(v.as_isize())
}

/*
实现一个打印图形面积的函数， 它接收一个可以计算面积的类型作为参数， 比如圆形，三角形，正方形，需要用到泛型和泛型约束
*/
#[allow(dead_code)]
fn computes_area<T: Area>(t: &T) -> f64 {
    t.computes()
}
trait Area {
    fn computes(&self) -> f64;
}
#[allow(dead_code)]
enum Shape {
    Circle(Circle),
    Triangle(Triangle),
    Square(Square),
}
#[derive(Debug)]
struct Circle {
    lenght: f64,
}
#[derive(Debug)]
struct Triangle {
    length: f64,
    height: f64,
}
#[derive(Debug)]
struct Square {
    lenght: f64,
}

impl Area for Shape {
    fn computes(&self) -> f64 {
        match self {
            Shape::Circle(e) => e.lenght.powi(2) * 2.0 * 3.14,
            Shape::Triangle(e) => e.length * e.height,
            Shape::Square(l) => l.lenght.powi(2),
        }
    }
}

impl Area for Square {
    fn computes(&self) -> f64 {
        self.lenght.powi(2)
    }
}

///
/// 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
///
#[allow(dead_code)]
fn sum_u32(ulist: &[u32]) -> Option<u32> {
    let mut ab = ulist.into_iter();
    if *ab.clone().max().unwrap() == 0 {
        //判断全部为0 返回
        return Some(0);
    }
    let mut next: u32 = 0;
    loop {
        match ab.next() {
            None => break,
            Some(val) => {
                if *val == 0 {
                    continue;
                } else {
                    next = next.wrapping_add(*val);
                    //溢出为0
                }
            }
        }
    }
    //判断是否溢出
    if next == 0 {
        None
    } else {
        Some(next)
    }
}

enum LightTime {
    R = 10,
    Y = 1,
    G = 6,
}

trait Porcess {
    fn as_isize(self) -> isize;
    fn porcess(v: isize) -> String {
        format!("time:{}", v)
    }
}

impl Porcess for LightTime {
    fn as_isize(self) -> isize {
        self as isize
    }
    fn porcess(v: isize) -> String {
        format!("t:{}", v)
    }
}

#[test]
fn test_sum_u32() {
    let a = [0, 0, 8, 5, 6];
    let b = [u32::MIN, 0];
    let c = [u32::MIN, 1, u32::MAX];
    let d = [u32::MIN, u32::MAX];
    assert_eq!(sum_u32(&a), Some(19));
    assert_eq!(sum_u32(&b), Some(0));
    assert_eq!(sum_u32(&c), None);
    assert_eq!(sum_u32(&d), Some(u32::MAX));
}

/// 泛型约束
/// Shape 和 Square 实现了 Area
#[test]
fn test_area_bound() {
    let ab = Shape::Circle(Circle { lenght: 1.0 });
    assert_eq!(6.28, computes_area(&ab));
    let t = Square { lenght: 2.0 };
    assert_eq!(4.0, computes_area(&t));
    //computes_area(&Triangle{length:1.0,height:2.0});
    //the trait `Area` is not implemented for `Triangle`
}


 
#[allow(dead_code)]
enum TrafficLight {
    RED,
    YELLOW,
    GREEN,
}

trait WaitTime {
    fn wait(&self) -> u8;
}
impl WaitTime for TrafficLight {
    fn wait(&self) -> u8 {
        match self {
            Self::RED => 60,
            Self::GREEN => 40,
            Self::YELLOW => 10,
        }
    }
}



/// 为枚举交通信号灯实现一个 trait，
/// trait里包含一个返回时间的方法，不同的灯持续的时间不同
#[test]
fn test_light() {
    let red = TrafficLight::RED;
    assert_eq!(60, red.wait());
    assert_eq!(10, TrafficLight::YELLOW.wait());
    assert_eq!(40, TrafficLight::GREEN.wait());
}
