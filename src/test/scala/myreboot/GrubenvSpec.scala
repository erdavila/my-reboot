package myreboot

import org.scalatest.{FunSpec, Matchers}

class GrubenvSpec extends FunSpec with Matchers {
  describe("Grubenv") {
    describe("fromLines()") {
      it("creates a new instance from the lines") {
        val lines = Iterator(
          "# First line",
          "a=b",
          "x=123",
        )

        val grubenv = Grubenv.fromLines(lines)

        val expectedLines = Vector(
          "# First line",
          "a=b",
          "x=123",
        )
        grubenv.lines should be (expectedLines)
      }

      it("ignores the last line if it starts with '#'") {
        val lines = Iterator(
          "# First line",
          "a=b",
          "####",
        )

        val grubenv = Grubenv.fromLines(lines)

        val expectedLines = Vector(
          "# First line",
          "a=b",
        )
        grubenv.lines should be (expectedLines)
      }

      it("ignores the last line if it is empty") {
        val lines = Iterator(
          "# First line",
          "a=b",
        )

        val grubenv = Grubenv.fromLines(lines)

        val expectedLines = Vector(
          "# First line",
          "a=b",
        )
        grubenv.lines should be (expectedLines)
      }
    }

    describe("set()") {
      it("replaces a line when the key is found") {
        val lines = Vector(
          "# First line",
          "a=b",
          "x=123",
        )
        val grubenv = new Grubenv(lines)

        grubenv.set("a", "zzz")

        val expectedLines = Vector(
          "# First line",
          "a=zzz",
          "x=123",
        )
        grubenv.lines should be (expectedLines)
      }

      it("adds a line when the key is not found") {
        val lines = Vector(
          "# First line",
          "a=b",
          "x=123",
        )
        val grubenv = new Grubenv(lines)

        grubenv.set("new", "zzz")

        val expectedLines = Vector(
          "# First line",
          "a=b",
          "x=123",
          "new=zzz",
        )
        grubenv.lines should be (expectedLines)
      }
    }

    describe("unset()") {
      it("removes a line") {
        val lines = Vector(
          "# First line",
          "a=b",
          "x=123",
        )
        val grubenv = new Grubenv(lines)

        grubenv.unset("a")

        val expectedLines = Vector(
          "# First line",
          "x=123",
        )
        grubenv.lines should be (expectedLines)
      }
    }

    describe("toFileContent") {
      val lines = Vector(
        "# First line",
        "a=b",
        "x=123",
      )
      val grubenv = new Grubenv(lines)

      val string = grubenv.toFileContent

      it("should return a string of size 1024") {
        string should have length 1024
      }

      it("should include all the content") {
        string should startWith ("# First line\na=b\nx=123\n####")
      }
    }
  }
}
