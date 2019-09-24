;;; File:  main.scm
;; Author: ymiyamoto
;;
;; Created on Mon Sep 23 13:01:46 2019
;;
(use gauche.array)
(use util.combinations)

(define (read-number)
  (string->number (read-line)))

(define (read-numbers)
  (map string->number (string-split (read-line) #\space)))

(define (1+ x) (+ x 1))

(define (1- x) (- x 1))

(let* ((n (read-number))
       (s (string->vector (read-line))))
  (define arr (make-array (shape 0 n 0 n) 0))

  (for-each
   (lambda (i)
     (if (eqv? (eqv? (vector-ref s i) (vector-ref s (1- n))))
	 (array-set! arr (vector i (1- n)) 1)
	 (array-set! arr (vector (1- n) i) 1)))
   (iota n))

  (let loop ((i (- n 2)))
    (when (>= i 0)
      (let inner ((j (- n 2)))
	(when (>= j 0)
	  (array-set! arr (vector i j)
		      (if (eqv? (vector-ref s i) (vector-ref s j))
			  (1+ (array-ref arr (vector (1+ i) (1+ j))))
			  0))
	  (inner (1- j))))
      (loop (1- i))))

  (print
   (apply max (map (lambda (i j) (min (array-ref arr (vector i j)) (- j i))
		      (cartesian-product (iota (1- n)) (iota (1- n))))))))
