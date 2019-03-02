package myreboot

import java.io.{File, FileReader, FileWriter}
import java.util.Properties

object PropertiesFile {
  def load(file: File): PropertiesFile = {
    val props = new Properties
    if (file.exists()) {
      val reader = new FileReader(file)
      try {
        props.load(reader)
      } finally {
        reader.close()
      }
    }
    new PropertiesFile(props, file)
  }

  private def save(props: Properties, file: File): Unit = {
    val writer = new FileWriter(file)
    try {
      props.store(writer, null)
    } finally {
      writer.close()
    }
  }
}

class PropertiesFile(props: Properties, file: File) {
  def get(key: String): Option[String] = {
    val valueOrNull = props.getProperty(key)
    Option(valueOrNull)
  }

  def set(key: String, value: String): Unit =
    props.setProperty(key, value)

  def save(): Unit =
    PropertiesFile.save(props, file)
}
