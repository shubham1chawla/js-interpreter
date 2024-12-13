use call::CallExpressionParsable;
use eatable::Eatable;
use member::MemberExpressionParsable;

use super::*;

pub trait NewExpressionParsable {
    /**
     * NewExpression
     *  : 'new' MemberExpression Arguments
     *  ;
     */
    fn new_expression(&mut self) -> Result<Tree, SyntaxError>;
}

impl NewExpressionParsable for Parser {
    fn new_expression(&mut self) -> Result<Tree, SyntaxError> {
        self.eat(TokenType::NewKeyword)?;
        Ok(Tree::NewExpression {
            callee: Box::new(self.member_expression()?),
            arguments: Box::new(self.arguments()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use expressions::tests::assert_tree;

    use super::*;

    #[test]
    fn test_parse_simple_new_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::NewExpression {
                        callee: Box::new(Tree::Identifier { name: String::from("Point") }),
                        arguments: Box::new(vec![
                            Tree::Identifier { name: String::from("x") },
                            Tree::Identifier { name: String::from("y") },
                        ]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "new Point(x, y);");
    }

    #[test]
    fn test_parse_namedspace_new_expression() {
        let expected = Tree::Program {
            body: Box::new(vec![
                Tree::ExpressionStatement {
                    expression: Box::new(Tree::NewExpression {
                        callee: Box::new(Tree::MemberExpression {
                            object: Box::new(Tree::Identifier { name: String::from("MyNamespace") }),
                            property: Box::new(Tree::Identifier { name: String::from("MyClass") }),
                            computed: false,
                        }),
                        arguments: Box::new(vec![]),
                    }),
                },
            ]),
        };
        assert_tree(expected, "new MyNamespace.MyClass();");
    }
}
