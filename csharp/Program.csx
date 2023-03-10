static int Roll()
{
    var random = new Random();
    var result = random.Next(1, 7);
    Console.WriteLine(result);
}

Roll();
