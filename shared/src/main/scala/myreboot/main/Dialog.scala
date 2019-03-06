package myreboot.main

import myreboot.Platform
import scalafx.Includes._
import scalafx.application.JFXApp
import scalafx.application.JFXApp.PrimaryStage
import scalafx.geometry.Insets
import scalafx.scene.Scene
import scalafx.scene.control.Button
import scalafx.scene.layout.VBox

object Dialog extends JFXApp {

  private val platform = Platform()

  stage = new PrimaryStage {
    title = s"My Reboot - ${platform.name}"
    resizable = false
    scene = new Scene(
      new VBox(10) {
        padding = Insets(10)
        children =
          for (a <- platform.actions)
            yield new Button(a.label) {
              maxWidth = Double.MaxValue
              onAction = handle {
                a()
                stage.close()
              }
            }
      }
    )
  }
}
