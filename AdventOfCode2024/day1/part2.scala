import scala.io.StdIn.readLine

@main
def run(): Unit =
  val (left, right) = Iterator
    .continually(readLine)
    .takeWhile((s) => (s != null && s.trim.nonEmpty))
    .map(_.split("   "))
    .foldLeft((List[Integer](), List[Integer]()))((accs, pair) =>
      (pair(0).toInt :: accs(0), pair(1).toInt :: accs(1))
    )
  val c = Counter(right.iterator)
  val similarity = left
    .map(i => i * c.get(i))
    .sum
  println(similarity)

class Counter[T](var it: Iterator[T]):
  private val counts = collection.mutable.Map[T, Int]()
  for el <- it do
    val currentCount = counts.getOrElse(el, 0)
    counts.put(el, currentCount + 1)

  def get(item: T): Int = counts.getOrElse(item, 0)
