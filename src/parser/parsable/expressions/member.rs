use crate::prelude::*;

use super::eatable::Eatable;
use super::identifier::IdentifierParsable;
use super::primary::PrimaryExpressionParsable;
use super::statements::expression::ExpressionStatementParsable;

pub trait MemberExpressionParsable {
    /**
     * MemberExpression
     *  : PrimaryExpression
     *  | MemberExpression '.' Identifier
     *  | MemberExpression '[' Expression ']'
     *  ;
     */
    fn member_expression(&mut self) -> Result<Tree>;
}

impl MemberExpressionParsable for Parser {
    fn member_expression(&mut self) -> Result<Tree> {
        let mut object = self.primary_expression()?;

        while self.lookahead.token_type == TokenType::Dot || self.lookahead.token_type == TokenType::SquareBracketOpen {
            match self.lookahead.token_type {

                // Non-computed member expression '.' -> x.y
                TokenType::Dot => {
                    self.eat(TokenType::Dot)?;
                    object = Tree::MemberExpression {
                        object: Box::new(object),
                        property: Box::new(self.identifier()?),
                        computed: false,
                    };
                },

                // Computed member expression '[...]' -> x['y']
                _ => {
                    self.eat(TokenType::SquareBracketOpen)?;
                    object = Tree::MemberExpression {
                        object: Box::new(object),
                        property: Box::new(self.expression()?),
                        computed: true,
                    };
                    self.eat(TokenType::SquareBracketClose)?;
                },
            }
        }

        // Either PrimaryExpression or MemberExpression
        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::parser::parsable::tests::*;

    #[test]
    fn test_parse_non_computed_member_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::MemberExpression {
                        object: Box::new(Tree::Identifier { name: String::from("str") }),
                        property: Box::new(Tree::Identifier { name: String::from("length") }),
                        computed: false,
                    }),
                },
            ]),
        };
        assert_tree(expected, "str.length;");
    }

    #[test]
    fn test_parse_assigning_non_computed_member_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::MemberExpression {
                            object: Box::new(Tree::Identifier { name: String::from("x") }),
                            property: Box::new(Tree::Identifier { name: String::from("y") }),
                            computed: false,
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "x.y = 1;");
    }

    #[test]
    fn test_parse_assigning_computed_member_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::AssignmentExpression {
                        operator: String::from("="),
                        left: Box::new(Tree::MemberExpression {
                            object: Box::new(Tree::Identifier { name: String::from("arr") }),
                            property: Box::new(Tree::NumericLiteral { value: 0.0 }),
                            computed: true,
                        }),
                        right: Box::new(Tree::NumericLiteral { value: 1.0 }),
                    }),
                },
            ]),
        };
        assert_tree(expected, "arr[0] = 1;");
    }

    #[test]
    fn test_parse_chained_member_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::MemberExpression {
                        object: Box::new(Tree::MemberExpression {
                            object: Box::new(Tree::MemberExpression {
                                object: Box::new(Tree::Identifier { name: String::from("a") }),
                                property: Box::new(Tree::Identifier { name: String::from("b") }),
                                computed: false,
                            }),
                            property: Box::new(Tree::Identifier { name: String::from("c") }),
                            computed: false,
                        }),
                        property: Box::new(Tree::StringLiteral { value: String::from("d") }),
                        computed: true,
                    }),
                },
            ]),
        };
        assert_tree(expected, "a.b.c['d'];");
    }
}