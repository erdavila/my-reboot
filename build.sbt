import Logging._
import java.io.File
import scala.sys.process._


ThisBuild / name := "My Reboot"
ThisBuild / version := "0.1"

ThisBuild / scalaVersion := "2.12.8"


// Settings
lazy val jarName = settingKey[String]("Jar name")
lazy val userHome = settingKey[File]("User home directory")
lazy val installedAssetsDir = settingKey[File]("Directory where non-executables will be written")
lazy val installedJar = settingKey[File]("Installed Jar path")
lazy val installedIcon = settingKey[File]("Installed icon path")
lazy val installedMenuEntry = settingKey[File]("Installed desktop menu entry path")
lazy val installedScriptsDir = settingKey[File]("Directory where scripts will be written")
lazy val installedMainLaunchingScript = settingKey[File]("Main launching script")
lazy val installedRebootScript = settingKey[File]("Script to reboot")
lazy val installedOptionsScript = settingKey[File]("Script to show, set and unset options")
lazy val installedSwitchDisplayScript = settingKey[File]("Script to switch displays")
lazy val installedLaunchingScripts = settingKey[Seq[(File, String)]]("Scripts and their main class names")

// Tasks
lazy val installJar = taskKey[Unit]("Installs the jar file")
lazy val installLaunchingScripts = taskKey[Unit]("Installs launching scripts")
lazy val installLaunchingIcons = taskKey[Unit]("Installs launching icons")
lazy val setSwitchDisplayToRunOnStartup = taskKey[Unit]("Register SwitchDisplay to run on Windows startup")
lazy val runSetup = taskKey[Unit]("Runs setup")
lazy val install = taskKey[Unit]("Installs")

val DialogClassName = "myreboot.main.Dialog"
val RebootClassName = "myreboot.main.Reboot"
val OptionsClassName = "myreboot.main.Options"
val SwitchDisplayClassName = "myreboot.main.SwitchDisplay"
lazy val JavawPosixPath = findPath("javaw.exe")


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

  userHome := file(scala.sys.env.getOrElse("HOME", System.getProperty("user.home"))),
  installedAssetsDir := userHome.value / "my-reboot",
  installedJar := installedAssetsDir.value / jarName.value,
  installedScriptsDir := userHome.value / "bin",
  installedMainLaunchingScript := installedScriptsDir.value / "my-reboot-dialog",
  installedRebootScript := installedScriptsDir.value / "my-reboot",
  installedOptionsScript := installedScriptsDir.value / "my-reboot-options",
  installedLaunchingScripts := Seq(
    installedMainLaunchingScript.value -> DialogClassName,
    installedRebootScript.value -> RebootClassName,
    installedOptionsScript.value -> OptionsClassName,
  ),

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
           |exec java -cp $escapedTargetJarPath $mainClass "$$@"
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

    installLaunchingIcons := {
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

      log.info("Criando entrada no menu")
      Seq("xdg-desktop-menu", "install", "--novendor", "--mode", "user", menuEntryFile.getPath).!!
      Seq("xdg-desktop-menu", "forceupdate", "--mode", "user").!!
    },
    install := Def.sequential(
      installJar,
      installLaunchingScripts,
      installLaunchingIcons,
      runSetup,
    ).value,
  )


lazy val windows = (project in file("windows"))
  .dependsOn(shared)
  .settings(commonSettings: _*)
  .settings(
    libraryDependencies += "net.java.dev.jna" % "jna" % "5.2.0",

    installedIcon := installedAssetsDir.value / "icon.ico",
    installedSwitchDisplayScript := installedScriptsDir.value / "my-reboot-switch-display",
    installedLaunchingScripts += (installedSwitchDisplayScript.value -> SwitchDisplayClassName),

    installLaunchingIcons := {
      val log = streams.value.log

      def installShortcut(
        className: String,
        name: String,
        description: String,
        iconPath: String,
        iconOffset: Option[Int] = None,
      ): Unit = {
        val places = Seq(
          ("--desktop", "na Área de Trabalho"),
          ("--smprograms", "no Menu Iniciar"),
        )
        for ((placeArg, placeName) <- places) {
          log.info(s"Criando atalho para $name $placeName")
          val cmd = Seq(
            "mkshortcut",
            placeArg,
            "--name", name,
            "--desc", description,
            "--icon", iconPath
          ) ++ iconOffset.map(offs => Seq("--iconoffset", offs.toString)).getOrElse(Seq.empty) ++ Seq(
            "--arguments", s"-cp ${installedJar.value.getPath} $className",
            JavawPosixPath,
          )
          cmd.!!
        }
      }

      val iconFile = installedIcon.value
      log.copying(iconFile)
      IO.copyFile((ThisBuild / baseDirectory).value / "icon.ico", iconFile)
      val iconPosixPath = Seq("cygpath", "-u", iconFile.getPath).!!
      installShortcut(
        className = DialogClassName,
        name = "My Reboot",
        description = "Opções de reinicialização",
        iconPath = iconPosixPath,
      )

      installShortcut(
        className = SwitchDisplayClassName,
        name = "Alternar Tela",
        description = "Alternar tela",
        iconPath = findPath("DisplaySwitch.exe"),
        iconOffset = Some(2),
      )
    },
    setSwitchDisplayToRunOnStartup := {
      streams.value.log.info("Definindo DisplaySwitch para executar no início do Windows")
      val javawPath = Seq("cygpath", "-w", JavawPosixPath).!!.stripLineEnd
      def quoted(str: String): String = "\"" + str + "\""

      val key = raw"""\HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run\MyRebootSwitchDisplay"""
      val value = s"${quoted(javawPath)} -cp ${quoted(installedJar.value.getPath)} $SwitchDisplayClassName saved"
      s"regtool set '$key' '$value'".!!
    },
    install := Def.sequential(
      installJar,
      installLaunchingScripts,
      installLaunchingIcons,
      setSwitchDisplayToRunOnStartup,
      runSetup,
    ).value,
  )


lazy val myReboot = (project in file("."))
  .aggregate(shared, linux, windows)
  .settings(
    installJar / aggregate := false,
    installLaunchingScripts / aggregate := false,
    installLaunchingIcons / aggregate := false,
    setSwitchDisplayToRunOnStartup / aggregate := false,
    runSetup  / aggregate := false,
    install / aggregate := false,

    install := Def.taskDyn {
      OS.which match {
        case Linux => Def.task { (linux / install).value }
        case Windows => Def.task { (windows / install).value }
      }
    }.value,
  )


def findPath(program: String): String =
  Seq("which", program).!!.stripLineEnd
