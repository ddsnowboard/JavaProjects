import scala.io.StdIn.readLine

def isSafe(levels:Iterator[Int]):Boolean = 
  val windows = levels.sliding(2).map(l => (l(0), l(1))).toList
// This isn't right; I can't just skip 1 problem. I need to try removing all the
// reports and see if any of those outputs is OK.
  val gradualEnough = windows.count(block => 
      val delta = Math.abs(block(0) - block(1))
      !(delta >= 1 && delta <= 3)
      ) <= 1
  gradualEnough && (windows.forall(_ > _) || windows.forall(_ < _))


@main
def run(): Unit =
  val safeLines = Iterator
    .continually(readLine)
    .takeWhile((s) => (s != null && s.trim.nonEmpty))
    .map(_.split(" ").map(_.toInt).iterator)
    .count(isSafe)

  println(safeLines)
