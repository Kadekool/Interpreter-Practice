let INTEGER = 'INTEGER';
let PLUS = 'PLUS';
let EOF = 'EOF';

struct Token{
    type: String;
    value: String;
}

impl Token{
    fn init(&self,type: &str, value: &i32){
        Token {
            token:token.to_string();
            name:name.to_string();
        }
    }

    fn str(&self) -> String{
        return ('Token({},{})',self.type,self.value);
    }

    fn repr(&self){
        str();
    }

}

struct Interpreter{
    text: String;
    pos: i32;
    current_token: String;
}

impl Interpreter{
    fn init(&self, text:&str){
        Interpreter{
            text:text.to_string();
            pos:0;
            current_token="";
        }
    }

    fn error(self){
        panic!("Error parsing input")
    }
}
