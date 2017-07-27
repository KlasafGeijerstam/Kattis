using System;

namespace BasicInterpreter
{
    class PrintLineCommand : Command
    {
        private Variable message;
        public PrintLineCommand(string line) : base(line)
        {
            message = Variable.FromString(line.Substring(line.IndexOf("PRINTLN") + 8));
        }

        public override void Execute()
        {
            Console.WriteLine(message.ToString());
            Interpreter.NextCommand(Label);
        }
    }
}
