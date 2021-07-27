// fn main() {
 
//     let name: String = "Evode".to_string();
//     println!("Hello, {}", name);

// }
//  pub struct Counter{
     
//  }

// create struct
pub struct Counter {
    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
}

// add methods to the struct
impl Counter {
    // returns 8-bit signed integer, must match the type from our struct's 'val' defined above
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    // increment the counter
    pub fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        self.val = self.val + 1;
        let log_message = format!("Increased number to {}", self.val);
        println!("{}", log_message.to_string());
    }

    // decrement (subtract from) the counter
    pub fn decrement(&mut self) {
        // note: see cautionary message in increment()
        self.val = self.val - 1;
        let log_message = format!("Decreased number to {}", self.val);
        println!("{}", log_message.to_string());
    }
}

fn main() {
    let mut counter = Counter {
        val: 0
    };
    counter.increment();
    println!("After incrementing: {}", counter.get_num());
}