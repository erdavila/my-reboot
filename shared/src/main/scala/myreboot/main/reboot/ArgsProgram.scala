package myreboot.main.reboot

import myreboot.WithCode
import myreboot.main.reboot.ArgsProgram.{Args, Result}

class ArgsProgram[A](val run: Args => Result[A]) {
  final def map[B](f: A => B): ArgsProgram[B] =
    flatMap { a => ArgsProgram.unit(f(a)) }

  final def flatMap[B](f: A => ArgsProgram[B]): ArgsProgram[B] =
    new ArgsProgram[B]({ args =>
      run(args).flatMap { extractedArg =>
        f(extractedArg.value).run(extractedArg.remaining)
      }
    })
}

object ArgsProgram {
  type Args = Vector[String]
  case class ExtractedArg[A](value: A, remaining: Args)
  type Result[A] = Either[String, ExtractedArg[A]]

  def success[A](value: A, remaining: Args) = Right(ExtractedArg(value, remaining))
  def conflictingArgs(arg1: String, arg2: String) = Left(s"$arg1 ou $arg2?!")
  def unknownArg(arg: String) = Left(s"O que é $arg?!")

  def unit[A](a: A): ArgsProgram[A] =
    new ArgsProgram[A]({ args => success(a, args) })

  def argOfType[A <: WithCode](implicit companion: WithCode.Companion[A]): ArgsProgram[Option[A]] =
    new ArgsProgram[Option[A]]({ args =>
      args.foldLeft(success(None, Vector.empty): Result[Option[A]]) { (result, arg) =>
        result.flatMap { extractedArg =>
          val maybeA1 = extractedArg.value
          val maybeA2 = companion.byCode(arg)
          (maybeA1, maybeA2) match {
            case (Some(a1), Some(a2)) if a1 != a2 =>
              conflictingArgs(a1.code, a2.code)

            case _ =>
              val newUnmatchedArgs = if (maybeA2.isDefined) {
                extractedArg.remaining
              } else {
                extractedArg.remaining :+ arg
              }
              success(maybeA1 orElse maybeA2, newUnmatchedArgs)
          }
        }
      }
    })

  def noMoreArgs: ArgsProgram[Unit] =
    new ArgsProgram[Unit]({ args =>
      args.headOption match {
        case Some(arg) => unknownArg(arg)
        case None => success((), Vector.empty)
      }
    })
}
