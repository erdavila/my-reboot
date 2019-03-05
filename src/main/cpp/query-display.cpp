#include <iostream>
#include <string>
#include <vector>
#include <windows.h>

using namespace std;

struct NameAndString {
	std::string name;
	std::string string;
	std::string id;

	explicit NameAndString(const DISPLAY_DEVICE& dd)
		: name(dd.DeviceName), string(dd.DeviceString), id(dd.DeviceID)
		{}
};

using Monitor = NameAndString;

struct Adapter {
	NameAndString nameAndString;
	vector<Monitor> monitors;
};


template <typename F>
static void query_display_devices(LPCSTR device, F func) {
	DISPLAY_DEVICE displayDevice;
	displayDevice.cb = sizeof(DISPLAY_DEVICE);

	for (DWORD devNum = 0; ; devNum++) {
		// https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-enumdisplaydevicesa
		BOOL result = EnumDisplayDevices(
			device,
			devNum,
			&displayDevice,
			0
		);
		if (!result) {
			break;
		}

		func(displayDevice);
	}
}

static vector<Monitor> query_monitors(const string& adapterName) {
	vector<Monitor> monitors;

	query_display_devices(
		adapterName.c_str(),
		[&monitors](const DISPLAY_DEVICE& displayDevice) {
			bool active = displayDevice.StateFlags & DISPLAY_DEVICE_ACTIVE;
			bool attached = displayDevice.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP;
			if (active && attached) {
				monitors.push_back(NameAndString(displayDevice));
			}
		}
	);

	return monitors;
}

static vector<Adapter> query_displays() {
	vector<Adapter> adapters;

	query_display_devices(
		NULL,
		[&adapters](const DISPLAY_DEVICE& displayDevice) {
			NameAndString adapterNameAndString(displayDevice);
			vector<Monitor> monitors = query_monitors(adapterNameAndString.name);
			if (!monitors.empty()) {
				adapters.push_back({ adapterNameAndString, monitors });
			}
		}
	);

	return adapters;
}

int main() {
	auto displays = query_displays();

	if (displays.size() > 1) {
		cout << "Extended" << endl;
	} else if (displays[0].monitors.size() > 1) {
		cout << "Mirrored" << endl;
	} else {
		cout << "Single" << endl;
		cout << displays[0].monitors[0].string << endl;
		cout << displays[0].monitors[0].name << endl;
		cout << displays[0].monitors[0].id << endl;
	}

	return 0;
}
