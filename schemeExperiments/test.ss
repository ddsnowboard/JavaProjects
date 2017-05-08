(define (sum-squares x y) (+ (* x x) (* y y)))

(define (square-bigger x y z) (cond 
                                ((and (< x y) (< x z)) (sum-squares z y))
                                ((and (< y z) (< y x)) (sum-squares x z))
                                ((and (< z x) (< z y)) (sum-squares x y))))


(define (cube-root x)
  (define DELTA .01)
  (define (abs-difference x y) (abs (- x y)))

  (define (next-number y) (/ (+ (/ x (* y y)) (* y 2)) 3))

  (define (good-enough y) (< (abs-difference (* y y y) x) DELTA))

  (define (cube-iter y) (if (good-enough y)
                          y
                          (cube-iter (next-number y))))

  (cube-iter 1))

(define (square x) (* x x))

(define (fast-exp b n) 
  (define (even? x) (= 0 (remainder x 2)))

  (define (exp-iter a b n) 
    (cond 
      ((= n 1) (* b a))
      ((even? n) (* (exp-iter 1 b (/ n 2)) (exp-iter a b (/ n 2))))
      (else (exp-iter (* a b) b (- n 1)))
      ))
  (exp-iter 1 b n))
