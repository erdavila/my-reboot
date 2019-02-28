package myreboot

import java.io.FileWriter
import scala.io.Source

object Grubenv {
  def load(path: String): Grubenv =
    fromLines(Source.fromFile(path).getLines())

  private[myreboot] def fromLines(linesIter: Iterator[String]): Grubenv = {
    val loadedLines = linesIter.toVector
    val lines = if (loadedLines.last.startsWith("#") || loadedLines.last == "") {
      loadedLines.dropRight(1)
    } else {
      loadedLines
    }
    new Grubenv(lines)
  }
}

class Grubenv private[myreboot](private[myreboot] var lines: Vector[String]) {

  def set(key: String, value: String): Unit = {
    val keyValueLine = s"$key=$value"

    var found = false
    val updatedLines = lines map { line =>
      if (line startsWith s"$key=") {
        found = true
        keyValueLine
      } else {
        line
      }
    }

    lines = if (found) {
      updatedLines
    } else {
      updatedLines :+ keyValueLine
    }
  }

  def unset(key: String): Unit =
    lines = lines.filterNot(_ startsWith s"$key=")

  def save(path: String): Unit = {
    val f = new FileWriter(path)
    try {
      f.write(this.toFileContent)
    } finally {
     f.close()
    }
  }

  private[myreboot] def toFileContent: String = {
    val Length = 1024
    lines.map(_ + "\n")
      .mkString
      .take(Length)
      .padTo(Length, '#')
  }
}
