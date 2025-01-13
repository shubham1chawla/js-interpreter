use crate::prelude::*;

use super::block::BlockStatementParsable;
use super::eatable::Eatable;
use super::expressions::assignment::AssignmentExpressionParsable;
use super::function::FunctionDeclarationParsable;
use super::identifier::IdentifierParsable;

pub trait ClassDeclarationParsable {
    /**
     * ClassDeclaration
     *  : 'class' Identifier OptClassExtends ClassBody
     *  ;
     * 
     * ClassExtends
     *  : 'extends' Identifier
     *  ;
     */
    fn class_delaration(&mut self) -> Result<Tree>;

    /**
     * ClassBody
     *  : '{' OptClassStatementList '}'
     *  ;
     */
    fn class_body(&mut self) -> Result<Tree>;

    /**
     * ClassStatementList
     *  : ClassStatement
     *  | ClassStatementList ClassStatement
     *  ;
     */
    fn class_statement_list(&mut self) -> Result<Vec<Tree>>;

    /**
     * ClassStatement
     *  : ConstructorDefinition
     *  | GetterDefinition
     *  | SetterDefinition
     *  | MethodDefinition
     *  | PropertyDefinition
     *  ;
     */
    fn class_statement(&mut self) -> Result<Tree>;

    /**
     * ConstructorDefinition
     *  : 'constructor' '(' OptFormalParameterList ')' BlockStatement
     *  ;
     */
    fn constructor_definition(&mut self) -> Result<Tree>;

    /**
     * GetterDefinition
     *  : 'get' Identifier '(' ')' BlockStatement
     *  ;
     */
    fn getter_definition(&mut self) -> Result<Tree>;
    
    /**
     * SetterDefinition
     *  : 'set' Identifier '(' Identifier ')' BlockStatement 
     *  ;
     */
    fn setter_definition(&mut self) -> Result<Tree>;

    /**
     * MethodDefinition
     *  : Identifier '(' OptFormalParameterList ')' BlockStatement
     *  ;
     */
    fn method_definition(&mut self, identifier: Tree) -> Result<Tree>;

    /**
     * PropertyDefinition
     *  : Identifier OptPropertyInitializer ';'
     *  ;
     */
    fn property_definition(&mut self) -> Result<Tree>;

