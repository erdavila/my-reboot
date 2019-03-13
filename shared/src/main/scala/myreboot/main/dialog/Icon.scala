package myreboot.main.dialog

import scalafx.scene.image.Image

case class Icon(resourceName: String) {
  def toImage: Image =
    new Image(getClass.getResourceAsStream(resourceName))
}

object Icon {
  val App = Icon("/icon.png")
  val Shutdown = Icon("/icon.png")
  val Windows = Icon("/windows.png")
  val Linux = Icon("/linux.png")
}
