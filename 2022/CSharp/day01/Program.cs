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
            Part1(input);
            Part2(input);
        }

        static List<Elf> ProcessInput(string input)
        {
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

            return elves;
        }

        static void Part1(string input)
        {
            var elves = ProcessInput(input);
            Elf maxElf = (from elf in elves
                          orderby elf.Calories.Sum() descending
                          select elf).First();
            Console.WriteLine($"Part 1: {maxElf.Calories.Sum()}");
        }

        static void Part2(string input)
        {
            var elves = ProcessInput(input);
            var elfMax = (from elf in elves
                          orderby elf.Calories.Sum() descending
                          select elf).Take(3).Sum(elf => elf.Calories.Sum());
            Console.WriteLine($"Part 2: {elfMax}");
        }

    }
}
