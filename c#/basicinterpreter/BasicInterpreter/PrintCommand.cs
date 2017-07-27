using System;

namespace BasicInterpreter
{

    class PrintCommand : Command
    {
        private Variable message;
        public PrintCommand(string line) : base(line)
        {
            message = Variable.FromString(line.Substring(line.IndexOf("PRINT") + 6));
        }

        public override void Execute()
        {
            Console.Write(message.ToString());
            Interpreter.NextCommand(Label);
        }
    }
    
}
 
