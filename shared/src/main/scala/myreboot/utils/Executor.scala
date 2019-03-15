package myreboot.utils

import scala.sys.process._

object Executor {
  /**
    * Starts the process; blocks until it exits.
    * Standard output and error are sent to the console.
    * If the exit code is non-zero, an exception is thrown.
    */
  def execute(command: String, arguments: String*): Unit = {
    val exitCode = (command +: arguments).!
    if (exitCode != 0) {
      scala.sys.error("Nonzero exit value: " + exitCode)
    }
  }
}
