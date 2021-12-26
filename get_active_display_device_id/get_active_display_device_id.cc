#include <iostream>
#include <string>
#include "Windows.h"

using namespace std;

// See https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesa

int main() {
    DISPLAY_DEVICE dd;
    dd.cb = sizeof(dd);

    bool result1 = true;
    for (int i = 0; result1; i++) {
        result1 = EnumDisplayDevices(nullptr, i, &dd, 0);
        if (result1) {
            string device_name = dd.DeviceName;

            bool result2 = true;
            for (int j = 0; result2; j++) {
                result2 = EnumDisplayDevices(device_name.c_str(), j, &dd, 0);
                if (result2  &&  (dd.StateFlags & DISPLAY_DEVICE_ACTIVE)) {
                    cout << dd.DeviceID << endl;
                    return 0;
                }
            }
        }
    }

    return 1;
}
