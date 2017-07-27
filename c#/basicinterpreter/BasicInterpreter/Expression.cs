using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace BasicInterpreter
{
    class Expression
    {
        private Variable left, right;
        private Operator expressionOperator;
        enum Operator { Plus,Minus,Multiply,Divide};
        public Expression(string line)
        {
            var exp = line.Split();
            if (exp.Length == 1)
            {
                //Unary
                left = Variable.FromString(exp[0]);
            }
            else
            {
                //Binary
                left = Variable.FromString(exp[0]);
                right = Variable.FromString(exp[2]);
                switch (exp[1])
                {
                    case "+":
                        expressionOperator = Operator.Plus;
                        break;
                    case "-":
                        expressionOperator = Operator.Minus;
                        break;
                    case "*":
                        expressionOperator = Operator.Multiply;
                        break;
                    case "/":
                        expressionOperator = Operator.Divide;
                        break;
                }
            }
        }

        public int GetValue()
        {
            if (right == null)
                return left.Get();
            else
            {
                switch (expressionOperator)
                {
                    case Operator.Plus:
                        return left.Get() + right.Get();
                    case Operator.Minus:
                        return left.Get() - right.Get();
                    case Operator.Multiply:
                        return left.Get() * right.Get();
                    case Operator.Divide:
                        return left.Get() / right.Get();
                    default:
                        throw new NotSupportedException();
                }
            }
        }
    }
}
