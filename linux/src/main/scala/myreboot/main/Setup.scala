package myreboot.main

import java.io.File
import myreboot.Configs
import myreboot.{Platform, OS}
import scala.io.Source

object Setup extends SetupBase {

  def main(args: Array[String]): Unit = {
    val grubCfgFile = new File("/boot/grub/grub.cfg")
    println(s"Lendo ${grubCfgFile.getPath}...")
    val menuentryLines = Source.fromFile(grubCfgFile).getLines().filter { line =>
      (line startsWith "menuentry ") || (line startsWith "submenu ")
    }.toList

    val entries = for (os <- OS.Values)
      yield {
        println("Detectando entrada com \"" + os.code + "\"...")
        val entryNumber = menuentryLines.indexWhere(_ contains os.code)
        assert(entryNumber >= 0, "Entrada não encontrada!")
        val key = os.code + "." + Configs.GrubEntrySubKey
        key -> entryNumber.toString
      }
    saveConfigs(entries, Platform.StateDir)
  }
}
