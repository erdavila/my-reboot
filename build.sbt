import Logging._
import scala.sys.process._


name := "My Reboot"
version := "0.1"

scalaVersion := "2.12.8"

libraryDependencies += "org.scalafx" %% "scalafx" % "8.0.181-R13"
libraryDependencies += "net.java.dev.jna" % "jna" % "5.2.0"

libraryDependencies += "org.scalatest" %% "scalatest" % "3.0.5" % "test"

assembly / mainClass := Some("myreboot.main.Dialog")
assembly / assemblyJarName := "my-reboot.jar"


lazy val installDir = settingKey[File]("Directory where executables will be written")
installDir := {
  val userHome = scala.sys.env.getOrElse("HOME", System.getProperty("user.home"))
  file(userHome) / "bin"
}

lazy val installedAssetsDir = settingKey[File]("Directory where non-executables will be written")
installedAssetsDir := installDir.value / "my-reboot"

lazy val installedJar = taskKey[File]("Installed Jar path")
installedJar := installedAssetsDir.value / (assembly / assemblyJarName).value

lazy val installedMainLaunchingScript = settingKey[File]("Main launching script")
installedMainLaunchingScript := installDir.value / "my-reboot-dialog.sh"

lazy val installedSwitchDisplayScript = settingKey[File]("Script to switch displays")
installedSwitchDisplayScript := installDir.value / "my-reboot-switch-display.sh"

lazy val installedIcon = taskKey[File]("Installed icon path")
installedIcon := installedAssetsDir.value / "icon.png"

lazy val installedMenuEntry = taskKey[File]("Installed desktop menu entry path")
installedMenuEntry := installedAssetsDir.value / "entry.desktop"


lazy val installJar = taskKey[Unit]("Installs the jar file")
installJar := {
  val log = streams.value.log

  val srcJarFile = assembly.value
  val destJarFile = installedJar.value

  log.copying(destJarFile)
  IO.copyFile(srcJarFile, destJarFile)
}


lazy val installLaunchingScripts = taskKey[Unit]("Installs launching scripts")
installLaunchingScripts := {
  val log = streams.value.log

  val targetJarFile = installedJar.value

  def installLaunchingScript(scriptFile: File, mainClass: String): Unit = {
    val targetJarPath = IO.relativize(scriptFile.getParentFile, targetJarFile)
      .map { "$(dirname $0)/" + _ }
      .getOrElse(targetJarFile.getPath)

    val adjustedTargetJarPath = OS.which match {
      case Linux => targetJarPath
      case Windows => "$(cygpath -w " + targetJarPath.replace('\\', '/') + ")"
    }

    log.writing(scriptFile)
    IO.write(scriptFile,
      s"""
         |#!/bin/bash
         |exec java -cp $adjustedTargetJarPath myreboot.main.$mainClass "$$@"
       """.stripMargin.trim + "\n"
    )
    if (OS.which == Linux) {
      IO.setPermissions(scriptFile, "rwxr-xr-x")
    }
  }

  installLaunchingScript(installedMainLaunchingScript.value, "Dialog")
  if (OS.which == Windows) {
    installLaunchingScript(installedSwitchDisplayScript.value, "SwitchDisplay")
  }
}


lazy val installMenuEntry = taskKey[Unit]("Installs desktop menu entry")
installMenuEntry := OS.linuxOnly {
  Def.task {
    val log = streams.value.log

    val iconFile = installedIcon.value
    log.copying(iconFile)
    IO.copyFile(baseDirectory.value / "icon.png", iconFile)

    val menuEntryFile = installedMenuEntry.value
    log.writing(menuEntryFile)
    IO.write(menuEntryFile,
        s"""
           |[Desktop Entry]
           |Version=${version.value}
           |Type=Application
           |Name=${name.value}
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
  }
}.value


lazy val install = taskKey[Unit]("Installs")
install := OS.select {
  case Linux => Def.sequential(
    installJar,
    installLaunchingScripts,
    installMenuEntry,
  )
  case Windows => Def.sequential(
    installJar,
    installLaunchingScripts,
  )
}.value
