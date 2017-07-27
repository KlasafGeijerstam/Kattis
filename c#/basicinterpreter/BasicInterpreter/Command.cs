namespace BasicInterpreter
{
    internal abstract class Command
    {
        public int Label { get; set; }
        protected string line;
        public Command(string line)
        {
            this.line = line;
            Label = int.Parse(line.Split()[0]);
        }

        public abstract void Execute();
    }
}
