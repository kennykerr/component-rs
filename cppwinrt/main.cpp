#include "pch.h"

using namespace winrt;
using namespace Component;

int main()
{
    Class c;
    c.Property(123);
    assert(c.Property() == 123);
    Class d = c.Make(456);
    assert(d.Property() == 456);
    printf("ok!");
}
