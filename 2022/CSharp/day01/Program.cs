namespace Day01
{

    struct Elf
    {
        public List<int> Calories { get; private set; }

        public Elf()
        {
            Calories = new List<int>();
        }
    }

    class Program
    {

        static void Main(string[] args)
        {
            string input = File.ReadAllText("./input.txt");
            string[] elfStrings = input.Split("\n\n");
            List<Elf> elves = new List<Elf>();

            foreach (string elfString in elfStrings)
            {
                Elf elf = new Elf();
                string[] calorieStrings = elfString.Split("\n");
                foreach (string calorieString in calorieStrings)
                {
                    if (calorieString.Length == 0)
                    {
                        continue;
                    }
                    elf.Calories.Add(int.Parse(calorieString.Trim()));
                }
                elves.Add(elf);
            }

            Elf maxElf = (from elf in elves
                          orderby elf.Calories.Sum() descending
                          select elf).First();
            Console.WriteLine(maxElf.Calories.Sum());
        }

    }
}
