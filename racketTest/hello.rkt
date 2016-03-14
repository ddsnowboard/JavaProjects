#lang racket
(define (fizzbuzz n l)
  (define (otherFizz i)
    (if (= 0 (remainder i 15)) "fizzbuzz\n" 
        (if (= 0 (remainder i 5)) "buzz\n" 
            (if (= 0 (remainder i 3)) "fizz\n" (string-append (number->string i) "\n")))))
  (if (= n 0) l 
      (fizzbuzz (- n 1) (cons (let ([out (otherFizz n)])
                                 (if (number? out) (number->string out) out)) l))))
(display (fizzbuzz 1000 '()))
