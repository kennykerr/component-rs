class Program
{
    static void Main(string[] args)
    {
        var c = new Component.Class();
        c.Property = 123;
        System.Diagnostics.Debug.Assert(c.Property == 123);
        System.Console.WriteLine("ok!");
    }
}
