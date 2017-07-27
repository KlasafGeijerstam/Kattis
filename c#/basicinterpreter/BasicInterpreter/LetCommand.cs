using System;

namespace BasicInterpreter
{
    class LetCommand : Command
    {
        private string targetVariable;
        private Expression expression;

        public LetCommand(string line) : base(line)
        {
            //0   1  2 3 4 5 6
            //10 LET A = 1 + 2
            targetVariable = line.Split()[2];
            expression = new Expression(line.Substring(line.IndexOf("LET") + 8));
        }

        public override void Execute()
        {
            Interpreter.variables[targetVariable] = expression.GetValue();
            Interpreter.NextCommand(Label);
        }
    }
}
 
