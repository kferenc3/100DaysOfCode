pub fn area(width: u32, height: u32) -> u32 {  //this function just simply requires 2 numbers. 
                                               //However it is not clear until this that these two are related
    width*height
}

pub fn area_tup(dimensions: (u32, u32)) -> u32 { //this rather requires a tuple with u32 elements, so the w and h are grouped
    dimensions.0 * dimensions.1
}

#[derive(Debug)]  //This is an outer attribute for the struct opting in to debug printing functionality
pub struct Rectangle{
    pub width: u32,
    pub height: u32,
}

impl Rectangle {    //instead of separate functions a method can be implemented on the struct as well.
    pub fn area_method(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool { //methods with multiple arguments. Again only references as we don't need the values, only doing some computation on them.
        self.width > other.width && self.height > other.height
    }
}

pub fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle { //a struct can have multiple impl blocks.
    pub fn square(size: u32) -> Self { //an associated function (i.e in an impl block) without the self
        Self {                         //to call this one would need: let sq = Rectangle::square(3) and 
            width: size,               //receive a rectangle struct that is a square
            height: size,
        }
    }
}