package myreboot.main

import java.io.File
import myreboot.Configs
import myreboot.{LinuxPlatform, OS}
import scala.io.Source

object Setup extends SetupBase {

  def main(args: Array[String]): Unit = {
    val grubCfgFile = new File("/boot/grub/grub.cfg")
    println(s"Lendo ${grubCfgFile.getPath}...")
    val menuentryLines = Source.fromFile(grubCfgFile).getLines().filter(_ startsWith "menuentry ").toList

    val entries = for (os <- OS.Values)
      yield {
        println("Detectando entrada com \"" + os.code + "\"...")
        val entryIndex = menuentryLines.indexWhere(_ contains os.code)
        assert(entryIndex >= 0, "Entrada não encontrada!")
        val entryNumber = 1 + entryIndex
        val key = os.code + "." + Configs.GrubEntrySubKey
        key -> entryNumber.toString
      }
    saveConfigs(entries, LinuxPlatform.StateDir)
  }
}
