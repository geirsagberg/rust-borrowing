var a = new string("abc");
var b = a;
b += "def";
Console.WriteLine(a);
Console.WriteLine(b);

var firstList = new List<int> { 1, 2, 3 };
var secondList = firstList;
secondList.Add(4);
Console.WriteLine(firstList.Count);
Console.WriteLine(secondList.Count);
