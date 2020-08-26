let INTEGER = "INTEGER";
let PLUS = "PLUS";
let EOF = "EOF";
let Minus = "Minus";
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
        self.str();
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

    fn advance(&self){
        self.pos += 1;
        if (self.pos > len(self.text)-1){
            self.current_char = "None";
        }else{
            self.current_char = self.text[self.pos];
        }
    }

    fn skip_whitespace(&self){
        while (self.current_char == " "){
            self.advance();
        }
    }

    fn get_next_token(&self){
        let mut text = self.text;

        if (self.pos > text.len()-1){
            return Token(EOF,"None");
        }

        let mut current_char = text[self.pos];

        if current_char.is_numeric(){
            self.advance();
            return_Token(Token(INTEGER, current_char));
        }

        if current_char == '+'{
            self.advance();
            return_Token(Token(PLUS, current_char));
        }

        if current_char == '-'{
            self.advance();
            return_Token(Token(MINUS, current_char));
        }

        self.error();

    fn return_Token(&self,token: Token) -> Token{
        return token;
    }

    fn eat(&self, token_type: String){
        if (self.current_token.type == token_type){
            self.current_token = self.get_next_token();
        }else{
            self.error()
        }
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
