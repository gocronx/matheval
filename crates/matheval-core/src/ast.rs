#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        func: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_number() {
        let expr = Expr::Number(42.0);
        assert_eq!(expr, Expr::Number(42.0));
        assert_ne!(expr, Expr::Number(43.0));
    }

    #[test]
    fn test_expr_variable() {
        let expr = Expr::Variable("x".to_string());
        assert_eq!(expr, Expr::Variable("x".to_string()));
        assert_ne!(expr, Expr::Variable("y".to_string()));
    }

    #[test]
    fn test_expr_binary() {
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        };
        
        let same = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        };
        
        assert_eq!(expr, same);
    }

    #[test]
    fn test_expr_unary() {
        let expr = Expr::Unary {
            op: UnaryOp::Neg,
            expr: Box::new(Expr::Number(5.0)),
        };
        
        let same = Expr::Unary {
            op: UnaryOp::Neg,
            expr: Box::new(Expr::Number(5.0)),
        };
        
        assert_eq!(expr, same);
    }

    #[test]
    fn test_expr_call() {
        let expr = Expr::Call {
            func: "sin".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        
        let same = Expr::Call {
            func: "sin".to_string(),
            args: vec![Expr::Number(0.0)],
        };
        
        assert_eq!(expr, same);
    }

    #[test]
    fn test_binary_op_equality() {
        assert_eq!(BinaryOp::Add, BinaryOp::Add);
        assert_ne!(BinaryOp::Add, BinaryOp::Sub);
    }

    #[test]
    fn test_unary_op_equality() {
        assert_eq!(UnaryOp::Neg, UnaryOp::Neg);
    }

    #[test]
    fn test_expr_clone() {
        let expr = Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Variable("x".to_string())),
            right: Box::new(Expr::Number(2.0)),
        };
        
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_nested_expr() {
        // (x + 1) * 2
        let expr = Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Variable("x".to_string())),
                right: Box::new(Expr::Number(1.0)),
            }),
            right: Box::new(Expr::Number(2.0)),
        };
        
        assert!(matches!(expr, Expr::Binary { .. }));
    }

    #[test]
    fn test_function_with_multiple_args() {
        let expr = Expr::Call {
            func: "max".to_string(),
            args: vec![
                Expr::Number(1.0),
                Expr::Number(2.0),
                Expr::Number(3.0),
            ],
        };
        
        if let Expr::Call { args, .. } = expr {
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected Call expression");
        }
    }
}
