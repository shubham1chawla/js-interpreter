use std::fmt::format;

use tree::Tree;
use tokenizer::{SyntaxError, Token, TokenType, Tokenizer};

mod tree;
mod tokenizer;

/**
 * Letter parser: recursive decent parser implementation
 */
pub struct Parser {
    tokenizer: Tokenizer,
    lookahead: Token,
}

impl Parser {

    /**
     * Creates a new parser instance with code content as string.
     */
    pub fn new(content_string: String) -> Result<Self, SyntaxError> {
        let mut tokenizer = Tokenizer::new(content_string);

        // Prime the tokenizer to obtain the first token
        // which is our lookahead. The lookahead is used
        // for predictive parsing.
        let lookahead = tokenizer.get_next_token()?;

        Ok(Self {
            tokenizer,
            lookahead,
        })
    }

    /**
     * Eats a token and advances the lookahead token.
     * Throws a Syntax error if lookahead doesn't match supplied token.
     */
    fn eat(&mut self, token_type: TokenType) -> Result<Token, SyntaxError> {
        if self.lookahead.token_type != token_type {
            return Err(SyntaxError {
                message: format(format_args!("Unexpected token {:?}, expected {:?}!", self.lookahead.token_type, token_type)),
            });
        }
        
        // Advance to the next token.
        let token = self.lookahead.clone();
        self.lookahead = self.tokenizer.get_next_token()?;
        Ok(token)
    }

    /**
     * Parses a string into an AST.
     */
    pub fn parse(&mut self) -> Result<Tree, SyntaxError> {
        self.program()
    }

