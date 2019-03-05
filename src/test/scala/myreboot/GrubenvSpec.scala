package myreboot

import java.io.File
import org.scalatest.{FunSpec, Matchers}

class GrubenvSpec extends FunSpec with Matchers {
  describe("Grubenv") {
    describe("cleanLines()") {
      it("discards lines containing only '#' or empty lines") {
        val lines = Iterator(
          "# First line",
          "a=b",
          "",
          "x=123",
          "#########",
        )

        val result = Grubenv.cleanLines(lines)

        val expectedLines = Vector(
          "# First line",
          "a=b",
          "x=123",
        )
        result.toVector should be (expectedLines)
      }
    }

    describe("toFileContent()") {
      val lines = Vector(
        "# First line",
        "a=b",
        "x=123",
      )

      val result = Grubenv.toFileContent(lines)

      it("should return a result of size 1024") {
        result should have length 1024
      }

      it("should include all the content") {
        result should startWith ("# First line\na=b\nx=123\n####")
      }
    }

    def grubenv = new Grubenv(
      Vector(
        "# First line",
        "a=b",
        "x=123",
      ),
      new File("")
    )

    describe("set()") {
      it("replaces a line when the key is found") {
        val instance = grubenv

        instance.set("a", "zzz")

        val expectedLines = Vector(
          "# First line",
          "a=zzz",
          "x=123",
        )
        instance.lines should be (expectedLines)
      }

      it("adds a line when the key is not found") {
        val instance = grubenv

        instance.set("new", "zzz")

        val expectedLines = Vector(
          "# First line",
          "a=b",
          "x=123",
          "new=zzz",
        )
        instance.lines should be (expectedLines)
      }
    }

    describe("unset()") {
      it("removes a line") {
        val instance = grubenv

        instance.unset("a")

        val expectedLines = Vector(
          "# First line",
          "x=123",
        )
        instance.lines should be (expectedLines)
      }
    }

  }
}
