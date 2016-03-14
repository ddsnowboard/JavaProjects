#lang racket
(define nextFib n l 
  (if (= n 0) l 
    ((cons (+ (list-ref (- (length l) 1)) (list-ref (- (length l) 1))) l))))
(display (for/sum ([i (lambda (n) 
                        (let ([a 1] [b 2])
