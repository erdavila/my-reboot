package myreboot.main

import java.io.File
import myreboot.Configs
import myreboot.{Platform, OS}
import scala.io.Source

object Setup extends SetupBase {

  private val MenuEntryRegex = """.*'([a-zA-Z0-9_\-]+)'\s*\{.*""".r

  def main(args: Array[String]): Unit = {
    val grubCfgFile = new File("/boot/grub/grub.cfg")
    println(s"Lendo ${grubCfgFile.getPath}...")
    val menuentryLines = Source.fromFile(grubCfgFile).getLines().filter { line => line startsWith "menuentry " }.toList

    val entries = for {
      os <- OS.Values

      _ = println("Detectando entrada com \"" + os.code + "\"...")
      line = menuentryLines.find(_ contains os.code) getOrElse { throw new Error("Entrada não encontrada!") }
      entryId = line match { case MenuEntryRegex(id) => id }

      key = os.code + "." + Configs.GrubEntrySubKey
    } yield {
      key -> entryId
    }

    saveConfigs(entries, Platform.StateDir)
  }
}
