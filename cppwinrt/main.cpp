#include "pch.h"

using namespace winrt;
using namespace Component;

int main()
{
    Class c;
    c.Property(123);
    assert(c.Property() == 123);
    printf("ok!");
}
