package myreboot

import java.io.File

trait OSPlatform {
  val StateDir: File
  protected lazy val configs: Configs = Configs.load(StateDir)
  lazy val bootOptions: BootOptions = BootOptions.using(StateDir, configs)

  def reboot(): Unit
  def shutdown(): Unit
}
