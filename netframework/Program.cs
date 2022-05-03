using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace netframework
{
    class Program
    {
        [DllImport("component.dll", CharSet = CharSet.Unicode, CallingConvention = CallingConvention.StdCall)]
        internal static extern Int32 GetClassInstance(Int32 value, out IntPtr class_instance);

        static void Main(string[] args)
        {
            var c = new Component.Class();
            c.Property = 123;
            System.Diagnostics.Debug.Assert(c.Property == 123);
            var d = c.Make(456);
            System.Diagnostics.Debug.Assert(d.Property == 456);
            var e = (Component.Class)c.MakeTypeErased(789);
            System.Diagnostics.Debug.Assert(e.Property == 789);
            GetClassInstance(101, out var ptr);
            var f = (Component.Class)Marshal.GetObjectForIUnknown(ptr);
            System.Diagnostics.Debug.Assert(f.Property == 101);
            Marshal.Release(ptr);
            System.Console.WriteLine("ok!");
        }
    }
}
