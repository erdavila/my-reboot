package myreboot.main

import myreboot.main.dialog.{Action, Icon}
import scalafx.Includes._
import scalafx.application.JFXApp
import scalafx.application.JFXApp.PrimaryStage
import scalafx.geometry.{Insets, Pos}
import scalafx.scene.Scene
import scalafx.scene.control.Button
import scalafx.scene.image.ImageView
import scalafx.scene.layout.VBox

abstract class DialogBase extends JFXApp {
  protected def platformName: String
  protected def actions: Seq[Action]

  stage = new PrimaryStage {
    title = s"My Reboot - $platformName"
    icons.add(Icon.App.toImage)
    resizable = false
    scene = new Scene(
      new VBox(10) {
        padding = Insets(10)
        children =
          for (action <- actions)
            yield new Button(action.label) {
              alignment = Pos.BaselineLeft
              maxWidth = Double.MaxValue
              graphic = new ImageView(action.icon.toImage) {
                fitHeight = 22
                preserveRatio = true
                smooth = true
              }
              onAction = handle {
                action()
                stage.close()
              }
            }
      }
    )
  }
}
