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

}