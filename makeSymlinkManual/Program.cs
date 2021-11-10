using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace makeSymlinkManual
{
    class Program
    {

        [DllImport("Kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
        [return: System.Runtime.InteropServices.MarshalAs(System.Runtime.InteropServices.UnmanagedType.I1)]
        static extern bool CreateSymbolicLink(string lpSymlinkFileName, string lpTargetFileName, SYMBOLIC_LINK_FLAG dwFlags);

        [Flags]
        enum SYMBOLIC_LINK_FLAG
        {
            File = 0,
            Directory = 1,
            AllowUnprivilegedCreate = 2
        }


        static void Main(string[] args)
        {
            if(args.Length < 2)
            {
                Console.WriteLine("Manually create a folder symlink");
                Console.WriteLine("Usage: " + Process.GetCurrentProcess().ProcessName + " <from> <to>");
                Console.WriteLine("All other arguments are ignored.");
            }
            else
            {
                if (CreateSymbolicLink(args[0],args[1],SYMBOLIC_LINK_FLAG.Directory))
                {
                    Console.WriteLine("Success.");
                }
                else
                {
                    Console.WriteLine("Failed: " + Marshal.GetLastWin32Error());
                }
            }
        }
    }
}
