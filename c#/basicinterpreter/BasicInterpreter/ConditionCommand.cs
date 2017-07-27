using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace BasicInterpreter
{
    class ConditionCommand : Command
    {
        enum ConditionType { Equals,GreaterThan,LessThan,NotEquals,LessAndEquals,GreaterAndEquals }
        private ConditionType conditionType;
        private Variable left,right;
        private int gotoTarget;

        public ConditionCommand(string line) : base(line)
        {   //0  1  2  3 4  5     6   7
            //50 IF A <= 5 THEN GOTO 20
            var expr = line.Split();
            left = Variable.FromString(expr[2]);
            right = Variable.FromString(expr[4]);
            gotoTarget = int.Parse(expr[7]);
            switch (expr[3])
            {
                case "=":
                    conditionType = ConditionType.Equals;
                    break;
                case ">":
                    conditionType = ConditionType.GreaterThan;
                    break;
                case "<":
                    conditionType = ConditionType.LessThan;
                    break;
                case "<>":
                    conditionType = ConditionType.NotEquals;
                    break;
                case "<=":
                    conditionType = ConditionType.LessAndEquals;
                    break;
                case ">=":
                    conditionType = ConditionType.GreaterAndEquals;
                    break;
            }
        }
        
        public override void Execute()
        {
            var result = false;
            switch (conditionType)
            {
                case ConditionType.Equals:
                    result = left.Get() == right.Get();
                    break;
                case ConditionType.GreaterThan:
                    result = left.Get() > right.Get();
                    break;
                case ConditionType.LessThan:
                    result = left.Get() < right.Get();
                    break;
                case ConditionType.NotEquals:
                    result = left.Get() != right.Get();
                    break;
                case ConditionType.LessAndEquals:
                    result = left.Get() <= right.Get();
                    break;
                case ConditionType.GreaterAndEquals:
                    result = left.Get() >= right.Get();
                    break;
            }
            if (result)
                Interpreter.SetCommand(gotoTarget);
            else
                Interpreter.NextCommand(Label);
        }
    }
}
