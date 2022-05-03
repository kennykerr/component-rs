using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace netframework
{
    class Program
    {
        static void Main(string[] args)
        {
            var c = new Component.Class();
            c.Property = 123;
            System.Diagnostics.Debug.Assert(c.Property == 123);
            var d = c.Make(456);
            System.Diagnostics.Debug.Assert(d.Property == 456);
            System.Console.WriteLine("ok!");
        }
    }
}
