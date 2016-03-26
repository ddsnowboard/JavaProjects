; Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
; 
; For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
; 
; What is the total of all the name scores in the file?
#lang racket
(require profile)
(require future-visualizer)
(require racket/future)
(define (index needle haystack)
  (define (helper needle haystack n)
    (if (>= n (string-length haystack))
      -1
      (if (eq? (string-ref haystack n) needle)
        n
        (helper needle haystack (+ 1 n)))))
  (helper needle haystack 0))
(define (score letter)
  (define scores (hash "A" 1 
                       "B" 2 
                       "C" 3 
                       "D" 4 
                       "E" 5 
                       "F" 6 
                       "G" 7 
                       "H" 8 
                       "I" 9 
                       "J" 10 
                       "K" 11 
                       "L" 12 
                       "M" 13 
                       "N" 14 
                       "O" 15 
                       "P" 16 
                       "Q" 17 
                       "R" 18 
                       "S" 19 
                       "T" 20 
                       "U" 21 
                       "V" 22 
                       "W" 23 
                       "X" 24 
                       "Y" 25 
                       "Z" 26))
  (hash-ref scores (string letter)))

(define (scoreWord str)
  (for/sum ([j (string->list str)]) (score j)))

(define (multiplyByPlace l)
  (define (helper l n)
    (if (null? (cdr l))
      (cons (* (car l) n) '())
      (cons (* (car l) n) (helper (cdr l) (+ 1 n)))))
  (helper l 1))
(define (runWithFuture lNames)
  (define (helper lNames)
    (if (null? (cdr lNames))
      (cons (future (lambda () (scoreWord (car lNames)))) '())
      (cons (future (lambda () (scoreWord (car lNames)))) (helper (cdr lNames)))))
  (helper lNames))

(define input (open-input-file "input.txt"))
(define sNames (read-line input))
(close-input-port input)
(define (mainFunc)
  (define lNames (sort 
                   (string-split 
                     (list->string 
                       (remove* '(#\") 
                                (string->list sNames))) ",") string<?))
  (define scores (multiplyByPlace 
                   (for/list 
                     ([i (runWithFuture lNames)]) 
                     (touch i))))
  (display (for/sum ([i scores]) i)))
(visualize-futures-thunk mainFunc)
