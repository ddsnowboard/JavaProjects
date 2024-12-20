#lang racket
(define (smallFibs l)
  (define (nextFib li)
    (+ (last li) (list-ref li (- (length li) 2))))
  (define next (nextFib l))
  (if 
    ;(<= next 2140000000)
    (<= next 45)
    (smallFibs (append l (list next)))
    l))
(display (for/sum ([i (smallFibs '(1 2))] #:when (= (modulo i 2) 0)) i))
(display "\n")
