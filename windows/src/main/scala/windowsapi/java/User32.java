package windowsapi.java;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Structure;
import com.sun.jna.win32.W32APIOptions;

public interface User32 extends Library {
    User32 INSTANCE = Native.load("user32", User32.class, W32APIOptions.DEFAULT_OPTIONS);

    // https://docs.microsoft.com/en-us/windows/desktop/api/wingdi/ns-wingdi-display_devicew
    @Structure.FieldOrder({"cb", "DeviceName", "DeviceString", "StateFlags", "DeviceID", "DeviceKey"})
    class DISPLAY_DEVICE extends Structure {
        public int cb;
        public char DeviceName[] = new char[32];
        public char DeviceString[] = new char[128];
        public int StateFlags;
        public char DeviceID[] = new char[128];
        public char DeviceKey[] = new char[128];

        public DISPLAY_DEVICE() {
            cb = size();
        }
    }

    // https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-enumdisplaydevicesw
    boolean EnumDisplayDevices(
        String lpDevice,
        int iDevNum,
        DISPLAY_DEVICE lpDisplayDevice,
        int dwFlags
    );

    int DISPLAY_DEVICE_ACTIVE = 1;
}
