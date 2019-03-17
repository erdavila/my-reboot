package myreboot

import java.io.File
import myreboot.utils.Executor.execute
import scala.annotation.tailrec
import windowsapi.scala.User32

object Platform extends OSPlatform {
  override val StateDir = new File("C:\\grubenv.dir")
  val DisplaySwitchExe = "DisplaySwitch.exe"

  override def reboot(): Unit = {
    switchDisplay(Monitor)
    shutdownNow(reboot = true)
  }

  override def shutdown(): Unit = {
    switchDisplay(Monitor)
    shutdownNow(reboot = false)
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
      execute(DisplaySwitchExe, configs.windowsDisplaySwitchArgs(display))
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
    execute("shutdown", arg, "/t", "0")
  }
}
