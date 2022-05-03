#include "pch.h"

using namespace winrt;
using namespace Component;

extern "C" HRESULT WINAPI GetClassInstance(int32_t value, void** class_instance);

int main()
{
    Class c;
    c.Property(123);
    assert(c.Property() == 123);
    Class d = c.Make(456);
    assert(d.Property() == 456);
    Class e = c.MakeTypeErased(789).as<Class>();
    assert(e.Property() == 789);

    HMODULE mod = LoadLibraryExW(L"component.dll", nullptr, LOAD_WITH_ALTERED_SEARCH_PATH);
    assert(mod != nullptr);
    if (mod != nullptr)
    {
        auto get_class_instance_fn = reinterpret_cast<decltype(&GetClassInstance)>((GetProcAddress(mod, "GetClassInstance")));
        assert(get_class_instance_fn != nullptr);
        if (get_class_instance_fn != nullptr)
        {
            void* ptr{ nullptr };
            get_class_instance_fn(101, &ptr);
            Class f{ nullptr };
            winrt::attach_abi(f, ptr);
            assert(f.Property() == 101);
        }

        FreeLibrary(mod);
    }

    printf("ok!");
}
