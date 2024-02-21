pub enum Expression {
    /// A value, like `a`.
    Variable {
        /// The string identifier for the variable.
        ident: String,
    },
    /// Application of a function to a value, like `a b`.
    Application {
        /// The left-hand side of the application.
        function: Box<Expression>,
        /// The right-hand side of the application.
        argument: Box<Expression>,
    },
    /// A function, like `\a . a`.
    Lambda {
        /// The singular argument for the lambda.
        argument: String,
        /// The body that the argument will be substituted into.
        body: Box<Expression>,
    },
}

impl Expression {
    /// Returns a Vec of all free/unbound variables in the expression.
    pub fn free_variables(&self) -> Vec<&String> {
        match self {
            Self::Variable { ident } => vec![ident],
            Self::Application { function, argument } => {
                let mut variables = function.free_variables();
                for variable in argument.free_variables() {
                    if variables.contains(&variable) {
                        continue;
                    }
                    variables.push(variable);
                }
                variables
            }
            Self::Lambda { argument, body } => body
                .free_variables()
                .into_iter()
                .filter(|&x| x != argument)
                .collect(),
        }
    }
}
