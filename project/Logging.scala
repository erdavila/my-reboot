import sbt.File
import sbt.internal.util.ManagedLogger

object Logging {

  implicit class LogOps(val log: ManagedLogger) extends AnyVal {
    def copying(file: File): Unit =
      log.info(s"Copiando ${file.getName} para ${file.getParent}")

    def writing(file: File): Unit =
      log.info(s"Escrevendo ${file.getName} em ${file.getParent}")
  }

}
