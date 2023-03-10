using Internal;

private static int Roll()
{
    Random random = new();
    int result = random.Next(1, 7);
    Console.WriteLine("Rolled: " + result);
    return result;
}

Roll();
