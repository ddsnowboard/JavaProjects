#lang racket
(define (nextFib n l)
  (if (= n 0) l 
    (nextFib (- n 1) (append* l (+ (list-ref l (- (length l) 1)) (list-ref  l (- (length l) 2)))))))
(display (for/sum ([i (nextFib 20 '(1 2))]) i))

