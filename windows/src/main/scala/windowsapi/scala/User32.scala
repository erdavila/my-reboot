package windowsapi.scala

import com.sun.jna.Native
import windowsapi.java

object User32 {
  case class DisplayDevice(name: String, string: String, active: Boolean, id: String, key: String)

  def enumDisplayDevices(): Iterator[DisplayDevice] =
    enumDisplayDevices(null)

  def enumDisplayDevices(displayAdapterName: String): Iterator[DisplayDevice] =
    Iterator.from(0)
      .map { n =>
        val displayDevice = new java.User32.DISPLAY_DEVICE
        val result = java.User32.INSTANCE.EnumDisplayDevices(displayAdapterName, n, displayDevice, 0)
        (result, displayDevice)
      }
      .takeWhile { case (result, _) => result }
      .map { case (_, displayDevice: java.User32.DISPLAY_DEVICE) =>
        DisplayDevice(
          name = Native.toString(displayDevice.DeviceName),
          string = Native.toString(displayDevice.DeviceString),
          active = (displayDevice.StateFlags & java.User32.DISPLAY_DEVICE_ACTIVE) != 0,
          id = Native.toString(displayDevice.DeviceID),
          key = Native.toString(displayDevice.DeviceKey),
        )
      }
}
