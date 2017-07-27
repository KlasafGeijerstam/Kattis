using System.Collections.Generic;
using System.Linq;

namespace BasicInterpreter
{
    partial class Interpreter
    {
        private static Dictionary<int, Command> lines;
        private static List<int> labels;
        internal static Dictionary<string, int> variables;
        private static Command currentCommand;

        static Interpreter()
        {
            lines = new Dictionary<int, Command>();
            variables = new Dictionary<string, int>();
            labels = new List<int>();
            Enumerable.Range(0, 26).ToList().ForEach(x => variables.Add(((char)('A' + x)).ToString(), 0));
        }
        
        public static void AddCommand(string command)
        {
            var stringCommand = command.Split()[1];
            Command newCommand = null;
            if(stringCommand == "PRINT")
                newCommand = new PrintCommand(command);
            else if (stringCommand == "PRINTLN")
                newCommand = new PrintLineCommand(command);
            else if(stringCommand == "LET")
            {
                newCommand = new LetCommand(command);
            }
            else
            {
                newCommand = new ConditionCommand(command);
            }
            lines.Add(newCommand.Label, newCommand);
            labels.Add(newCommand.Label);
        }

        public static void Run()
        {
            labels.Sort();
            currentCommand = lines[labels[0]];
            while(currentCommand != null)
                currentCommand.Execute();
        }

        internal static void NextCommand(int currentLabel)
        {
            var index = labels.BinarySearch(currentLabel);
            if (index != labels.Count - 1)
                currentCommand = lines[labels[index + 1]];
            else
                currentCommand = null;
        }

        internal static void SetCommand(int label)
        {
            currentCommand = lines[label];
        }
    }
}
 
