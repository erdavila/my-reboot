package myreboot.main

import java.io.File
import myreboot.{Configs, PropertiesFile}
import scala.io.StdIn

trait SetupBase {

  protected def saveConfigs(entries: Seq[(String, String)], stateDir: File): Unit = {
    val configsFile = new File(stateDir, Configs.FileName)
    val propsFile = PropertiesFile.load(configsFile)
    println(s"As seguintes propriedades serão salvas em ${configsFile.getPath}:")
    for ((key, value) <- entries) {
      println(s"  $key=$value")
      propsFile.set(key, value)
    }
    askToProceed()
    propsFile.save()
  }

  protected def askToProceed(): Unit = {
    StdIn.readLine("Pressione ENTER para prosseguir ")
    println()
  }
}
