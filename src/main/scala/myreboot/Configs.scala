package myreboot

import java.io.{File, FileReader}
import java.util.Properties

object Configs {

  def load(directory: File): Configs = {
    val props = new Properties
    val file = new File(directory, "my-reboot-configs.properties")
    val reader = new FileReader(file)
    props.load(reader)
    reader.close()

    def enumProperties[A <: WithCode](values: Seq[A], subKey: String): Map[A, String] =
      values.map { value =>
        val key = value.code + "." + subKey
        val property = props.getProperty(key)
        value -> property
      }.toMap

    new Configs(
      osGrubEntry = enumProperties(OS.Values, "grubEntry"),
      windowsDeviceIds = enumProperties(Display.Values, "deviceId"),
      windowsDisplaySwitchArgs = enumProperties(Display.Values, "displaySwitchArg"),
    )
  }

}

class Configs private(
  val osGrubEntry: Map[OS, String],
  val windowsDeviceIds: Map[Display, String],
  val windowsDisplaySwitchArgs: Map[Display, String],
) {
  def displayByDeviceId(deviceId: String): Option[Display] =
    windowsDeviceIds.collectFirst { case (display, `deviceId`) => display }
}
