package myreboot

import java.io.File
import scala.annotation.tailrec
import scala.sys.process._
import windowsapi.scala.User32

object Platform {
  val StateDir = new File("C:\\grubenv.dir")

  val DisplaySwitchExe = "DisplaySwitch.exe"

  lazy val configs: Configs = Configs.load(StateDir)

  def shutdown(): Unit = {
    switchDisplay(Monitor)
    shutdownNow(reboot = false)
  }

  def reboot(display: Display): Unit = {
    switchDisplay(Monitor)

    val bootOptions = BootOptions.using(StateDir, configs)
    bootOptions.setOS(Windows)
    bootOptions.setWindowsDisplay(display)

    shutdownNow(reboot = true)
  }

  def switchDisplay(display: Display): Unit = {
    def notInWantedDisplay = !currentDisplay().contains(display)

    @tailrec
    def waitToSwitch(count: Int): Unit = {
      Thread.sleep(1000)
      if (notInWantedDisplay) {
        if (count > 0) {
          waitToSwitch(count - 1)
        } else {
          System.err.println("Desistindo de esperar!")
        }
      }
    }

    if (notInWantedDisplay) {
      Seq(DisplaySwitchExe, configs.windowsDisplaySwitchArgs(display)).!!
      waitToSwitch(count = 10)
    } else {
      println(s"Tela atual já é a desejada: ${display.code}")
    }
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

  private def shutdownNow(reboot: Boolean): Unit = {
    val arg = if (reboot) "/g" else "/sg"
    Seq("shutdown", arg, "/t", "0").!!
  }
}
