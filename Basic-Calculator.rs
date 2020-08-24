let INTEGER = 'INTEGER';
let PLUS = 'PLUS';
let EOF = 'EOF';

struct Token{
    type: String;
    value: String;
}

impl Token{
    fn init(&self,type: &str, value: &i32) -> Token{
        Token {
            type:type.to_string();
            value:value.to_string();
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
        panic!("Error parsing input");
    }

    fn get_next_token(&self){
        let mut text = self.text;

        if (self.pos > text.len()-1){
            return Token(EOF,"None");
        }

        let mut current_char = text[self.pos];

        if current_char.is_numeric(){
            let token = Token(INTEGER, current_char);
            self.pos += 1;
            return_Token(token);
        }

        if current_char == '+'{
            let token = Token(PLUS, current_char);
            self.pos += 1;
            return_Token(token);
        }

        self.error();

    fn return_Token(&self,token: Token) -> Token{
        return token;
    }

    fn eat(&self, token_type: String){
        self.current_token = self.get_next_token();
    }

    fn expr(&self) ->i32{
        self.current_token = self.get_next_token();

        let left = self.current_token;
        self.eat(INTEGER);

        let op = self.current_token;
        self.eat(PLUS);

        let right = self.current_token;
        self.eat(INTEGER);

        return left.value + right.value;
    }


}

fn main(){
    while (true){
        
    }
}
