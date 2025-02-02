import scala.io.StdIn.readLine

def isSafe(levels:Iterator[Int]):Boolean = 
  val windows = levels.sliding(2).map(l => (l(0), l(1))).toList
  val gradual = windows.foldLeft(true)((acc:Boolean, block:(Int, Int)) => 
      val delta = Math.abs(block(0) - block(1))
      acc && delta >= 1 && delta <= 3
      )
  gradual && (windows.forall(_ > _) || windows.forall(_ < _))


@main
def run(): Unit =
  val safeLines = Iterator
    .continually(readLine)
    .takeWhile((s) => (s != null && s.trim.nonEmpty))
    .map(_.split(" ").map(_.toInt).iterator)
    .count(isSafe)

  println(safeLines)
