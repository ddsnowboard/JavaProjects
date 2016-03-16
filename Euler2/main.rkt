#lang racket
(define (smallFibs l)
  (define (nextFib li)
    (+ (last li) (list-ref li (- (length li) 2))))
  (define next (nextFib l))
  (if (<= next 4000000)
    (set! l (smallFibs (append l next)))
    (l)))
(display (for/sum ([i (smallFibs '(1 2))]) i))
