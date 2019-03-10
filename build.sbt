import Logging._
import java.io.File
import scala.sys.process._


ThisBuild / name := "My Reboot"
ThisBuild / version := "0.1"

ThisBuild / scalaVersion := "2.12.8"


// Settings
lazy val jarName = settingKey[String]("Jar name")
lazy val installDir = settingKey[File]("Directory where executables will be written")
lazy val installedAssetsDir = settingKey[File]("Directory where non-executables will be written")
lazy val installedJar = settingKey[File]("Installed Jar path")
lazy val installedMainLaunchingScript = settingKey[File]("Main launching script")
lazy val installedSwitchDisplayScript = settingKey[File]("Script to switch displays")
lazy val installedLaunchingScripts = settingKey[Seq[(File, String)]]("Scripts and their main class names")
lazy val installedIcon = settingKey[File]("Installed icon path")
lazy val installedMenuEntry = settingKey[File]("Installed desktop menu entry path")

// Tasks
lazy val installJar = taskKey[Unit]("Installs the jar file")
lazy val installLaunchingScripts = taskKey[Unit]("Installs launching scripts")
lazy val installMenuEntry = taskKey[Unit]("Installs desktop menu entry")
lazy val install = taskKey[Unit]("Installs")
lazy val runSetup = taskKey[Unit]("Runs setup")


lazy val shared = (project in file("shared"))
  .settings(
    libraryDependencies += "org.scalafx" %% "scalafx" % "8.0.181-R13",

    libraryDependencies += "org.scalatest" %% "scalatest" % "3.0.5" % "test",
  )


lazy val commonSettings = Seq(
  jarName := "my-reboot.jar",
  assembly / assemblyJarName := jarName.value,
  assembly / assemblyMergeStrategy := {
    case PathList("myreboot", "main", cls) if cls startsWith "Setup" => MergeStrategy.discard
    case p => (assembly / assemblyMergeStrategy).value(p)
  },

  installDir := {
    val userHome = scala.sys.env.getOrElse("HOME", System.getProperty("user.home"))
    file(userHome) / "bin"
  },
  installedAssetsDir := installDir.value / "my-reboot",
  installedJar := installedAssetsDir.value / jarName.value,
  installedMainLaunchingScript := installDir.value / "my-reboot-dialog.sh",
  installedLaunchingScripts := Seq(installedMainLaunchingScript.value -> "Dialog"),

  installJar := {
    val destJarFile = installedJar.value
    streams.value.log.copying(destJarFile)
    IO.copyFile(assembly.value, destJarFile)
  },
  installLaunchingScripts := {
    val targetJarFile = installedJar.value

    for ((scriptFile, mainClass) <- installedLaunchingScripts.value) {
      val targetJarPath = IO.relativize(scriptFile.getParentFile, targetJarFile)
        .getOrElse(targetJarFile.getPath)

      val escapedTargetJarPath = OS.which match {
        case Linux => targetJarPath
        case Windows => targetJarPath.replace("\\", "\\\\")
      }

      streams.value.log.writing(scriptFile)
      IO.write(scriptFile,
        s"""
           |#!/bin/bash
           |cd $$(dirname $$0)
           |exec java -cp $escapedTargetJarPath myreboot.main.$mainClass "$$@"
       """.stripMargin.trim + "\n"
      )
      if (OS.which == Linux) {
        IO.setPermissions(scriptFile, "rwxr-xr-x")
      }
    }
  },
  runSetup := {
    (Compile / runMain).toTask(" myreboot.main.Setup").value
  },
)


lazy val linux = (project in file("linux"))
  .dependsOn(shared)
  .settings(commonSettings: _*)
  .settings(
    installedIcon := installedAssetsDir.value / "icon.png",
    installedMenuEntry := installedAssetsDir.value / "entry.desktop",

    installMenuEntry := {
      val log = streams.value.log

      val iconFile = installedIcon.value
      log.copying(iconFile)
      IO.copyFile((ThisBuild / baseDirectory).value / "icon.png", iconFile)

      val menuEntryFile = installedMenuEntry.value
      log.writing(menuEntryFile)
      IO.write(menuEntryFile,
        s"""
           |[Desktop Entry]
           |Version=${version.value}
           |Type=Application
           |Name=${(ThisBuild / name).value}
           |Icon=${iconFile.getPath}
           |Exec=${installedMainLaunchingScript.value.getPath}
           |Comment=Reboot options
           |Categories=System;Utility;
           |Terminal=false
        """.stripMargin.trim + "\n"
      )

      log.info("Creating menu entry")
      Seq("xdg-desktop-menu", "install", "--novendor", "--mode", "user", menuEntryFile.getPath).!!
      Seq("xdg-desktop-menu", "forceupdate", "--mode", "user").!!
    },
    install := Def.sequential(
      installJar,
      installLaunchingScripts,
      installMenuEntry,
      runSetup,
    ).value,
  )


lazy val windows = (project in file("windows"))
  .dependsOn(shared)
  .settings(commonSettings: _*)
  .settings(
    libraryDependencies += "net.java.dev.jna" % "jna" % "5.2.0",

    installedSwitchDisplayScript := installDir.value / "my-reboot-switch-display.sh",
    installedLaunchingScripts += (installedSwitchDisplayScript.value -> "SwitchDisplay"),

    install := Def.sequential(
      installJar,
      installLaunchingScripts,
      runSetup,
    ).value,
  )


lazy val myReboot = (project in file("."))
  .aggregate(shared, linux, windows)
  .settings(
    install / aggregate := false,

    install := Def.taskDyn {
      OS.which match {
        case Linux => Def.task { (linux / install).value }
        case Windows => Def.task { (windows / install).value }
      }
    }.value,
  )
