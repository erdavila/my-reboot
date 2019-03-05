package myreboot.main

import java.io.File
import myreboot._
import scala.io.{Source, StdIn}

object Setup {

  def main(args: Array[String]): Unit =
    OS.which match {
      case Linux => linuxSetup()
      case Windows => ???
    }

  private def linuxSetup(): Unit = {
    val grubCfgFile = new File("/boot/grub/grub.cfg")
    println(s"Lendo ${grubCfgFile.getPath}...")
    val menuentryLines = Source.fromFile(grubCfgFile).getLines().filter(_ startsWith "menuentry ").toList

    val entries =
      for (os <- OS.Values)
      yield {
        println("Detectando entrada com \"" + os.code + "\"...")
        val entryIndex = menuentryLines.indexWhere(_ contains os.code)
        assert(entryIndex >= 0, "Entrada não encontrada!")
        val entryNumber = 1 + entryIndex
        val key = os.code + "." + Configs.GrubEntrySubKey
        key -> entryNumber.toString
      }

    val configsFile = new File(LinuxPlatform.StateDir, Configs.FileName)
    val propsFile = PropertiesFile.load(configsFile)
    println(s"As seguintes propriedades serão salvas em ${configsFile.getPath}:")
    for ((key, value) <- entries) {
      println(s"  $key=$value")
      propsFile.set(key, value)
    }
    StdIn.readLine("Pressione ENTER para prosseguir ")
    propsFile.save()

    println("PRONTO")
  }
}
