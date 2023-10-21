void RollD6() {
    var random = new Random();
    var result = random.Next(1, 7);
    Console.WriteLine("Rolled: " + result);
}

RollD6();