    /**
     * This function is the main entry point of the parser.
     * 
     * Program
     *  : StatementList
     *  ;
     */
    fn program(&mut self) -> Result<Tree, SyntaxError> {
        let statement_list = self.statement_list(TokenType::EOF)?;
        return Ok(Tree::Program { body: Box::new(statement_list) });
    }

    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_lookahead_type: TokenType) -> Result<Vec<Tree>, SyntaxError> {
        let mut statement_list = vec![];

        while self.lookahead.token_type != stop_lookahead_type {
            statement_list.push(self.statement()?);
        }

        Ok(statement_list)
    }

    /**
     * Statement
     *  : EmptyStatement
     *  | BlockStatement
     *  | VariableStatement
     *  | IfStatement
     *  | ExpressionStatement
     *  ;
     */
    fn statement(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::SemiColon => self.empty_statement(),
            TokenType::CurlyBracketOpen => self.block_statement(),
            TokenType::LetKeyword => self.variable_statement(),
            TokenType::IfKeyword => self.if_statement(),
            _ => self.expression_statement(),
        }
    }

    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::EmptyStatement)
    }

    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::CurlyBracketOpen)?;
        let body = match self.lookahead.token_type {
            TokenType::CurlyBracketClose => vec![],
            _ => self.statement_list(TokenType::CurlyBracketClose)?,
        };
        self.eat(TokenType::CurlyBracketClose)?;
        Ok(Tree::BlockStatement { body: Box::new(body) })
    }

    /**
     * VariableStatement
     *  : 'let' VariableDeclarationList ';'
     *  ;
     */
    fn variable_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::LetKeyword)?;
        let declarations = self.variable_declaration_list()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::VariableStatement { declarations: Box::new(declarations) })
    }

    /**
     * VariableDeclarationList
     *  : VariableDeclaration
     *  | VariableDeclarationList ',' VariableDeclaration
     *  ;
     */
    fn variable_declaration_list(&mut self) -> Result<Vec<Tree>, SyntaxError> {
        let mut declarations = vec![self.variable_declaration()?];

        while self.lookahead.token_type == TokenType::Comma {
            self.eat(TokenType::Comma)?;
            declarations.push(self.variable_declaration()?);
        }

        Ok(declarations)
    }

    /**
     * VariableDeclaration
     *  : Identifier OptVariableInitializer
     *  ;
     */
    fn variable_declaration(&mut self) -> Result<Tree, SyntaxError> {
        let identifier = self.identifier()?;

        // OptVariableInitializer
        let init = match self.lookahead.token_type {
            TokenType::Comma | TokenType::SemiColon => None,
            _ => Some(self.variable_initializer()?),
        };

        Ok(Tree::VariableDeclaration { 
            identifier: Box::new(identifier), 
            init: Box::new(init),
        })
    }

    /**
     * VariableInitializer
     *  : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn variable_initializer(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::SimpleAssignmentOperator)?;
        self.assignment_expression()
    }

    /**
     * IfStatement
     *  : 'if' '(' Expression ')' Statement
     *  | 'if' '(' Expression ')' Statement 'else' Statement
     *  ;
     */
    fn if_statement(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::IfKeyword)?;

        self.eat(TokenType::CircleBracketOpen)?;
        let test = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;

        let consequent = self.statement()?;
        let alternate = match self.lookahead.token_type {
            TokenType::ElseKeyword => {
                self.eat(TokenType::ElseKeyword)?;
                Some(self.statement()?)
            },
            _ => None,
        };

        Ok(Tree::IfStatement {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: Box::new(alternate),
        })
    }

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Result<Tree, SyntaxError> {
        let expression = self.expression()?;
        self.eat(TokenType::SemiColon)?;
        Ok(Tree::ExpressionStatement { expression: Box::new(expression) })
    }

    /**
     * Expression
     *  : AssignmentExpression
     *  ;
     */
    fn expression(&mut self) -> Result<Tree, SyntaxError> {
        self.assignment_expression()
    }

    /**
     * AssignmentExpression
     *  : RelationalExpression
     *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn assignment_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.relational_expression()?;

        // Checking if the lookahead token is not of assignment type, then its an AdditiveExpression
        if !self.is_assignment_operator() {
            return Ok(left);
        }

        // Consuming assignment operator
        let operator = self.assignment_operator()?.value;

        // Checking if the left hand side expression is valid, aka an identifier
        left = self.check_valid_assignment_target(left)?;

        // Right-recursing to create the AssignmentExpression
        let right = self.assignment_expression()?;

        Ok(Tree::AssignmentExpression { 
            operator, 
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    /**
     * Whether the token is an assignment operator.
     */
    fn is_assignment_operator(&self) -> bool {
        match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator | TokenType::ComplexAssignmentOperator => true,
            _ => false,
        }
    }

    /**
     * Extra check whether it's valid assignment target.
     */
    fn check_valid_assignment_target(&mut self, node: Tree) -> Result<Tree, SyntaxError> {
        if let Tree::Identifier {..} = node {
            return Ok(node);
        }
        Err(SyntaxError {
            message: String::from("Invalid left-hand side in assignment expression, expected Identifier!"),
        })
    }

    /**
     * AssignmentOperator
     *  : SIMPLE_ASSIGNMENT_OPERATOR
     *  | COMPLEX_ASSIGNMENT_OPERATOR
     *  ;
     */
    fn assignment_operator(&mut self) -> Result<Token, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator => self.eat(TokenType::SimpleAssignmentOperator),
            _ => self.eat(TokenType::ComplexAssignmentOperator),
        }
    }

    /**
     * RelationalExpression
     *  : AdditiveExpression
     *  | AdditiveExpression RELATIONAL_OPERATOR RelationalExpression
     *  ;
     * 
     * NOTE: Since AdditiveExpression has higher presidence over RelationalExpression
     * the left and right sub-tree of RelationalExpression looks for a AdditiveExpression.
     */
    fn relational_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.additive_expression()?;

        while self.lookahead.token_type == TokenType::RelationalOperator {
            // Operator: <, >, <=, >=
            let operator = self.eat(TokenType::RelationalOperator)?.value;

            // Extracting the right literal
            let right = self.additive_expression()?;
            
            // Enforcing left associativity
            left = Tree::BinaryExpression { 
                operator, 
                left: Box::new(left), 
                right: Box::new(right), 
            };
        }
        Ok(left)
    }

    /**
     * AdditiveExpression
     *  : MultiplicativeExpression
     *  | AdditiveExpression ADDITIVE_OPERATOR MultiplicativeExpression
     *  ;
     * 
     * NOTE: Since MultiplicativeExpression has higher presidence over AdditiveExpression
     * the left and right sub-tree of AdditiveExpression looks for a MultiplicativeExpression.
     */
    fn additive_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.multiplicative_expression()?;

        while self.lookahead.token_type == TokenType::AdditiveOperator {
            // Operator: +, -
            let operator = self.eat(TokenType::AdditiveOperator)?.value;

            // Extracting the right literal
            let right = self.multiplicative_expression()?;
            
            // Enforcing left associativity
            left = Tree::BinaryExpression { 
                operator, 
                left: Box::new(left), 
                right: Box::new(right), 
            };
        }
        Ok(left)
    }

    /**
     * MultiplicativeExpression
     *  : PrimaryExpression
     *  | MultiplicativeExpression MULTIPLICATIVE_OPERATOR PrimaryExpression
     *  ;
     * NOTE: Since PrimaryExpression has higher presidence over MultiplicativeExpression
     * the left and right sub-tree of MultiplicativeExpression looks for a PrimaryExpression.
     */
    fn multiplicative_expression(&mut self) -> Result<Tree, SyntaxError> {
        let mut left = self.primary_expression()?;

        while self.lookahead.token_type == TokenType::MultiplicativeOperator {
            // Operator: *, /
            let operator = self.eat(TokenType::MultiplicativeOperator)?.value;

            // Extracting the right literal
            let right = self.primary_expression()?;
            
            // Enforcing left associativity
            left = Tree::BinaryExpression { 
                operator, 
                left: Box::new(left), 
                right: Box::new(right), 
            };
        }
        Ok(left)
    }

    /**
     * PrimaryExpression
     *  : Literal
     *  | ParanthesizedExpression
     *  | LeftHandSideExpression
     *  ;
     */
    fn primary_expression(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::Number | TokenType::String => self.literal(),
            TokenType::CircleBracketOpen => self.paranthesized_expression(),
            _ => self.left_hand_side_expression(),
        }
    }

    /**
     * ParanthesizedExpression
     *  : '(' Expression ')'
     *  ;
     */
    fn paranthesized_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::CircleBracketOpen)?;
        let expression = self.expression()?;
        self.eat(TokenType::CircleBracketClose)?;
        Ok(expression)
    } 

    /**
     * LeftHandSideExpression
     *  : Identifier
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.identifier()
    }

    /**
     * Identifier
     *  : IDENTIFIER
     *  ;
     */
    fn identifier(&mut self) -> Result<Tree, SyntaxError> {
        let name = self.eat(TokenType::Identifier)?.value;
        Ok(Tree::Identifier { name })
    }

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> Result<Tree, SyntaxError> {
        match self.lookahead.token_type {
            TokenType::Number => self.numeric_literal(),
            _ => self.string_literal(),
        }
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = self.eat(TokenType::Number)?;
        match token.value.parse() {
            Err(_) => Err(SyntaxError {
                message: String::from("Expected a parsable numeric value!"),
            }),
            Ok(parsed) => Ok(Tree::NumericLiteral { value: parsed })
        }
    }

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Result<Tree, SyntaxError> {
        let token = self.eat(TokenType::String)?;

        // Removing quotes from start and end
        let value = String::from(&token.value[1..(token.value.len()-1)]);
        return Ok(Tree::StringLiteral { value })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tree(expected: Tree, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_ok());
        assert_eq!(expected, tree_result.unwrap());
    }

    fn assert_syntax_error(expected: SyntaxError, content_string: &str) {
        let parser_result = Parser::new(content_string.to_owned());
        assert!(parser_result.is_ok());
        let mut parser = parser_result.unwrap();
        let tree_result = parser.parse();
        assert!(tree_result.is_err());
        assert_eq!(expected, tree_result.unwrap_err());
    }

    // ----- TESTS FOR COMMENTS ----- //

    #[test]
    fn test_parse_single_line_comments() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "// Comment \n 42;");
    }

    #[test]
    fn test_parse_multi_line_comments() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        let content_string = "\
        /* Multi-line comment \n\
        * Hello
        */
        \"Hello\";";
        assert_tree(expected, content_string);
    }

    // ----- TESTS FOR LITERALS ----- //

    #[test]
    fn test_parse_literal_numeric() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "42;");
    }

    #[test]
    fn test_parse_literal_string() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "\"Hello\";");
    }

    #[test]
    fn test_parse_missing_semicolon() {
        let expected = SyntaxError {
            message: String::from("Unexpected token EOF, expected SemiColon!"),
        };
        assert_syntax_error(expected, "42");
    }

    // ----- TESTS FOR WHITESPACES ----- //

    #[test]
    fn test_parse_literal_numeric_with_whitespaces() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                }
            ]), 
        };
        assert_tree(expected, "    42;");
    }

    #[test]
    fn test_parse_literal_string_with_whitespaces() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "  Hello, World!  ".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "  \"  Hello, World!  \";  ");
    }

    // ----- TESTS FOR EMPTY FILE ----- //

    #[test]
    fn test_parse_empty_content() {
        let expected = Tree::Program { body: Box::new(vec![]) };
        assert_tree(expected, "");
    }

    // ----- TESTS FOR STATEMENT LIST ----- //

    #[test]
    fn test_parse_statement_list() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, "42;\"Hello\";");
    }

    // ----- TESTS FOR BLOCK STATEMENT ----- //

    #[test]
    fn test_parse_empty_block() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::BlockStatement { body: Box::new(vec![]) }
            ]), 
        };
        assert_tree(expected, "{}");
    }

    #[test]
    fn test_parse_block_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::BlockStatement { 
                    body: Box::new(vec![
                        Tree::ExpressionStatement { 
                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                        }
                    ]) 
                },
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]), 
        };
        assert_tree(expected, "42; { //Commenting 42 -> 42;\n 'Hello'; } \"Hello\";");
    }

    #[test]
    fn test_parse_nested_block_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::BlockStatement { 
                    body: Box::new(vec![
                        Tree::BlockStatement { 
                            body: Box::new(vec![
                                Tree::ExpressionStatement { 
                                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                                },
                                Tree::BlockStatement { 
                                    body: Box::new(vec![
                                        Tree::ExpressionStatement { 
                                            expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                                        }
                                    ]) 
                                },
                            ]) 
                        },
                        Tree::BlockStatement { 
                            body: Box::new(vec![]) 
                        },
                    ]) 
                },
            ]), 
        };
        assert_tree(expected, "{{ 42; { 'Hello'; } }{}}");
    }

    #[test]
    fn test_parse_invalid_block_statement() {
        let expected = SyntaxError {
            message: String::from("Unexpected token EOF, expected Identifier!"),
        };
        assert_syntax_error(expected, "{");
    }

    // ----- TESTS FOR EMPTY STATEMENT ----- //

    #[test]
    fn test_parse_simple_empty_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
            ]),
        };
        assert_tree(expected, ";");
    }

    #[test]
    fn test_parse_empty_statements() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::NumericLiteral { value: 42.0 } ),
                },
                Tree::EmptyStatement,
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::StringLiteral { value: "Hello".to_owned() } ),
                }
            ]),
        };
        assert_tree(expected, ";\n42;\n;\n'Hello';");
    }

    // ----- TESTS FOR BINARY EXPRESSIONS ----- //

    #[test]
    fn test_parse_additive_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("+"), 
                        left: Box::new(Tree::BinaryExpression { 
                            operator: String::from("-"), 
                            left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                }
            ]), 
        };
        assert_tree(expected, "3 - 2 + 1;");
    }

    #[test]
    fn test_parse_multiplicative_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("*"), 
                        left: Box::new(Tree::BinaryExpression { 
                            operator: String::from("/"), 
                            left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                }
            ]), 
        };
        assert_tree(expected, "3 / 2 * 1;");
    }

    #[test]
    fn test_parse_multiplicative_additive_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("+"), 
                        left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                        right: Box::new(Tree::BinaryExpression { 
                            operator: String::from("/"), 
                            left: Box::new(Tree::NumericLiteral { value: 2.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 1.0 }), 
                        })
                    }),
                }
            ]), 
        };
        assert_tree(expected, "3 + 2 / 1;");
    }

    #[test]
    fn test_parse_paranthesized_binary_expressions() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::BinaryExpression { 
                        operator: String::from("/"), 
                        left: Box::new(Tree::BinaryExpression { 
                            operator: String::from("+"), 
                            left: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            right: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                }
            ]), 
        };
        assert_tree(expected, "(3 + 2) / 1;");
    }

    // ----- TESTS FOR ASSIGNMENT EXPRESSIONS ----- //

    #[test]
    fn test_parse_simple_assignment_expression_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num = 42;");
    }

    #[test]
    fn test_parse_simple_assignment_expression_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("str") }), 
                        right: Box::new(Tree::StringLiteral { value: String::from("Hello, World!") }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "str = 'Hello, World!';");
    }

    #[test]
    fn test_parse_simple_assignment_expression_3() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("xyz") }), 
                        right: Box::new(Tree::BinaryExpression { 
                            operator: String::from("+"), 
                            left: Box::new(Tree::NumericLiteral { value: 2.0 }), 
                            right: Box::new(Tree::NumericLiteral { value: 3.0 }),
                        }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "xyz = 2 + 3;");
    }

    #[test]
    fn test_parse_chained_assignment_expression() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("="), 
                        left: Box::new(Tree::Identifier { name: String::from("x") }), 
                        right: Box::new(Tree::AssignmentExpression { 
                            operator: String::from("="), 
                            left: Box::new(Tree::Identifier { name: String::from("y") }), 
                            right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                        }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "x = y = 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("+="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num += 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("-="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num -= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_3() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("*="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num *= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_expression_4() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::ExpressionStatement { 
                    expression: Box::new(Tree::AssignmentExpression { 
                        operator: String::from("/="), 
                        left: Box::new(Tree::Identifier { name: String::from("num") }), 
                        right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                    }), 
                },
            ]),
        };
        assert_tree(expected, "num /= 42;");
    }

    #[test]
    fn test_parse_invalid_assignment_expression() {
        let expected = SyntaxError {
            message: String::from("Invalid left-hand side in assignment expression, expected Identifier!"),
        };
        assert_syntax_error(expected, "42 = 42;");
    }

    // ----- TESTS FOR VARIABLE STATEMENT ----- //

    #[test]
    fn test_parse_simple_no_init_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("y") }),
                            init: Box::new(None),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let y;");
    }

    #[test]
    fn test_parse_simple_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("str") }),
                            init: Box::new(Some(Tree::StringLiteral { value: String::from("Hello") })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let str = 'Hello';");
    }

    #[test]
    fn test_parse_multiple_no_init_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("a") }),
                            init: Box::new(None),
                        },
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("b") }),
                            init: Box::new(None),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let a, b;");
    }

    #[test]
    fn test_parse_multiple_variable_statement() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("c") }),
                            init: Box::new(None),
                        },
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("d") }),
                            init: Box::new(Some(Tree::NumericLiteral { value: 42.0 })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let c, d = 42;");
    }

    #[test]
    fn test_parse_chained_variable_statement_1() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                            init: Box::new(Some(Tree::AssignmentExpression {
                                operator: String::from("="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let x = y = 42;");
    }

    #[test]
    fn test_parse_chained_variable_statement_2() {
        let expected = Tree::Program { 
            body: Box::new(vec![
                Tree::VariableStatement { 
                    declarations: Box::new(vec![
                        Tree::VariableDeclaration {
                            identifier: Box::new(Tree::Identifier { name: String::from("x") }),
                            init: Box::new(Some(Tree::AssignmentExpression {
                                operator: String::from("-="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 }),
                            })),
                        },
                    ]),
                },
            ]),
        };
        assert_tree(expected, "let x = y -= 42;");
    }

    #[test]
    fn test_parse_complex_assignment_operator_variable_statement() {
        let expected = SyntaxError {
            message: String::from("Unexpected token ComplexAssignmentOperator, expected SimpleAssignmentOperator!"),
        };
        assert_syntax_error(expected, "let x *= 42;");
    }

    #[test]
    fn test_parse_literal_eq_literal_variable_statement() {
        let expected = SyntaxError {
            message: String::from("Unexpected token Number, expected Identifier!"),
        };
        assert_syntax_error(expected, "let 42 = 42;");
    }

    // ----- TESTS FOR IF STATEMENT ----- //

    #[test]
    fn test_parse_no_alternate_simple_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::BlockStatement { body: Box::new(vec![]) }),
                    alternate: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "if (x) {}");
    }

    #[test]
    fn test_parse_simple_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("+="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 1.0 })
                        }),
                    }),
                    alternate: Box::new(Some(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 42.0 })
                        }),
                    })),
                },
            ]),
        };
        assert_tree(expected, "if (x) x += 1; else x = 42;");
    }

    #[test]
    fn test_parse_chained_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::IfStatement {
                        test: Box::new(Tree::Identifier { name: String::from("y") }),
                        consequent: Box::new(Tree::ExpressionStatement {
                            expression: Box::new(Tree::AssignmentExpression {
                                operator: String::from("+="),
                                left: Box::new(Tree::Identifier { name: String::from("x") }),
                                right: Box::new(Tree::Identifier { name: String::from("y") })
                            }),
                        }),
                        alternate: Box::new(Some(Tree::ExpressionStatement {
                            expression: Box::new(Tree::AssignmentExpression {
                                operator: String::from("="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 })
                            }),
                        })),
                    }),
                    alternate: Box::new(Some(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 10.0 })
                        }),
                    })),
                },
            ]),
        };
        assert_tree(expected, "if (x) if (y) x += y; else y = 42; else x = 10;");
    }

    #[test]
    fn test_parse_else_if_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::Identifier { name: String::from("x") }),
                    consequent: Box::new(Tree::ExpressionStatement {
                        expression: Box::new(Tree::AssignmentExpression {
                            operator: String::from("+="),
                            left: Box::new(Tree::Identifier { name: String::from("x") }),
                            right: Box::new(Tree::NumericLiteral { value: 42.0 })
                        }),
                    }),
                    alternate: Box::new(Some(Tree::IfStatement {
                        test: Box::new(Tree::Identifier { name: String::from("y") }),
                        consequent: Box::new(Tree::ExpressionStatement {
                            expression: Box::new(Tree::AssignmentExpression {
                                operator: String::from("+="),
                                left: Box::new(Tree::Identifier { name: String::from("y") }),
                                right: Box::new(Tree::NumericLiteral { value: 42.0 })
                            }),
                        }),
                        alternate: Box::new(Some(Tree::BlockStatement {
                            body: Box::new(vec![
                                Tree::ExpressionStatement {
                                    expression: Box::new(Tree::AssignmentExpression {
                                        operator: String::from("="),
                                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                                        right: Box::new(Tree::NumericLiteral { value: 42.0 })
                                    }),
                                },
                                Tree::ExpressionStatement {
                                    expression: Box::new(Tree::AssignmentExpression {
                                        operator: String::from("="),
                                        left: Box::new(Tree::Identifier { name: String::from("y") }),
                                        right: Box::new(Tree::NumericLiteral { value: 10.0 })
                                    }),
                                }
                            ]),
                        })),
                    })),
                },
            ]),
        };
        assert_tree(expected, "if (x) x += 42; else if (y) y += 42; else { x = 42; y = 10; }");
    }

    // ----- TESTS FOR RELATIONAL EXPRESSIONS ----- //

    #[test]
    fn test_parse_simple_relational_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::BinaryExpression {
                        operator: String::from(">="),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 42.0, }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x >= 42;");
    }

    #[test]
    fn test_parse_complex_relational_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::Identifier { name: String::from("y") }),
                        right: Box::new(Tree::BinaryExpression {
                            operator: String::from(">"),
                            left: Box::new(Tree::BinaryExpression {
                                operator: String::from("*"),
                                left: Box::new(Tree::BinaryExpression {
                                    operator: String::from("+"),
                                    left: Box::new(Tree::Identifier { name: String::from("x") }),
                                    right: Box::new(Tree::NumericLiteral { value: 10.0 }),
                                }),
                                right: Box::new(Tree::NumericLiteral { value: 3.0 }),
                            }),
                            right: Box::new(Tree::NumericLiteral { value: 100.0 }),
                        }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "y = (x + 10) * 3 > 100;");
    }

    #[test]
    fn test_parse_relational_expression_if_statement() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::IfStatement {
                    test: Box::new(Tree::BinaryExpression {
                        operator: String::from("<"),
                        left: Box::new(Tree::Identifier { name: String::from("x") }),
                        right: Box::new(Tree::NumericLiteral { value: 42.0, }),
                    }),
                    consequent: Box::new(Tree::BlockStatement { body: Box::new(vec![]) }),
                    alternate: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "if (x < 42) {}");
    }

}