    /**
     * PropertyInitializer
     *  : SIMPLE_ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn property_initializer(&mut self) -> Result<Tree>;
}

impl ClassDeclarationParsable for Parser {
    fn class_delaration(&mut self) -> Result<Tree> {
        self.eat(TokenType::ClassKeyword)?;
        let identifier = self.identifier()?;

        // OptClassExtends
        let super_class = match self.lookahead.token_type {
            TokenType::CurlyBracketOpen => None,
            _ => {
                self.eat(TokenType::ExtendsKeyword)?;
                Some(self.identifier()?)
            },
        };

        let body = self.class_body()?;

        Ok(Tree::ClassDeclaration {
            idenifier: Box::new(identifier),
            body: Box::new(body),
            super_class: Box::new(super_class),
        })
    }

    fn class_body(&mut self) -> Result<Tree> {
        self.eat(TokenType::CurlyBracketOpen)?;

        // OptClassStatementList
        let statements = match self.lookahead.token_type {
            TokenType::CircleBracketClose => vec![],
            _ => self.class_statement_list()?,
        };

        self.eat(TokenType::CurlyBracketClose)?;

        Ok(Tree::ClassBody { body: Box::new(statements) })
    }

    fn class_statement_list(&mut self) -> Result<Vec<Tree>> {
        let mut statements = vec![];

        while self.lookahead.token_type != TokenType::CurlyBracketClose {
            statements.push(self.class_statement()?);
        }

        Ok(statements)
    }

    fn class_statement(&mut self) -> Result<Tree> {
        match self.lookahead.token_type {
            TokenType::ConstructorKeyword => self.constructor_definition(),
            TokenType::GetKeyword => self.getter_definition(),
            TokenType::SetKeyword => self.setter_definition(),
            _ => self.property_definition(),
        }
    }

    fn constructor_definition(&mut self) -> Result<Tree> {
        self.eat(TokenType::ConstructorKeyword)?;
        self.eat(TokenType::CircleBracketOpen)?;

        // OptFormalParameterList
        let params = match self.lookahead.token_type {
            TokenType::CircleBracketClose => vec![],
            _ => self.formal_parameter_list()?,
        };

        self.eat(TokenType::CircleBracketClose)?;

        let body = self.block_statement()?;

        Ok(Tree::ConstructorDefinition {
            value: Box::new(Tree::FunctionExpression {
                identifier: Box::new(None),
                params: Box::new(params),
                body: Box::new(body),
            }),
        })
    }

    fn getter_definition(&mut self) -> Result<Tree> {
        self.eat(TokenType::GetKeyword)?;
        let identifier = self.identifier()?;

        self.eat(TokenType::CircleBracketOpen)?;

        // No params

        self.eat(TokenType::CircleBracketClose)?;

        let body = self.block_statement()?;

        Ok(Tree::GetterDefinition {
            key: Box::new(identifier),
            value: Box::new(Tree::FunctionExpression {
                identifier: Box::new(None),
                params: Box::new(vec![]),
                body: Box::new(body),
            }),
        })
    }

    fn setter_definition(&mut self) -> Result<Tree> {
        self.eat(TokenType::SetKeyword)?;
        let identifier = self.identifier()?;

        self.eat(TokenType::CircleBracketOpen)?;

        // Exactly one param
        let param = self.identifier()?;

        self.eat(TokenType::CircleBracketClose)?;

        let body = self.block_statement()?;

        Ok(Tree::SetterDefinition {
            key: Box::new(identifier),
            value: Box::new(Tree::FunctionExpression {
                identifier: Box::new(None),
                params: Box::new(vec![
                    param,
                ]),
                body: Box::new(body),
            }),
        })
    }

    fn method_definition(&mut self, identifier: Tree) -> Result<Tree> {
        self.eat(TokenType::CircleBracketOpen)?;

        // OptFormalParameterList
        let params = match self.lookahead.token_type {
            TokenType::CircleBracketClose => vec![],
            _ => self.formal_parameter_list()?,
        };

        self.eat(TokenType::CircleBracketClose)?;

        let body = self.block_statement()?;

        Ok(Tree::MethodDefinition {
            key: Box::new(identifier),
            value: Box::new(Tree::FunctionExpression {
                identifier: Box::new(None),
                params: Box::new(params),
                body: Box::new(body),
            }),
        })
    }

    fn property_definition(&mut self) -> Result<Tree> {
        let identifier = self.identifier()?;

        // Checking if production is MethodDefinition
        if self.lookahead.token_type == TokenType::CircleBracketOpen {
            return self.method_definition(identifier);
        }
        
        // OptPropertyInitializer
        let value = match self.lookahead.token_type {
            TokenType::SimpleAssignmentOperator => Some(self.property_initializer()?),
            _ => None,
        };

        self.eat(TokenType::SemiColon)?;

        Ok(Tree::PropertyDefinition {
            key: Box::new(identifier),
            value: Box::new(value),
        })
    }

    fn property_initializer(&mut self) -> Result<Tree> {
        self.eat(TokenType::SimpleAssignmentOperator)?;
        self.assignment_expression()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_simple_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![]),
                    }),
                    super_class: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "class Point{}");
    }

    #[test]
    fn test_parse_extended_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point3D") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![]),
                    }),
                    super_class: Box::new(Some(Tree::Identifier { name: String::from("Point") })),
                },
            ]),
        };
        assert_tree(expected, "class Point3D extends Point{}");
    }

    #[test]
    fn test_parse_property_definition_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![
                            Tree::PropertyDefinition {
                                key: Box::new(Tree::Identifier { name: String::from("x") }),
                                value: Box::new(Some(Tree::NumericLiteral { value: 10.0 })),
                            },
                            Tree::PropertyDefinition {
                                key: Box::new(Tree::Identifier { name: String::from("y") }),
                                value: Box::new(None),
                            },
                        ]),
                    }),
                    super_class: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "
            class Point {
                x = 10;
                y;
            }
        ");
    }

    #[test]
    fn test_parse_constructor_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![
                            Tree::ConstructorDefinition {
                                value: Box::new(Tree::FunctionExpression {
                                    identifier: Box::new(None),
                                    params: Box::new(vec![
                                        Tree::Identifier { name: String::from("x") },
                                        Tree::Identifier { name: String::from("y") },
                                    ]),
                                    body: Box::new(Tree::BlockStatement {
                                        body: Box::new(vec![
                                            Tree::ExpressionStatement { 
                                                expression: Box::new(Tree::AssignmentExpression {
                                                    operator: String::from("="),
                                                    left: Box::new(Tree::MemberExpression {
                                                        object: Box::new(Tree::ThisLiteral),
                                                        property: Box::new(Tree::Identifier { name: String::from("x") }),
                                                        computed: false,
                                                    }),
                                                    right: Box::new(Tree::Identifier { name: String::from("x") }),
                                                }),
                                            },
                                            Tree::ExpressionStatement { 
                                                expression: Box::new(Tree::AssignmentExpression {
                                                    operator: String::from("="),
                                                    left: Box::new(Tree::MemberExpression {
                                                        object: Box::new(Tree::ThisLiteral),
                                                        property: Box::new(Tree::Identifier { name: String::from("y") }),
                                                        computed: false,
                                                    }),
                                                    right: Box::new(Tree::Identifier { name: String::from("y") }),
                                                }),
                                            },
                                        ]),
                                    }),
                                }),
                            },
                        ]),
                    }),
                    super_class: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "
            class Point {
                constructor(x, y) {
                    this.x = x;
                    this.y = y;
                }
            }
        ");
    }

    #[test]
    fn test_parse_getter_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![
                            Tree::GetterDefinition {
                                key: Box::new(Tree::Identifier { name: String::from("x") }),
                                value: Box::new(Tree::FunctionExpression {
                                    identifier: Box::new(None),
                                    params: Box::new(vec![]),
                                    body: Box::new(Tree::BlockStatement {
                                        body: Box::new(vec![
                                            Tree::ReturnStatement {
                                                argument: Box::new(Some(Tree::Identifier { name: String::from("x") })),
                                            },
                                        ]),
                                    }),
                                }),
                            },
                        ]),
                    }),
                    super_class: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "
            class Point {
                get x() { return x; }
            }
        ");
    }

    #[test]
    fn test_parse_invalid_getter_class_declaration() {
        let expected = Error::Syntax("Unexpected token Identifier, expected CircleBracketClose!".to_string());
        assert_syntax_error(expected, "
            class Point {
                get x(y) {}
            }
        ");
    }

    #[test]
    fn test_parse_setter_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![
                            Tree::SetterDefinition {
                                key: Box::new(Tree::Identifier { name: String::from("x") }),
                                value: Box::new(Tree::FunctionExpression {
                                    identifier: Box::new(None),
                                    params: Box::new(vec![
                                        Tree::Identifier { name: String::from("y") },
                                    ]),
                                    body: Box::new(Tree::BlockStatement {
                                        body: Box::new(vec![
                                            Tree::ExpressionStatement {
                                                expression: Box::new(Tree::AssignmentExpression {
                                                    operator: String::from("="),
                                                    left: Box::new(Tree::MemberExpression {
                                                        object: Box::new(Tree::ThisLiteral),
                                                        property: Box::new(Tree::Identifier { name: String::from("x") }),
                                                        computed: false,
                                                    }),
                                                    right: Box::new(Tree::Identifier { name: String::from("y") }),
                                                }),
                                            },
                                        ]),
                                    }),
                                }),
                            },
                        ]),
                    }),
                    super_class: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "
            class Point {
                set x(y) { this.x = y; }
            }
        ");
    }

    #[test]
    fn test_parse_invalid_setter_class_declaration_1() {
        let expected = Error::Syntax("Unexpected token Comma, expected CircleBracketClose!".to_string());
        assert_syntax_error(expected, "
            class Point {
                set x(y, z) {}
            }
        ");
    }

    #[test]
    fn test_parse_invalid_setter_class_declaration_2() {
        let expected = Error::Syntax("Unexpected token CircleBracketClose, expected Identifier!".to_string());
        assert_syntax_error(expected, "
            class Point {
                set x() {}
            }
        ");
    }

    #[test]
    fn test_parse_method_definition_class_declaration() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ClassDeclaration {
                    idenifier: Box::new(Tree::Identifier { name: String::from("Point") }),
                    body: Box::new(Tree::ClassBody {
                        body: Box::new(vec![
                            Tree::MethodDefinition {
                                key: Box::new(Tree::Identifier { name: String::from("hello") }),
                                value: Box::new(Tree::FunctionExpression {
                                    identifier: Box::new(None),
                                    params: Box::new(vec![
                                        Tree::Identifier { name: String::from("name") },
                                    ]),
                                    body: Box::new(Tree::BlockStatement {
                                        body: Box::new(vec![
                                            Tree::ReturnStatement {
                                                argument: Box::new(Some(Tree::BinaryExpression {
                                                    operator: String::from("+"),
                                                    left: Box::new(Tree::StringLiteral { value: String::from("hello, ") }),
                                                    right: Box::new(Tree::Identifier { name: String::from("name") }),
                                                })),
                                            },
                                        ]),
                                    }),
                                }),
                            },
                        ]),
                    }),
                    super_class: Box::new(None),
                },
            ]),
        };
        assert_tree(expected, "
            class Point {
                hello(name) {
                    return 'hello, ' + name;
                }
            }
        ");
    }
}
