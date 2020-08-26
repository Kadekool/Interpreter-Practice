INTEGER, PLUS, EOF, MINUS = 'INTEGER', 'PLUS', 'EOF', 'MINUS'

class Token(object):
    def __init__(self,type,value):
        self.type = type

        self.value = value

    def __str__(self):
        return 'Token({type},{value})'.format(
            type = self.type,
            value = repr(self.value)
        )
    
    def __repr__(self):
        return self.__str__()

class Interpreter(object):
    def __init__(self, text):
        self.text = text

        self.pos = 0

        self.current_token = None
    
    def error(self):
        raise Exception('Error parsing input')

        def advance(self):
        self.pos += 1
        if self.pos > len(self.text) - 1:
            self.current_char = None  
        else:
            self.current_char = self.text[self.pos]
    
    def skip_whitespace(self):
        while self.current_char is not None and self.current_char.isspace():
            self.advance()


    def get_next_token(self):

        text = self.text

        if self.pos >len(text) -1:
            return Token(EOF, None)
        
        current_char = text[self.pos]

        if current_char.isdigit():
            self.advance()
            return Token(INTEGER, int(current_char))
        
        if current_char == '+':
            self.advance()
            return Token(PLUS,current_char)
        
        if current_char == '-':
            self.advance()
            return Token(MINUS,current_char)

        self.error()

        def eat(self, token_type):
            if self.current_token.type == token_type:
                self.current_token = self.get_next_token()
            else:
                self.error()

        def expr(self):
            self.current_token = self.get_next_token()

            left = self.current_token
            self.eat(INTEGER)

            op = self.current_token
            if op.type == PLUS:
                self.eat(PLUS)
            else:
                self.eat(MINUS)

            right = self.current_token
            self.eat(INTEGER)
            if op.type == PLUS:
                result = left.value + right.value
            else:
                result = left.value - right.value
            return result

def main():
    while True:
        try:
            # To run under Python3 replace 'raw_input' call
            # with 'input'
            text = raw_input('calc> ')
        except EOFError:
            break
        if not text:
            continue
        interpreter = Interpreter(text)
        result = interpreter.expr()
        print(result)

if __name__ == '__main__':
    main()