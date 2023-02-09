use std::fmt;

pub struct Elf {
    pub number: i32,
    pub values: Vec<String>,
    pub calories: i32,
}

impl Elf {
    pub fn new(number: i32, values: Vec<String>, calories: i32) -> Self {
        Self {
            values: values,
            calories: calories,
            number: number,
        }
    }
}

impl fmt::Debug for Elf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Elf")
            .field("values", &self.values)
            .finish()
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "Elf: {number} has {calories} calories. ";
        for name in &self.values {
            fmt.write_str(str)?;
            fmt.write_str(name)?;
            str = ", ";
        }
        Ok(())
    }
}
