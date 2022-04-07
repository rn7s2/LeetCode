class Program
{
    int add(int a, int b)
    {
        return a + b;
    }

    public void Run()
    {
        Console.WriteLine(add(1, 2));
    }

    public static void Main()
    {
        new Program().Run();
    }
}
