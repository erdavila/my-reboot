package myreboot

import java.io.{File, FileWriter}
import scala.io.Source

object Grubenv {
  def load(file: File): Grubenv = {
    val lines = cleanLines(Source.fromFile(file).getLines())
    new Grubenv(lines.toVector, file)
  }

  private[myreboot] def cleanLines(lines: Iterator[String]): Iterator[String] =
    lines.filter(line => line.exists(_ != '#'))

  private def save(lines: Vector[String], file: File): Unit = {
    val f = new FileWriter(file)
    try {
      f.write(toFileContent(lines))
    } finally {
      f.close()
    }
  }

  private[myreboot] def toFileContent(lines: Vector[String]): String = {
    val Length = 1024
    lines.map(_ + "\n")
      .mkString
      .take(Length)
      .padTo(Length, '#')
  }
}

class Grubenv private[myreboot](private[myreboot] var lines: Vector[String], file: File) {

  def get(key: String): Option[String] =
    lines.collectFirst {
      case line if line startsWith s"$key=" =>
        line.dropWhile(_ != '=').drop(1)
    }

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

  def save(): Unit =
    Grubenv.save(lines, file)
}
