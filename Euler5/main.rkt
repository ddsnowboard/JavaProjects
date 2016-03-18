#lang racket
(define (divisible? n)
  ; All the prime numbers between 1 and 20 will take care of all the other numbers
  (if (not (for/and ([i (in-range 1 21)]) (= (remainder n i) 0)))
    (divisible? (+ n 1))
    n))
(display (divisible? 11))
