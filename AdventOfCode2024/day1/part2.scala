import scala.io.StdIn.readLine

@main
def run(): Unit =
  val (l, r) = Iterator
    .continually(readLine)
    .takeWhile((s) => (s != null && s.trim.nonEmpty))
    .map(_.split("   "))
    .foldLeft((List[Integer](), List[Integer]()))((accs, pair) =>
      (pair(0).toInt :: accs(0), pair(1).toInt :: accs(1))
    )
  val left = l.sorted
  val right = r.sorted
  val distances =
    left
      .zip(right)
      .map(((l: Integer, r: Integer) => Math.abs(r - l)).tupled)
  println(distances.sum)
