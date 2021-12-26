#include <iostream>
#include <string>
#include "Windows.h"

using namespace std;

static void enumerate(const char* name, bool recurse, const string& indent = "") {
    DISPLAY_DEVICE dd;
    dd.cb = sizeof(dd);

    bool result = true;
    for (int i = 0; result; i++) {
        result = EnumDisplayDevices(name, i, &dd, 0);
        if (result) {
            cout << indent << "Name: " << dd.DeviceName << endl;
            cout << indent << "String: " << dd.DeviceString << endl;
            cout << indent << "ID: " << dd.DeviceID << endl;
            cout << indent << "Key: " << dd.DeviceKey << endl;
            cout << indent << "Active: " << boolalpha << (dd.StateFlags & DISPLAY_DEVICE_ACTIVE != 0) << endl;

            if (recurse) {
                cout << indent << "Monitors:" << endl;
                enumerate(dd.DeviceName, false, indent + "  ");
            }

            cout << endl;
        }
    }
}

int main() {
    enumerate(nullptr, true);
}
