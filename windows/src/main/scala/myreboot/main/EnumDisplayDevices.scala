package myreboot.main

import myreboot.Platform
import windowsapi.scala.User32

object EnumDisplayDevices {
  def main(args: Array[String]): Unit = {
    for (adapter <- User32.enumDisplayDevices()) {
      println("Name: " + adapter.name)
      println("String: " + adapter.string)
      println("ID: " + adapter.id)
      println("Key: " + adapter.key)
      println("Monitors:")

      for (monitor <- User32.enumDisplayDevices(adapter.name)) {
        println("  Name: " + monitor.name)
        println("  String: " + monitor.string)
        println("  ID: " + monitor.id)
        println("  Key: " + monitor.key)
        println("  Active: " + monitor.active)
        println()
      }

      println()
    }

    println("currentDeviceId: " + Platform.currentDeviceId())
  }
}
