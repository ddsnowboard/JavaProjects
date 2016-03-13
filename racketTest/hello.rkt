#lang racket
(define (fizzbuzz n)
  (define (_fizzbuzz i)
    (if (= 0 (remainder i 15)) "fizzbuzz" 
      (if (= 0 (remainder i 5)) "buzz" 
        (if (= 0 (remainder i 3)) "buzz" i))))
  (if (= n 1) 1 
    (let ([i n])
      (string-append (_fizzbuzz i) (fizzbuzz (- i 1)))))
  (print (fizzbuzz 15))
