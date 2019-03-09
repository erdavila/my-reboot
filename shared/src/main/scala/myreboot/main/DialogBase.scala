package myreboot.main

import myreboot.Action
import scalafx.Includes._
import scalafx.application.JFXApp
import scalafx.application.JFXApp.PrimaryStage
import scalafx.geometry.Insets
import scalafx.scene.Scene
import scalafx.scene.control.Button
import scalafx.scene.image.Image
import scalafx.scene.layout.VBox

abstract class DialogBase extends JFXApp {
  protected def platformName: String
  protected def actions: Seq[Action]

  stage = new PrimaryStage {
    title = s"My Reboot - $platformName"
    icons.add(new Image(classOf[DialogBase].getResourceAsStream("/icon.png")))
    resizable = false
    scene = new Scene(
      new VBox(10) {
        padding = Insets(10)
        children =
          for (action <- actions)
            yield new Button(action.label) {
              maxWidth = Double.MaxValue
              onAction = handle {
                action()
                stage.close()
              }
            }
      }
    )
  }
}
