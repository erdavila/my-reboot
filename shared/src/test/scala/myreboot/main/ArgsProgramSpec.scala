package myreboot.main

import myreboot._
import myreboot.main.ArgsProgram._
import org.scalatest.{FunSpec, Matchers}

class ArgsProgramSpec extends FunSpec with Matchers {
  describe("ArgsProgram") {
    describe(".map") {
      it("transforms the value when the result is Success") {
        val program = ArgsProgram.unit(7).map { i => i.toString }

        val result = program.run(Vector("a", "b"))

        result should be (success("7", Vector("a", "b")))
      }

      it("keeps any Failure") {
        val program = new ArgsProgram[Int]({ _ => unknownArg("x") }).map { i => i.toString }

        val result = program.run(Vector("a", "b"))

        result should be (unknownArg("x"))
      }
    }

    describe(".flatMap") {
      it("composes") {
        val program = ArgsProgram.unit(7).flatMap { i => ArgsProgram.unit(i.toString) }

        val result = program.run(Vector("a", "b"))

        result should be (success("7", Vector("a", "b")))
      }

      it("keeps any Failure") {
        val program = new ArgsProgram[Int]({ _ => unknownArg("x") }).flatMap { i => ArgsProgram.unit(i.toString) }

        val result = program.run(Vector("a", "b"))

        result should be (unknownArg("x"))
      }
    }
  }

  describe("ArgsProgram.argOfType") {
    it("consumes arg that identifies a value of a type") {
      {
        val result = argOfType[Display].run(Vector("monitor", "linux"))
        result should be (success(Some(Monitor), Vector("linux")))
      }

      {
        val result = argOfType[Display].run(Vector("linux", "monitor"))
        result should be (success(Some(Monitor), Vector("linux")))
      }
    }

    it("ignores repetitions") {
      val result = argOfType[Display].run(Vector("monitor", "linux", "monitor", "abc"))
      result should be (success(Some(Monitor), Vector("linux", "abc")))
    }

    it("succeeds with None when there is no value of the type") {
      val result = argOfType[Display].run(Vector("linux", "windows"))

      result should be (success(None, Vector("linux", "windows")))
    }

    it("fails when there are different values of the type") {
      val result = argOfType[Display].run(Vector("monitor", "windows", "tv"))

      result should be (conflictingArgs("monitor", "tv"))
    }
  }

  describe("ArgsProgram.hasArg") {
    it("consumes specified arg providing true value") {
      val result = hasArg("x").run(Vector("a", "x", "b"))

      result should be (success(true, Vector("a", "b")))
    }

    it("ignores repetitions") {
      val result = hasArg("x").run(Vector("a", "x", "b", "x", "c"))

      result should be (success(true, Vector("a", "b", "c")))
    }

    it("provides false when the specified arg is not present") {
      val result = hasArg("x").run(Vector("a", "b"))

      result should be (success(false, Vector("a", "b")))
    }
  }

  describe("ArgsProgram.noMoreArgs") {
    it("succeeds if there are no args") {
      val result = noMoreArgs.run(Vector.empty)

      result should be (success((), Vector.empty))
    }

    it("fails if there are any args") {
      val result = noMoreArgs.run(Vector("a", "b"))

      result should be (unknownArg("a"))
    }
  }

  describe("composition") {
    val program = for {
      os <- argOfType[OS]
      display <- argOfType[Display]
      _ <- noMoreArgs
    } yield {
      (os, display)
    }

    it("works") {
      import org.scalatest.prop.TableDrivenPropertyChecks._

      val cases = Table(
        ("args"                                      , "expected result"),
        (Vector.empty                                , success((None       , None    ), Vector.empty)),
        (Vector("linux")                             , success((Some(Linux), None    ), Vector.empty)),
        (Vector("linux", "linux")                    , success((Some(Linux), None    ), Vector.empty)),
        (Vector("tv")                                , success((None       , Some(TV)), Vector.empty)),
        (Vector("linux", "tv")                       , success((Some(Linux), Some(TV)), Vector.empty)),
        (Vector("linux", "windows")                  , conflictingArgs("linux", "windows")),
        (Vector("tv", "linux", "tv", "windows", "tv"), conflictingArgs("linux", "windows")),
        (Vector("linux", "abc", "tv")                , unknownArg("abc")),
      )

      forAll (cases) { (args, result) =>
        program.run(args) should equal(result)
      }
    }
  }
}
