pub struct TGAImage {
    width: i32,
    height: i32, 
    bytespp: i32,
    data: Vec<u8>,
}

impl TGAImage {

    // Constructor with default values
    pub fn new() -> Self {
        TGAImage {
            width: 0, 
            height: 0, 
            bytespp: 0, 
            data: vec![],
        }
    }

    //Constructor with provided dimensions, no data
    pub fn with_dimensions(width: i32, height: i32, bytespp: i32) -> Self {
        let data_size = (width * height * bytespp) as usize;

        TGAImage {
            width, 
            height, 
            bytespp, 
            data: vec![0; data_size],
        }
    }

    pub fn with_data(width: i32, height: i32, bytespp: i32, data: Vec<u8>) -> Self {
        TGAImage {
            width,
            height,
            bytespp,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
