class Program
{
    static void Main(string[] args)
    {
        var c = new Component.Class();
        c.Property = 123;
        Debug.Assert(c.Property() == 123);
    }
}
