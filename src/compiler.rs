use std::collections::HashMap;

use crate::chunk::*;
use crate::scanner::*;
use crate::value::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    pub fn next(&self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}

type ParseFn<'sc> = fn(&mut Parser<'sc>) -> ();

struct ParseRule<'sc> {
    prefix: Option<ParseFn<'sc>>,
    infix: Option<ParseFn<'sc>>,
    precedence: Precedence,
}

impl<'sc> ParseRule<'sc> {
    pub fn new(
        prefix: Option<ParseFn<'sc>>,
        infix: Option<ParseFn<'sc>>,
        precedence: Precedence,
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

pub struct Parser<'sc> {
    current: Token<'sc>,
    previous: Token<'sc>,
    had_error: bool,
    panic_mode: bool,

    current_chunk: &'sc mut Chunk,
    scanner: Scanner<'sc>,
    rules: HashMap<TokenType, ParseRule<'sc>>,
}

impl<'sc> Parser<'sc> {
    pub fn new(chunk: &'sc mut Chunk, source: &'sc str) -> Self {
        let mut rules = HashMap::new();

        let mut rule = |kind, prefix, infix, precedence| {
            rules.insert(kind, ParseRule::new(prefix, infix, precedence));
        };

        use Precedence as P;
        use TokenType::*;

        rule(LeftParen, Some(Parser::grouping), None, P::None);
        rule(RightParen, None, None, P::None);
        rule(LeftBrace, None, None, P::None);
        rule(RightBrace, None, None, P::None);
        rule(Comma, None, None, P::None);
        rule(Dot, None, None, P::None);
        rule(Minus, Some(Parser::unary), Some(Parser::binary), P::Term);
        rule(Plus, None, Some(Parser::binary), P::Term);
        rule(Semicolon, None, None, P::None);
        rule(Slash, None, Some(Parser::binary), P::Factor);
        rule(Star, None, Some(Parser::binary), P::Factor);
        rule(Bang, Some(Parser::unary), None, P::None);
        rule(BangEqual, None, Some(Parser::binary), P::Equality);
        rule(Equal, None, None, P::None);
        rule(EqualEqual, None, Some(Parser::binary), P::Equality);
        rule(Greater, None, Some(Parser::binary), P::Comparison);
        rule(GreaterEqual, Some(Parser::binary), None, P::Comparison);
        rule(Less, None, Some(Parser::binary), P::Comparison);
        rule(LessEqual, None, Some(Parser::binary), P::Comparison);
        rule(Identifier, None, None, P::None);
        rule(String, None, None, P::None);
        rule(Number, Some(Parser::number), None, P::None);
        rule(And, None, None, P::None);
        rule(Class, None, None, P::None);
        rule(Else, None, None, P::None);
        rule(False, Some(Parser::literal), None, P::None);
        rule(For, None, None, P::None);
        rule(Fun, None, None, P::None);
        rule(If, None, None, P::None);
        rule(Nil, Some(Parser::literal), None, P::None);
        rule(Or, None, None, P::None);
        rule(Print, None, None, P::None);
        rule(Return, None, None, P::None);
        rule(Super, None, None, P::None);
        rule(This, None, None, P::None);
        rule(True, Some(Parser::literal), None, P::None);
        rule(Var, None, None, P::None);
        rule(While, None, None, P::None);
        rule(Error, None, None, P::None);
        rule(Eof, None, None, P::None);

        Self {
            current: Token::new(),
            previous: Token::new(),
            had_error: false,
            panic_mode: false,
            current_chunk: chunk,
            scanner: Scanner::new(source),
            rules,
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub fn advance(&mut self) -> bool {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if self.current.kind == TokenType::Error {
                self.error_at_current(self.current.lexeme);
            } else {
                break;
            }
        }

        !self.had_error
    }

    pub fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind == kind {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.had_error = true;

        eprint!("[line {}] Error", token.line);
        match token.kind {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => {}
            _ => eprint!(" at '{}'", token.lexeme),
        }
        eprintln!(": {}", message);
    }

    fn emit(&mut self, operator: Op) {
        self.current_chunk
            .add_operator(operator, self.previous.line as u16);
    }

    fn emit_return(&mut self) {
        self.emit(Op::Return);
    }

    fn emit_constant(&mut self, value: Value) {
        let index: usize = self
            .current_chunk
            .add_constant(value, self.previous.line as u16);
        self.emit(Op::Constant(index));
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn number(&mut self) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn literal(&mut self) {
        match self.previous.kind {
            TokenType::False => self.emit(Op::False),
            TokenType::True => self.emit(Op::True),
            TokenType::Nil => self.emit(Op::Nil),
            _ => panic!("Impossible literal type"),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type: TokenType = self.previous.kind;
        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit(Op::Negate),
            TokenType::Bang => self.emit(Op::Not),
            _ => {}
        }
    }

    fn binary(&mut self) {
        let operator_type: TokenType = self.previous.kind;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::Plus => self.emit(Op::Add),
            TokenType::Minus => self.emit(Op::Subtract),
            TokenType::Star => self.emit(Op::Multiply),
            TokenType::Slash => self.emit(Op::Divide),
            TokenType::BangEqual => self.emit(Op::NotEqual),
            TokenType::EqualEqual => self.emit(Op::Equal),
            TokenType::Greater => self.emit(Op::Greater),
            TokenType::GreaterEqual => self.emit(Op::GreaterEqual),
            TokenType::Less => self.emit(Op::Less),
            TokenType::LessEqual => self.emit(Op::LessEqual),
            _ => {}
        }
    }

    fn is_lower_precedence(&mut self, precedence: Precedence) -> bool {
        let current_precedence = self.get_rule(self.current.kind).precedence;
        precedence <= current_precedence
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous.kind).prefix;

        let prefix_rule = match prefix_rule {
            Some(rule) => rule,
            None => {
                self.error("Expect expression.");
                return;
            }
        };
        prefix_rule(self);

        while self.is_lower_precedence(precedence) {
            self.advance();
            let infix_rule = self.get_rule(self.previous.kind).infix.unwrap();
            infix_rule(self);
        }
    }

    fn get_rule(&self, kind: TokenType) -> &ParseRule<'sc> {
        self.rules.get(&kind).clone().unwrap()
    }
}

pub fn compile<'sc>(chunk: &'sc mut Chunk, source: &str) -> bool {
    let mut parser: Parser = Parser::new(chunk, source);
    parser.advance();
    parser.expression();
    parser.consume(TokenType::Eof, "Expect end of expression.");
    parser.end_compiler();
    #[cfg(feature = "debug_print_code")]
    {
        if !parser.had_error {
            crate::debug::disasemble_chunk(parser.current_chunk, "code")
        }
    }
    true
}
