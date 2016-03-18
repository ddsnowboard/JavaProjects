#lang racket
(define (divisible? n)
  ; All the prime numbers between 1 and 20 will take care of all the other numbers
  (if (for/and ([i '(2 3 5 7 11 13 17 19)]) (remainder n i))
    (divisible? (+ n 1))
    n))
(display (divisible? 11))
