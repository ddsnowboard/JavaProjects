#lang racket
(define nums (string->list "1234567890"))
(define (func x)
  (and 
    (= (modulo (string->number (substring (list->string x) 1 4)) 2) 0)
    (= (modulo (string->number (substring (list->string x) 2 5)) 3) 0)
    (= (modulo (string->number (substring (list->string x) 3 6)) 5) 0)
    (= (modulo (string->number (substring (list->string x) 4 7)) 7) 0)
    (= (modulo (string->number (substring (list->string x) 5 8)) 11) 0)
    (= (modulo (string->number (substring (list->string x) 6 9)) 13) 0)
    (= (modulo (string->number (substring (list->string x) 7 10)) 17) 0)))
(define goodStrings (stream-filter func (sequence->stream (in-permutations nums))))
(display (for/sum ([x goodStrings]) (string->number (list->string x))))
