#lang racket
(define (divisible? n)
  (if (not (for/and ([i (in-range 1 21)]) (= (remainder n i) 0)))
    (divisible? (+ n 1))
    n))
(display (divisible? 11))
(display "\n")
