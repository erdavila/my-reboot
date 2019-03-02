import Logging._
import scala.sys.process._


name := "My Reboot"
version := "0.1"

scalaVersion := "2.12.8"

libraryDependencies += "org.scalafx" %% "scalafx" % "8.0.181-R13"
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

lazy val installedQueryDisplayExe = settingKey[File]("query-display.exe tool")
installedQueryDisplayExe := installDir.value / "query-display.exe"

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


lazy val installLaunchingScript = taskKey[Unit]("Installs launching script")
installLaunchingScript := {
  val log = streams.value.log

  val dialogShFile = installedMainLaunchingScript.value
  val targetJarFile = installedJar.value

  val targetJarPath = IO.relativize(dialogShFile.getParentFile, targetJarFile)
    .map { "$(dirname $0)/" + _ }
    .getOrElse(targetJarFile.getPath)

  log.writing(dialogShFile)
  IO.write(dialogShFile,
    s"""
       |#!/bin/bash
       |exec java -cp $targetJarPath myreboot.main.Dialog
     """.stripMargin.trim + "\n"
  )
  IO.setPermissions(dialogShFile, "rwxr-xr-x")
}


lazy val installMenuEntry = taskKey[Unit]("Installs desktop menu entry")
installMenuEntry := {
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


lazy val buildQueryDisplayExe = taskKey[File]("Builds query-display.exe (Windows only)")
buildQueryDisplayExe := OS.windowsOnly {
  Def.task {
    val s = streams.value
    val outputDir = target.value / "native"
    IO.createDirectory(outputDir)

    val build = FileFunction.cached(s.cacheDirectory) { inFiles =>
      for (inFile <- inFiles)
      yield {
        val (baseName, "cpp") = IO.split(inFile.getName)
        val outFile = outputDir / (baseName + ".exe")
        s.log.info(s"Building ${outFile.getPath}")
        Seq("c++", "-Wall", "-mwindows", inFile.getPath, "-o", outFile.getPath).!!
        outFile
      }
    }

    val inputFiles = Set(baseDirectory.value / "src" / "main" / "cpp" / "query-display.cpp")
    val outputFiles = build(inputFiles)

    val Seq(outputFile) = outputFiles.toSeq
    outputFile
  }
}.value


lazy val installQueryDisplayExe = taskKey[Unit]("Installs query-display.exe tool (Windows only)")
installQueryDisplayExe := OS.windowsOnly {
  Def.task {
    val log = streams.value.log

    val srcExeFile = buildQueryDisplayExe.value
    val destExeFile = installedQueryDisplayExe.value

    log.copying(destExeFile)
    IO.copyFile(srcExeFile, destExeFile)
  }
}.value


lazy val install = taskKey[Unit]("Installs")
install := Def.sequential(
  installJar,
  installLaunchingScript,
  installMenuEntry,
).value
