using System;

namespace BasicInterpreter
{
    internal class Variable
    {
        enum VariableType { Constant,StringConstant,Variable };
        private string identifier;
        private object value;
        private VariableType type;
        private Variable()
        {

        }
        public override string ToString()
        {
            return type == VariableType.Variable ? Interpreter.variables[identifier].ToString() : value.ToString();
        }

        public int Get()
        {
            if (type == VariableType.Variable)
                return Interpreter.variables[identifier];
            else if (type == VariableType.Constant)
                return (int)value;
            else
                throw new NotSupportedException("Can not convert StringConstant to int");
        }

        public static Variable FromString(string i)
        {
            if (char.IsUpper(i[0]))
                return new Variable()
                {
                    type = VariableType.Variable,
                    identifier = i
                };

            else if (char.IsDigit(i[0]) || i[0] == '-')
                return new Variable()
                {
                    type = VariableType.Constant,
                    value = int.Parse(i)
                };
            else
                return new Variable()
                {
                    type = VariableType.StringConstant,
                    value = i.Replace("\"", string.Empty)
                       
                };
        }
    }
}
 
