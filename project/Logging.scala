import sbt.File
import sbt.internal.util.ManagedLogger

object Logging {

  implicit class LogOps(val log: ManagedLogger) extends AnyVal {
    def copying(file: File): Unit =
      log.info(s"Copying ${file.getName} to ${file.getParent}")

    def writing(file: File): Unit =
      log.info(s"Writing ${file.getName} in ${file.getParent}")
  }

}
