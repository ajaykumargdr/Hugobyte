
pub fn add<T:std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

pub fn sub<T:std::ops::Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

pub fn mul<T:std::ops::Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}

pub fn div<T:std::ops::Div<Output = T>>(x: T, y:T) -> T {
    x / y
}

