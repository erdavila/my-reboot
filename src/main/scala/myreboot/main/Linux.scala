package myreboot.main

import scalafx.application.JFXApp
import scalafx.application.JFXApp.PrimaryStage
import scalafx.geometry.Insets
import scalafx.scene.Scene
import scalafx.scene.control.Button
import scalafx.scene.layout.VBox

object Linux extends JFXApp {
  private class MaxWidthButton(text: String) extends Button(text) {
    maxWidth = Double.MaxValue
  }

  stage = new PrimaryStage {
    title = "MyReboot - Linux"
    resizable = false
    scene = new Scene(
      new VBox(10) {
        padding = Insets(10)
        children = Seq(
          new MaxWidthButton("Desligar"),
          new MaxWidthButton("Reiniciar"),
          new MaxWidthButton("Reiniciar no Windows usando o monitor"),
          new MaxWidthButton("Reiniciar no Windows usando a TV"),
        )
      }
    )
  }
}
