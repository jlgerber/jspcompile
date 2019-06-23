
#[derive(Debug, PartialEq, Eq)]
pub enum Regex {
    Simple{
        name:String, 
        value: String
    },
    Complex{
        name: String, 
        positive: String, 
        negative: String
    },
}

