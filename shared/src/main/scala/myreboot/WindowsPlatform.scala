package myreboot

import java.io.File
import scala.sys.process._
import windowsapi.scala.User32

object WindowsPlatform {
  val StateDir = new File("C:\\grubenv.dir")

  val DisplaySwitchExe = "DisplaySwitch.exe"

  lazy val configs: Configs = Configs.load(StateDir)

  def shutdown(): Unit = {
    switchDisplay(Monitor)
    Seq("shutdown", "/sg", "/t", "0").!!
  }

  def switchDisplay(display: Display): Unit =
    if (currentDisplay().contains(display)) {
      println(s"Already in wanted display: ${display.code}")
    } else {
      Seq(DisplaySwitchExe, configs.windowsDisplaySwitchArgs(display)).!!
    }

  def currentDisplay(): Option[Display] =
    for {
      deviceId <- currentDeviceId()
      display <- configs.displayByDeviceId(deviceId)
    } yield display

  def currentDeviceId(): Option[String] = {
    val activeDisplayDevices = for {
      adapter <- User32.enumDisplayDevices().toList
      displayDevice <- User32.enumDisplayDevices(adapter.name)
      if displayDevice.active
    } yield displayDevice

    activeDisplayDevices match {
      case List(displayDevice) => Some(displayDevice.id)
      case _ => None
    }
  }
}
